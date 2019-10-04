//#[macro_use]
//extern crate log;
//extern crate android_logger;

mod excel_to_json;
use excel_to_json::*;

use jni::objects::{JClass, JObject, JString};
use jni::sys::{jint, jobject};
use jni::{JNIEnv, JavaVM};
use serde_json::to_string;

//JNI加载完成
#[no_mangle]
pub extern "C" fn JNI_OnLoad(_jvm: JavaVM, _reserved: *mut std::ffi::c_void) -> jint {
    //android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Info));
    //info!("JNI_OnLoad.");
    jni::sys::JNI_VERSION_1_6
}

#[no_mangle]
pub extern "C" fn Java_io_github_planet0104_excel2json_ExcelToJson_convert(
    env: JNIEnv,
    _class: JClass,
    file: JString,
) -> jobject {
    let mje = |err| format!("{:?}", err);

    let result = (|| -> Result<JString, String> {
        let file: String = env.get_string(file).map_err(mje)?.into();
        match convert(&file) {
            Ok(v) => {
                let s = to_string(&v).unwrap_or(String::from("json转换失败"));
                let s = env.new_string(&s).map_err(mje)?;
                Ok(s)
            }
            Err(err) => Err(format!("{:?}", err)),
        }
    })();

    match result {
        Ok(s) => s.into_inner(),
        Err(err) => {
            let _ = env.throw_new("java/lang/Exception", format!("excel转换失败: {:?}", err));
            JObject::null().into_inner()
        }
    }
}
