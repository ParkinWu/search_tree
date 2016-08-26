use std::fmt::Debug;

#[derive(Debug)]
struct Node<T: Debug> {
    val: T,
    next: Option<Box<Node<T>>>
}

impl<T: Debug> Drop for Node<T> {
    fn drop(&mut self) {
        println!("--------------------------------");
        println!("node droped val = {:#?}", self.val);
        println!("--------------------------------");
    }
}

#[derive(Debug)]
struct LinkedList<T: Debug> {
    head: Option<Box<Node<T>>>,
}

impl<T: Debug> Drop for LinkedList<T> {
    fn drop(&mut self) {
        println!("--------------------------------");
        println!("list droped");
        println!("--------------------------------");
    }
}

impl<T: Debug> LinkedList<T> {
    fn new() -> LinkedList<T> {
        return LinkedList {
            head: None,
        };

    }

//    fn delete_last(&mut self) -> Option<Box<Node<T>>> {
//        match self.head {
//            Some(ref mut n) => {
//                unsafe {
//                    let mut raw_ptr = n as *mut Box<Node<T>>;
//                    while let Some(ref mut n) = (*raw_ptr).next {
//                        raw_ptr = n;
//                    }
//                    println!("*raw_ptr = {:#?}", *raw_ptr);
//                    return Some(*raw_ptr);
//                }
//            },
//            None => {
//                return None;
//            },
//        }
//    }

    fn append(&mut self, val: T) {
        let temp = Some(Box::new(Node {
            val: val,
            next: None,
        }));
        match self.head {
            Some(ref mut v) => {
                unsafe {
                    let mut raw_ptr = v as *mut Box<Node<T>>;
                    loop {
                        match (*raw_ptr).next {
                            Some(ref mut n) => {
                                raw_ptr = n as *mut Box<Node<T>>;
                            },
                            None => {
                                break;
                            },
                        }
                    }

                    (*raw_ptr).next = temp;
                }
            },
            None => {
                self.head = temp;
                return;
            },
        }

    }
}

fn main() {
    let mut list = LinkedList::new();
    list.append(20);
    list.append(30);
    list.append(35);
    println!("list = {:#?}", list);
}