
// Box pointers should be used to create infinite sized objects
#[derive(Debug)]
pub struct LinkedList<T>{
    data: T,
    next: Option<Box<LinkedList<T>>>
}

impl <T: std::ops::AddAssign> LinkedList<T>{
    pub fn add_up(&mut self, n:T) {
        self.data += n;
    } 
}

fn main() {

    let mut ll = LinkedList {data: 3, next: Some(Box::new(LinkedList {data: 4, next: None}))};

    ll.add_up(3);

    // Use ref mut for box references
    if let Some(ref mut v) = ll.next {
        v.add_up(8);
    }
    

    println!("This is a rust linked list: {:?}",ll);
}
