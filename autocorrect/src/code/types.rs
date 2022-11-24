use crate::config::Config;

// dectermines file_type is support
pub fn match_filename(filename_or_ext: &str) -> String {
    let ext = get_file_extension(filename_or_ext);

    // Return file type by config
    if let Some(file_type) = Config::current().get_file_type(&ext) {
        return file_type.into();
    }

    filename_or_ext.into()
}

// dectermines file_type is support
pub fn is_support_type(filename_or_ext: &str) -> bool {
    Config::current().get_file_type(filename_or_ext).is_some()
}

// get file extension from filepath, return filename if not has exit
#[allow(clippy::comparison_chain)]
pub fn get_file_extension(filename: &str) -> String {
    let filename = filename.trim();
    if is_support_type(filename) {
        return String::from(filename);
    }

    let filename = filename.split('/').last().unwrap().to_string();
    let path_parts: Vec<&str> = filename.split('.').collect();
    let mut ext: String = path_parts.last().unwrap().to_string();

    let part_len = path_parts.len();
    if part_len > 2 {
        let double_ext = path_parts[(part_len - 2)..part_len].join(".");
        if is_support_type(double_ext.as_str()) {
            ext = double_ext
        }
    } else if part_len < 2 {
        ext = filename;
    }

    ext
}

#[cfg(test)]
mod tests {
    use crate::config::setup_test;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn is_is_support_type() {
        assert!(is_support_type("html"));
        assert!(is_support_type("htm"));
        assert!(is_support_type("html.erb"));
        assert!(is_support_type("rust"));
        assert!(is_support_type("rs"));

        assert!(!is_support_type("foo"));
        assert!(!is_support_type("index.html"));
        assert!(!is_support_type("gettext"));
        assert!(!is_support_type("gettext.po"));
        assert!(!is_support_type("gettext.pot"));
    }

    #[test]
    fn is_get_file_extension() {
        assert_eq!("text", get_file_extension("text"));
        assert_eq!("txt", get_file_extension("txt"));
        assert_eq!("rb", get_file_extension("/foo/bar/dar.rb"));
        assert_eq!("rb", get_file_extension("/foo/bar/aaa.dar.rb"));
        assert_eq!("html.erb", get_file_extension("/foo/bar/dar.html.erb"));
        assert_eq!("html.erb", get_file_extension("html.erb"));
        assert_eq!("Gemfile", get_file_extension("Gemfile"));
        assert_eq!("js", get_file_extension("/dar.js"));
        assert_eq!("dar", get_file_extension("/foo/bar/dar"));
    }

    #[test]
    fn test_match_filename() {
        setup_test();
        assert_eq!("markdown".to_owned(), match_filename("app.md"));
        assert_eq!("markdown".to_owned(), match_filename("app.mdx"));

        assert_eq!("html".to_owned(), match_filename("app.htm"));
        assert_eq!("html".to_owned(), match_filename("app.html"));
        assert_eq!("html".to_owned(), match_filename("app.html.erb"));

        assert_eq!("javascript".to_owned(), match_filename("app.js"));
        assert_eq!("javascript".to_owned(), match_filename("app.ts"));
        assert_eq!("javascript".to_owned(), match_filename("app.js.erb"));

        assert_eq!("conf".to_owned(), match_filename("app.properties"));
        assert_eq!("conf".to_owned(), match_filename("app.ini"));
        assert_eq!("conf".to_owned(), match_filename("app.toml"));
        assert_eq!("conf".to_owned(), match_filename("app.cfg"));

        assert_eq!("strings".to_owned(), match_filename("app.strings"));

        assert_eq!("python".to_owned(), match_filename("app.py"));

        assert_eq!("java".to_owned(), match_filename("main.proto"));

        assert_eq!("kotlin".to_owned(), match_filename("app.gradle"));
        assert_eq!("kotlin".to_owned(), match_filename("app.kt"));

        assert_eq!("xml".to_owned(), match_filename("zh-CN.xml"));

        assert_eq!("asciidoc".to_owned(), match_filename("bar.adoc"));
        assert_eq!("asciidoc".to_owned(), match_filename("bar.asc"));

        assert_eq!("java".to_owned(), match_filename("bar.proto"));

        assert_eq!("latex".to_owned(), match_filename("bar.tex"));

        assert_eq!("gettext".to_owned(), match_filename("bar.pot"));
        assert_eq!("gettext".to_owned(), match_filename("bar.po"));

        // Follow file type in .autocorrecrrc.default
        assert_eq!("ruby".to_owned(), match_filename("Gemfile"));
        assert_eq!("ruby".to_owned(), match_filename("Rakefile"));
        assert_eq!("ruby".to_owned(), match_filename("Profile"));
        assert_eq!("ruby".to_owned(), match_filename("foo.gemspec"));
    }
}
