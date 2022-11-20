use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "сильные ссылки leaf = {}, слабые ссылки = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "сильные ссылки branch = {}, слабые ссылки = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "сильные ссылки leaf = {}, слабые ссылки = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }
    println!(
        "родительский узел leaf = {:?}",
        leaf.parent.borrow().upgrade()
    );
    println!(
        "сильные ссылки leaf = {}, слабые ссылки = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
