use std::{env, fs, path::Path};

pub const FOO: [&str; 3] = ["foo", "bar", "baz"];

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("spellchecks.rs");

    let data = fs::read_to_string(Path::new("./spellcheck/noun.txt"))
        .expect("Failed to read SPELLCHECK.txt");
    let pair_re: regex::Regex = regex::Regex::new(r"\s*=\s*").unwrap();

    let mut lines = data.lines().collect::<Vec<_>>();

    // Sort: Longger first, then alphabetically.
    lines.sort_by(|a, b| {
        let mut a = *a;
        let mut b = *b;

        let pair_a = pair_re.split(a).collect::<Vec<_>>();
        if pair_a.len() == 2 {
            a = pair_a[0];
        }
        let pair_b = pair_re.split(b).collect::<Vec<_>>();
        if pair_b.len() == 2 {
            b = pair_b[0];
        }
        a = a.trim();
        b = b.trim();

        b.len().cmp(&a.len()).then(a.cmp(b))
    });

    let mut dicts: Vec<String> = vec![];

    let re_dicts = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut left_str = *l;
            let mut right_str = *l;

            let pair = pair_re.split(l).collect::<Vec<_>>();
            if pair.len() == 2 {
                left_str = pair[0];
                right_str = pair[1];
            }

            dicts.push(format!(
                r#""{}" => "{}""#,
                left_str.trim(),
                right_str.trim()
            ));

            format!(
                r#""{}" => regexp!(r"(?im)(\s|^)+({})(\s|$)+")"#,
                left_str.trim(),
                left_str.trim().replace('-', r"\-").replace('.', r"\.")
            )
        })
        .collect::<Vec<_>>();

    // println!(
    //     "---------------------------- n\n{}\n\n-----------------------------",
    //     array_str
    // );
    // panic!("Break");

    fs::write(
        &dest_path,
        format!(
            r#"lazy_static! {{
                static ref SPELLCHECK_DICT: HashMap<&'static str, &'static str> = map![ {} ];
                static ref SPELLCHECK_RE_DICT: HashMap<&'static str, Regex> = map![ {} ]; 
            }}"#,
            dicts.join(",\n"),
            re_dicts.join(",\n")
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=speelcheck/noun.txt");
    println!("cargo:rerun-if-changed=build.rs");
}
