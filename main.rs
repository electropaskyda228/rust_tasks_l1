use std::collections::HashMap;

fn main() {
    // Пример строки
    let s = String::from("youR string");

    // Вывод результата
    print!("{}", checker(&s));
}


fn checker(s: &str) -> bool {
    // Храним в HashMap'e для каждого символа факт его наличия в строке
    let mut data_check = HashMap::new();

    // Проходимся по строке и проверяем: не встречали ли мы уже такой символ
    for symbol in s.to_lowercase().chars() {
        if data_check.contains_key(&symbol) {
            return false;
        }
        data_check.insert(symbol, 1);
    }

    true
}
