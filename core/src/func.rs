use jni::sys::{jstring, jobjectArray};
use jni::objects::{JClass, JString};
use jni::JNIEnv;
use crate::jnitools::{jstring_to_string, CLASS_NAME_STRING};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::grid::DataSource;
use crate::func::value::Value;

mod bifs;
pub mod util;
pub mod value;

trait Evaluate<Src> where Src: DataSource {
    fn eval(&self, data_source: &Src) -> EvalResult;
}

#[derive(Debug)]
pub enum ParseError {

}

pub type ParseResult = Result<Function, ParseError>;

pub type EvalResult = Result<Value, String>;

#[derive(Clone)]
pub struct Function {
    bif: bool,
    pub source: String,
}

impl Function {
    pub fn new_bif(source: String) -> Self {
        Self {
            bif: true,
            source,
        }
    }

    pub fn new(source: String) -> Self {
        Self {
            bif: false,
            source,
        }
    }

    pub fn is_bif(&self) -> bool {
        self.bif
    }

    pub fn run<Src>(&self, _data_source: &Src) -> EvalResult where Src: DataSource {
        Err("Not implemented yet".into())
    }
}

lazy_static! {
    static ref FUNCTIONS: Arc<Mutex<HashMap<String, Function>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn parse(src: &str) -> ParseResult {
    Ok(Function::new(src.into()))
}

pub fn find_function(id: &str) -> Option<Function> {
    let functions = FUNCTIONS.lock().unwrap();
    functions.get(id).cloned()
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

#[no_mangle]
pub extern fn Java_org_dedda_cellar_core_FunctionCore_delete(
    env: JNIEnv,
    _class: JClass,
    function_id: JString,
) {
    let function_id = jstring_to_string(&env, function_id);
    let mut functions = FUNCTIONS.lock().unwrap();
    functions.remove(&function_id);
}

#[no_mangle]
pub extern fn Java_org_dedda_cellar_core_FunctionCore_listIds(
    env: JNIEnv,
    _class: JClass,
) -> jobjectArray {
    let ids: Vec<String> = {
        let functions = FUNCTIONS.lock().unwrap();
        functions.iter().filter(|(_, v)| !v.is_bif()).map(|(k, _)| k).cloned().collect()
    };
    let arr = env.new_object_array(
        ids.len() as i32,
        env.find_class(CLASS_NAME_STRING).unwrap(),
        env.new_string("").unwrap()).unwrap();
    ids.into_iter().enumerate().for_each(|(idx, id)| {
        env.set_object_array_element(arr, idx as i32, env.new_string(&id).unwrap()).unwrap()
    });
    arr
}