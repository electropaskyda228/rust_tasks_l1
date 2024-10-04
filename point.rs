pub struct Point {
    x: f32,
    y: f32
}


impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point {
            x: x,
            y: y
        }
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn get_x(&self) -> f32{
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    // Функция нахождения расстояния между двумя точками
    pub fn get_length(&self, pnt: &Point) -> f32{
        ((self.x - pnt.get_x()).powf(2.0) + (self.y - pnt.get_y()).powf(2.0)).sqrt()
    }
}