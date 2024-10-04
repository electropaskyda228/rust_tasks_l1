use std::any::{Any, TypeId};

fn main() {
    let boxed: Box<dyn Any> = Box::new(4_i32);

    assert_eq!((&*boxed).type_id(), TypeId::of::<i32>());
}