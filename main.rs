fn main() {
    let s = String::from("snow dog sun");

    print!("{}", reverse_string(&s));
}

fn reverse_string(s: &str) -> String {
    let mut vector: Vec<&str> = Vec::new();
    for word in s.split(' ') {
        vector.push(word);
    }

    vector.reverse();

    let mut answer: String = String::from("");
    for word in vector.iter() {
        answer += word;
        answer += " ";
    }

    answer.trim().to_string()
}