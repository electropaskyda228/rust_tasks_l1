mod saferemovevec;

use saferemovevec::SafeRemoveVec;
fn main() {
    let mut vec = vec![0i8, 120i8];

    println!("{:?}", vec.safe_remove(1));
    println!("{:?}", vec.safe_remove(1));
    println!("{:?}", vec.safe_remove(0));
    println!("{:?}", vec.safe_remove(0));
}