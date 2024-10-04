fn main() {
    print!("{}\n", change_bit_to_zero(8, 4));
    print!("{}", change_bit_to_one(32, 4));
}

// Меняем бит, если надо
fn change_bit_to_zero(number: i64, i: u32) -> i64 {
    if found_bit(number, i) == 1 {
        return number - 2_i64.pow(i - 1);
    }
    number
}

fn change_bit_to_one(number: i64, i: u32) -> i64 {
    if found_bit(number, i) == 0 {
        return number + 2_i64.pow(i - 1);
    }
    number
} 

// Находим нужный бит в числе
fn found_bit(number: i64, i: u32) -> i64 {
    let mut nmb_copy: i64 = number;
    for _ in 1..i {
        nmb_copy /= 2;
    }
    nmb_copy % 2
}