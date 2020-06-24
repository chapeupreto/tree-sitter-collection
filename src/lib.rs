use tree_sitter::Language;

extern "C" {
    pub fn tree_sitter_go() -> Language;
    pub fn tree_sitter_c() -> Language;
    pub fn tree_sitter_rust() -> Language;
    pub fn tree_sitter_javascript() -> Language;
    pub fn tree_sitter_java() -> Language;
    pub fn tree_sitter_typescript() -> Language;
    pub fn tree_sitter_tsx() -> Language;
    pub fn tree_sitter_toml() -> Language;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_all() {
        unsafe {
            tree_sitter_go();
            tree_sitter_c();
            tree_sitter_rust();
            tree_sitter_javascript();
            tree_sitter_java();
            tree_sitter_typescript();
            tree_sitter_tsx();
            tree_sitter_toml();
        }
    }
}
