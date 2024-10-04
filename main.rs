use std::collections::HashSet;

fn main() {
    let s_1 = HashSet::from(["Tom", "Robert", "Peka"]);
    let s_2 = HashSet::from(["Peka", "Tom", "Lot"]);

    let s_3: HashSet<_> = s_1.intersection(&s_2).collect();
    println!("{:?}", s_3);
}