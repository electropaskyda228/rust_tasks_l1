use std::collections::HashMap;

fn main() {
    let s = String::from("youR string");
    print!("{}", checker(&s));
}


fn checker(s: &str) -> bool {
    let mut data_check = HashMap::new();

    for symbol in s.to_lowercase().chars() {
        if data_check.contains_key(&symbol) {
            return false;
        }
        data_check.insert(symbol, 1);
    }

    true
}
