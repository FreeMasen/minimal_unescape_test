static JS: &str = include_str!("failing.js");
fn main() {
    for line in JS.lines() {
        if let Some(s) = unescape::unescape(line) {
            println!("successfully escaped value: {} -> {}", line, s);
        } else {
            println!("failed to escape value {}", line);
        };
    }
}
