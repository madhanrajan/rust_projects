

use std::mem::replace;

struct Node<T>{
    data: T,
    next: Option<Box<Node<T>>>
}

struct Stack<T>{
    top: Option<Node<T>>
}

impl<T> Node<T>{
    fn new(data: T) -> Node<T> {
        Node {data, next: None}
    }
}

impl<T> Stack<T>{
    fn new() -> Stack<T> {
        Stack { top: None }
    }

    fn is_empty(&self) -> bool {
        if let Some(_) = self.top {
            false 
        }else{
            true
        }
    }

    fn push(&mut self, data:T) {

        let mut node = Node::new(data);

        if let Some(v) = replace(&mut self.top, None){
            node.next = Some(Box::new(v));
        }

        self.top = Some(node);
    }

    fn pop(&mut self) -> Option<T>{

        if let Some(v) = replace(&mut self.top, None){
            self.top = match v.next {
                Some(n) => Some(*n),
                None => None
            };
            Some(v.data)
        }else{
            None
        }

    }
}

fn main() {

    let node = Node::<u32>::new(5);   
    
    let mut stk = Stack::<u32>::new();
    println!("{}",stk.is_empty());
    stk.push(5);
    println!("{}",stk.is_empty());
    if let Some(val) = stk.pop(){
        println!("Value of {} is removed. Stack is empty: {}", val, stk.is_empty());
    };
    


}
