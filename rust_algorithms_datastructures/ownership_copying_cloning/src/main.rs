
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32
}

#[derive(Debug, Clone, Copy)]
struct Point{
    x: i32,
    y: i32
}

impl Person {
    pub fn new(name:String, age: u32) -> Self{
        Person {name: name, age:age}
    }
}

fn main() {

    let p = Point {x: 15, y: 20};
    // By deriving copy, clone not necessary
    let mut p2 = p;
    p2.y = 3;
    

    let a = Person::new("Steve".to_string(), 5);
    // Clone required to create another instance
    let mut b = a.clone();
    b.name = "Rogers".to_string();

    println!(" Original obj has val {:?} and cloned obj has val {:?}", a, b);
    println!("Copy: Original obj has val {:?} and copied obj has val {:?}", p, p2);
}
