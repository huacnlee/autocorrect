use autocorrect::{LineResult, LintResult};
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jintArray, jlong, jsize, jstring};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_io_github_huacnlee_AutoCorrect_format(
    env: JNIEnv,
    _class: JClass,
    text: JString,
) -> jstring {
    let text: String = env.get_string(text).unwrap().into();
    let out = autocorrect::format(&text);
    let output = env.new_string(out).unwrap();

    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_io_github_huacnlee_AutoCorrect_formatFor(
    env: JNIEnv,
    _class: JClass,
    text: JString,
    filename: JString,
) -> jstring {
    let text: String = env.get_string(text).unwrap().into();
    let filename: String = env.get_string(filename).unwrap().into();
    let result = autocorrect::format_for(&text, &filename);
    let output = env.new_string(result.out).unwrap();

    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_io_github_huacnlee_AutoCorrect_nativeLintFor(
    env: JNIEnv,
    _class: JClass,
    text: JString,
    filename: JString,
) -> jlong {
    let text: String = env.get_string(text).unwrap().into();
    let filename: String = env.get_string(filename).unwrap().into();

    let result = autocorrect::lint_for(&text, &filename);
    Box::into_raw(Box::new(result)) as jlong
}

#[no_mangle]
/// # Safety
pub unsafe extern "system" fn Java_io_github_huacnlee_AutoCorrect_nativeLintResultString(
    env: JNIEnv,
    _class: JClass,
    lint_result: jlong,
    field: JString,
) -> jstring {
    let result = &*(lint_result as *const LintResult);
    let field: String = env.get_string(field).unwrap().into();

    let val = match field.as_str() {
        "filepath" => result.filepath.clone(),
        "raw" => result.raw.clone(),
        _ => "".to_owned(),
    };

    let output = env.new_string(val).unwrap();
    output.into_raw()
}

#[no_mangle]
/// # Safety
pub unsafe extern "system" fn Java_io_github_huacnlee_AutoCorrect_nativeLintResultLines(
    env: JNIEnv,
    _class: JClass,
    lint_result: jlong,
) -> jintArray {
    let result = &*(lint_result as *const LintResult);

    let mut line_ptrs: Vec<jlong> = vec![];
    for line in result.lines.clone() {
        let line_ptr = Box::into_raw(Box::new(line)) as jlong;
        line_ptrs.push(line_ptr);
    }

    let lines = env.new_long_array(line_ptrs.len() as jsize).unwrap();
    env.set_long_array_region(lines, 0, &line_ptrs).unwrap();
    lines
}

#[no_mangle]
/// # Safety
pub unsafe extern "system" fn Java_io_github_huacnlee_AutoCorrect_nativeLineResultString(
    env: JNIEnv,
    _class: JClass,
    line_result: jlong,
    field: JString,
) -> jstring {
    let result = &*(line_result as *const LineResult);
    let field: String = env.get_string(field).unwrap().into();
    let val = match field.as_str() {
        "new" => result.new.clone(),
        "old" => result.old.clone(),
        _ => "".to_owned(),
    };

    let output = env.new_string(val).unwrap();
    output.into_raw()
}

#[no_mangle]
/// # Safety
pub unsafe extern "system" fn Java_io_github_huacnlee_AutoCorrect_nativeLineResultLong(
    env: JNIEnv,
    _class: JClass,
    line_result: jlong,
    field: JString,
) -> jlong {
    let result = &*(line_result as *const LineResult);
    let field: String = env.get_string(field).unwrap().into();

    let val = match field.as_str() {
        "line" => result.line,
        "col" => result.col,
        "severity" => result.severity as usize,
        _ => 0,
    };

    val as jlong
}

#[no_mangle]
#[allow(unused)]
pub extern "system" fn Java_io_github_huacnlee_AutoCorrect_loadConfig(
    env: JNIEnv,
    _class: JClass,
    config_str: JString,
) {
    let config_str: String = env.get_string(config_str).unwrap().into();

    match autocorrect::config::load(&config_str) {
        Ok(_config) => Ok(()),
        Err(e) => Err(&format!("{e}")),
    };
}

#[no_mangle]
pub extern "system" fn Java_io_github_huacnlee_AutoCorrect_nativeNewIgnorer(
    env: JNIEnv,
    _class: JClass,
    work_dir: JString,
) -> jlong {
    let work_dir: String = env.get_string(work_dir).unwrap().into();

    let ignorer = autocorrect::ignorer::Ignorer::new(&work_dir);

    Box::into_raw(Box::new(ignorer)) as jlong
}

#[no_mangle]
/// # Safety
pub unsafe extern "system" fn Java_io_github_huacnlee_AutoCorrect_nativeIgnorerIsIgnored(
    env: JNIEnv,
    _class: JClass,
    ignorer: jlong,
    filepath: JString,
) -> jboolean {
    let ignorer = &*(ignorer as *const autocorrect::ignorer::Ignorer);
    let filepath: String = env.get_string(filepath).unwrap().into();

    ignorer.is_ignored(&filepath) as jboolean
}
