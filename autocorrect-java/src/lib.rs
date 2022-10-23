use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_AutoCorrect_format(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input: String = env.get_string(input).unwrap().into();
    let out = autocorrect::format(&input);
    let output = env.new_string(out).unwrap();

    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_AutoCorrect_formatFor(
    env: JNIEnv,
    _class: JClass,
    input: JString,
    filename: JString,
) -> jstring {
    let input: String = env.get_string(input).unwrap().into();
    let filename: String = env.get_string(filename).unwrap().into();
    let result = autocorrect::format_for(&input, &filename);
    let output = env.new_string(result.out).unwrap();

    output.into_raw()
}
