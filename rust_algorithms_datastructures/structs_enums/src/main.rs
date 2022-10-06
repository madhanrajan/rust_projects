#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    fave_color: Color
}

#[derive(Debug)]
pub enum Color {
    Red (String),
    Green,
    Blue
}

impl Person {
    pub fn print(self) -> String{
        format!("Name: {}, age: {}, has {} children", self.name, self.age, self.children)
    }
}

fn main() {

    let person = Person { name: "Sam".to_string(), age: 30, children: 5, fave_color: Color::Green};

    println!("{}", person.print());

    let color = Color::Red("hi there".to_string());

    match color{
        Color::Red(s) => println!("Slightly red with val {}",s),
        Color::Green => println!("Maybe green"),
        Color::Blue => println!("Probably blue"),
        
    }
}
