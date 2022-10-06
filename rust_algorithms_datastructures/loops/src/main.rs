

pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32
}

impl Iterator for Stepper {

    type Item = i32;

    fn next(&mut self) -> Option<i32>{
        if self.curr >= self.max {
            return None;
        }

        let res = self.curr;
        self.curr += self.step;

        Some(res)
    }
 }

fn main() {

    // loop_testbed();

    let mut st = Stepper {
        curr: 2,
        step: 3,
        max: 15
    };


    // loop {
    //     match st.next(){ 
    //         Some(v) => println!("loop {}",v),
    //         None => break
    //     }
    // }

    // Same statement using while let

    let mut st2 = Stepper {
        curr: 1,
        step: 3,
        max: 20
    };


    while let Some(v) = st2.next(){
        println!("val st2 {}", v);
    }


    // For loop implementation

    let mut st3 = Stepper {
        curr: 0,
        step: 5,
        max: 30
    };

    for i in st3 {
        println!("{}",i)
    }
    
}

fn loop_testbed(){

    let mut n = 0;

    // With infinite loops
    loop{
        n += 1;

        if n > 10 {
            break;
        }
        println!("Hello, world! {}", n);
    }

    n = 0;

    // With while loops
    while n < 10 {
        n += 1;
        println!("Hello, world! {}", n);
    }

    for i in 0..10 {
        println!("Hello, world {}", i);
    }

}