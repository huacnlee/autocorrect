extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Ignorer {
    ignorer: autocorrect::ignorer::Ignorer,
}

#[wasm_bindgen]
impl Ignorer {
    #[wasm_bindgen(constructor)]
    pub fn new(work_dir: &str) -> Ignorer {
        let ignorer = autocorrect::ignorer::Ignorer::new(work_dir);

        Ignorer { ignorer }
    }

    #[wasm_bindgen(js_name = "isIgnored")]
    pub fn is_ignored(&self, path: &str) -> bool {
        self.ignorer.is_ignored(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ignored() {
        let current_dir = std::env::current_dir().unwrap();
        let work_dir = current_dir.parent().unwrap().to_str().unwrap();
        let ignorer = Ignorer::new(work_dir);
        assert!(ignorer.is_ignored("src/main.rs"));
        assert!(ignorer.is_ignored("pkg/foo/bar"));
    }
}
