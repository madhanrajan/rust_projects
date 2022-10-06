#[derive(Debug)]

pub enum Res<T,E> {
    Thing(T),
    Error(E)
}

// Using options
pub enum Option<T> {
    Some(T),
    None
}

fn main() {
    result_testbed();
}

fn result_testbed(){
    let a = divide(15,5);
    let b = divide(10,0);

    println!("Hello, world!");

    println!("a = {:?}, b = {:?}",a,b);
    
    // match a {
    //     Res::Thing(val) => println!("val = {}", val),
    //     _ => {},
    // }


    // Alternative to the above statement
    if let Res::Thing(v) = a {
        println!("val = {}", v);
    }

    // Use Result library instead, which is similar 
    if let Ok(v) = divide_std(6,3){
        println!("{}", v);
    }
}


fn divide(a:i32,b:i32) -> Res<i32, String>{
    if b == 0 {
        Res::Error("Cannot divide by 0".to_string())
    }else{
        Res::Thing(a/b)
    }
}

// Using std library
fn divide_std(a:i32,b:i32) -> Result<i32, String>{
    if b == 0 {
        Err("Cannot divide by 0".to_string())
    }else{
        Ok(a/b)
    }
}