extern crate jni;
#[macro_use]
extern crate lazy_static;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;

pub mod grid;
pub mod jnitools;
pub mod func;

#[no_mangle]
pub extern fn Java_org_dedda_cellar_core_CoreKt_ping(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    env.new_string("Pong").expect("Couldn't create Java String!").into_inner()
}
