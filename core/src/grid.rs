use jni::sys::{jstring, jint};
use jni::JNIEnv;
use jni::objects::JClass;

#[no_mangle]
pub extern fn Java_org_dedda_cellar_core_GridCore_contentAt(
    env: JNIEnv,
    _class: JClass,
    x: jint,
    y: jint,
) -> jstring {
    env.new_string("Hello").expect("Couldn't create Java String!").into_inner()
}