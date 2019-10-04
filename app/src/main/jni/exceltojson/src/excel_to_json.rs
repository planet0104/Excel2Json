use calamine::{open_workbook, DataType, Error, Range, Reader, Xls, Xlsx};
use serde_json::{json, Value};
use std::collections::HashMap;

fn open_excel(file: &str) -> Result<HashMap<String, Range<DataType>>, Error> {
    let mut sheets = HashMap::new();
    if file.ends_with("xlsx") {
        let mut excel: Xlsx<_> = open_workbook(file)?;
        let sheet_names: Vec<String> = excel
            .sheet_names()
            .iter()
            .map(|s| String::from(s))
            .collect();

        for sheet_name in sheet_names {
            if let Some(r) = excel.worksheet_range(&sheet_name) {
                sheets.insert(sheet_name, r?);
            }
        }
    } else {
        let mut excel: Xls<_> = open_workbook(file)?;
        let sheet_names: Vec<String> = excel
            .sheet_names()
            .iter()
            .map(|s| String::from(s))
            .collect();

        for sheet_name in sheet_names {
            if let Some(r) = excel.worksheet_range(&sheet_name) {
                sheets.insert(sheet_name, r?);
            }
        }
    }
    Ok(sheets)
}

pub fn convert(file: &str) -> Result<Value, Error> {
    let sheets = open_excel(file)?;

    let mut val = HashMap::new();
    for (sheet_name, s) in sheets {
        let mut sheet = vec![];
        for row in s.rows() {
            let mut record = vec![];
            for col in row {
                let val = match &col {
                    &DataType::Int(i) => format!("{}", i),
                    &DataType::Bool(b) => format!("{}", b),
                    &DataType::Float(f) => format!("{}", f),
                    &DataType::String(s) => s.clone(),
                    _ => String::new(),
                };
                record.push(val);
            }
            sheet.push(record);
        }
        val.insert(sheet_name, sheet);
    }
    Ok(json!(val))
}
