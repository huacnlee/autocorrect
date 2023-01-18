use std::{env, fs, path::Path};

fn main() {
    let config_str = fs::read_to_string(Path::new("../.autocorrectrc.template"))
        .expect("Failed to read .autocorrectrc.template");
    let code = format!(r###"static CONFIG_TEMPLATE: &str = r#"{config_str}"#;"###);

    // println!("{}", code);
    // panic!("");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config_template.rs");
    fs::write(dest_path, code).unwrap();

    println!("cargo:rerun-if-changed=.autocorrect.template");
    println!("cargo:rerun-if-changed=build.rs");
}
