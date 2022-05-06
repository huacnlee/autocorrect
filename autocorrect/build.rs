use std::{env, fs, path::Path};

fn main() {
    let config_str = fs::read_to_string(Path::new(".autocorrectrc.default"))
        .expect("Failed to read .autocorrectrc.default");
    let code = format!(
        r###"lazy_static! {{
                static ref CURRENT_CONFIG: RwLock<Config> = RwLock::new(Config::from_str(r#"{}"#).unwrap());
            }}"###,
        config_str
    );

    // println!("{}", code);
    // panic!("");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("default_config.rs");
    fs::write(&dest_path, code).unwrap();

    println!("cargo:rerun-if-changed=.autocorrectrc.default");
    println!("cargo:rerun-if-changed=build.rs");
}
