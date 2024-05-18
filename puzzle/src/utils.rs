pub fn is_integer(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}