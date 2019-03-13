use crate::boxpoints::List::{Cons, Nil};

// Using a Box<T> to Store Data on the Heap
fn a() {
    let a = Box::new(5);
    println!("a = {}", a)
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn b() {
    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
}