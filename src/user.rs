#![allow(unused_variables, dead_code)]
struct User {
    name: String,
    age: u16,
    weight: f32,
}

impl User {
    fn new(name: String, age: u16, weight: f32) -> Self {
        Self { 
            name: name, 
            age: age, 
            weight: weight
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
    
    fn age(&self) -> u16 {
        self.age
    }

    fn weight(&self) -> f32 {
        self.weight
    }

    fn set_age(&mut self, new_age: u16) {

        self.age = new_age;
    }

    fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}

fn main() {
    let mut bob = User::new(String::from("Bob"), 18, 85.6);
    println!("{}'s age is {} and his weight is {}.",bob.name(),bob.age(),bob.weight());

    bob.set_weight(79.5);

    println!("New weight of {} is {}.", bob.name(), bob.weight());
}

