use jni::sys::jstring;
use jni::objects::{JClass, JString};
use jni::JNIEnv;
use crate::jnitools::jstring_to_string;

pub enum ParseError {

}

pub type ParseResult = Result<Function, ParseError>;

pub struct Function;

pub fn parse(_src: &str) -> ParseResult {
    Ok(Function)
}

#[no_mangle]
pub extern fn Java_org_dedda_cellar_core_FunctionCore__compile(
    env: JNIEnv,
    _class: JClass,
    source: JString,
) -> jstring {
    let source = jstring_to_string(&env, source);
    let response = match parse(&source) {
        Ok(_function) => ";",
        Err(_err) => ";error!?",
    };
    env.new_string(response).unwrap().into_inner()
}