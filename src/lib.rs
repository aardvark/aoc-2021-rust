pub fn load_to_strings<T>(path: &str, f: fn(&String) -> Vec<T>) -> Vec<T> {
    let res = std::fs::read_to_string(path);
    return if res.is_ok() {
        f(&res.unwrap_or_default())
    } else {
        Vec::new()
    };
}
