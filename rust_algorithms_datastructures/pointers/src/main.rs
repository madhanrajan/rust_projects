
#[derive(Debug, Clone)]
pub struct Person{
    name: String,
    age: i32
}

impl Person{
    pub fn new(name: String, age: i32) -> Self{
        Person {name, age}
    }

    pub fn get(&self) -> String {
        format!("{} is {} years old.", self.name, self.age)
    }

    pub fn increment_age(&mut self){
        self.age += 1; 
    }

    pub fn destroy(self){}

}

fn main() {

    let mut p = Person::new("John".to_string(), 32);
    println!("{}",p.get());
    p.increment_age();
    println!("{}",p.get());
    // p.destroy();

    let age = get_age(&p);
    // Increment function doesn't work before there is already a reference to the pointer
    // p.increment_age();

    
    println!("Person's age is {}", age);
    p.increment age();

    
}

pub fn get_age(s: &Person) -> &i32 {
    &s.age
}