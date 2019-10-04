package io.github.planet0104.excel2json;

public class ExcelToJson {
    static{
        System.loadLibrary("exceltojson");
    }
    public static native String convert(String file);
}
