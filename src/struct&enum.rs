#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}
#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog
}

impl Animal {
    pub fn new() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat
        }
    }

    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
    }

    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Cat => println!("cat"),
            AnimalType::Dog => println!("dog")
        }
    }
}

fn main() {
    let mut animal = Animal::new();
    animal.check_type();
    animal.change_to_cat();
    animal.change_to_dog();
    animal.check_type();
}
