use std::path::Path;

pub struct Ignorer {
    ignorer: ignore::gitignore::Gitignore,
}

static AUTOCORRECTIGNORE: &str = ".autocorrectignore";
static GITIGNORE: &str = ".gitignore";

impl Ignorer {
    pub fn new<P>(work_dir: P) -> Ignorer
    where
        P: AsRef<Path>,
    {
        let work_dir = work_dir.as_ref();
        let mut builder = ignore::gitignore::GitignoreBuilder::new(&work_dir);
        builder.add(work_dir.join(AUTOCORRECTIGNORE));
        builder.add(work_dir.join(GITIGNORE));
        let ignorer = builder.build().expect("failed to build ignorer");
        Ignorer { ignorer }
    }

    pub fn is_ignored<P>(&self, path: P) -> bool
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        self.ignorer
            .matched_path_or_any_parents(path, false)
            .is_ignore()
            || self
                .ignorer
                .matched_path_or_any_parents(path, true)
                .is_ignore()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ignored() {
        let current_dir = std::env::current_dir().unwrap();
        let work_dir = current_dir.parent().unwrap().to_path_buf();
        // println!("-- work_dir: {:?}", work_dir);
        let ignorer = Ignorer::new(&work_dir);
        assert!(ignorer.is_ignored("src/main.rs"));
        assert!(ignorer.is_ignored("pkg/foo/bar"));
        assert!(ignorer.is_ignored("node_modules/@huacnlee/autocorrect/index.js"));
        assert!(!ignorer.is_ignored("example/index.js"));
        assert!(!ignorer.is_ignored("example/package.json"));
        assert!(ignorer.is_ignored("test/fixtures/this-file-will-ignore.rs"));
    }
}
