use jni::JNIEnv;
use jni::objects::JString;

pub const CLASS_NAME_STRING: &'static str = "java/lang/String";

pub fn jstring_to_string(
    env: &JNIEnv,
    j_string: JString,
) -> String {
    env.get_string(j_string).unwrap().to_str().unwrap().into()
}