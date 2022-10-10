
#[derive(Debug, Clone)]
struct Person{
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

    pub fn destroy(self){
        
    }

}

fn main() {

    let mut p = Person::new("John".to_string(), 32);
    println!("{}",p.get());
    p.increment_age();
    println!("{}",p.get());
    p.destroy();


}
