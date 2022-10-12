extern crate rb_sys;

use autocorrect::{format, format_for};
use rb_sys::{
    rb_define_module, rb_define_module_function, rb_string_value_cstr, rb_utf8_str_new, VALUE,
};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_long};

#[inline]
unsafe fn cstr_to_string(str: *const c_char) -> String {
    CStr::from_ptr(str).to_string_lossy().into_owned()
}

#[no_mangle]
unsafe extern "C" fn pub_format(_klass: VALUE, mut input: VALUE) -> VALUE {
    let ruby_string = cstr_to_string(rb_string_value_cstr(&mut input));

    let result = format(&ruby_string);
    let size = result.len() as c_long;
    let result_cstring = CString::new(result).unwrap();

    rb_utf8_str_new(result_cstring.as_ptr(), size)
}

#[no_mangle]
unsafe extern "C" fn pub_format_for(_klass: VALUE, mut input: VALUE, mut filepath: VALUE) -> VALUE {
    let input = cstr_to_string(rb_string_value_cstr(&mut input));
    let filepath = cstr_to_string(rb_string_value_cstr(&mut filepath));

    let result = format_for(&input, &filepath);
    let size = result.out.len() as i64;
    let result_cstring = CString::new(result.out).unwrap();

    rb_utf8_str_new(result_cstring.as_ptr(), size)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_autocorrect() {
    let name = CString::new("AutoCorrect").unwrap();

    let klass = unsafe { rb_define_module(name.as_ptr()) };
    let fn_format_name = CString::new("format").unwrap();
    let fn_format_for_name = CString::new("format_for").unwrap();

    let format_callback = unsafe {
        std::mem::transmute::<
            unsafe extern "C" fn(VALUE, VALUE) -> VALUE,
            unsafe extern "C" fn() -> VALUE,
        >(pub_format)
    };

    let format_for_callback = unsafe {
        std::mem::transmute::<
            unsafe extern "C" fn(VALUE, VALUE, VALUE) -> VALUE,
            unsafe extern "C" fn() -> VALUE,
        >(pub_format_for)
    };
    unsafe {
        rb_define_module_function(klass, fn_format_name.as_ptr(), Some(format_callback), 1);

        rb_define_module_function(
            klass,
            fn_format_for_name.as_ptr(),
            Some(format_for_callback),
            2,
        );
    }
}
