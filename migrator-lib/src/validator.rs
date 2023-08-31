use std::path::Path;

pub fn validate_name(name: &str) -> bool {
    !name.contains('/')
}

pub fn is_initialized() -> bool {
    Path::new("./migrations").exists()
}
