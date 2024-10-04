fn main() {
    let s = "sun главрыба";
    println!("{}", reverse_string(s));
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}