use autocorrect::{LineResult, LintResult};
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jstring};
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

#[no_mangle]
pub unsafe extern "system" fn Java_AutoCorrect_lintFor(
    env: JNIEnv,
    _class: JClass,
    input: JString,
    filename: JString,
) -> jlong {
    let input: String = env.get_string(input).unwrap().into();
    let filename: String = env.get_string(filename).unwrap().into();

    let result = autocorrect::lint_for(&input, &filename);
    Box::into_raw(Box::new(result)) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_AutoCorrect_lintResultGetString(
    env: JNIEnv,
    _class: JClass,
    result: jlong,
    field: JString,
) -> jstring {
    let result = &*(result as *const LintResult);
    let field: String = env.get_string(field).unwrap().into();
    let val = match field {
        "filepath" => result.filepath.clone(),
        "raw" => result.raw.clone(),
    }

    let output = env.new_string(val).unwrap();
    output.into_raw()
}
