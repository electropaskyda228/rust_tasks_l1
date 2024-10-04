mod point;

use point::Point;

fn main() {
    let pnt_1 = Point::new(12.3, 12.4);
    let pnt_2 = Point::new(21.4, 25.6);

    print!("{}", pnt_1.get_length(&pnt_2));
}