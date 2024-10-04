pub trait Action {
    fn say(&self);
}


impl Action for Person {
    fn say(&self) {
        print!("Hello, {}", self.get_name());
    }
}

pub struct Person {
    name: String
} 

impl Person {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn new(name: String) -> Person {
        Person {
            name: name
        }
    } 
}

fn main() {
    let person = Person::new(String::from("Billy my boy"));
    person.say();
}