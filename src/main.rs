use std::ptr::null_mut;
use std::mem;
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Raw<T> {
    ptr: *const Node<T>,
}
impl<T> Raw<T> {
    fn none() -> Self {
        Raw { ptr: null_mut() }
    }
    fn some(ptr: &mut Node<T>) -> Self {
        Raw { ptr: ptr }
    }
    fn as_mut(&mut self) -> Option<&mut Node<T>> {
        unsafe {
            if self.ptr.is_null() {
                None
            } else {
                Some(&mut *(self.ptr as *mut Node<T>))
            }
        }

    }

    fn take(&mut self) -> Self {
        mem::replace(self, Raw::none())
    }
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Raw<T>,
}
impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node { elem: elem, next: None, prev: Raw::none() }
    }
    fn link(&mut self, mut next: Box<Self>) {
        next.prev = Raw::some(self);
        self.next = Some(next);
    }
    fn take_next(&mut self) -> Option<Box<Self>> {
        let mut next = self.next.take();
        next.as_mut().map(|node| {
            node.prev = Raw::none();
        });
        next
    }
}
#[derive(Debug)]
struct LinedList<T> {
    head: Link<T>,
    tail: Raw<T>,
    len: usize,
}

impl<T> LinedList<T> {
    fn new() -> Self {
        LinedList { head: None, tail: Raw::none(), len: 0 }
    }

    fn push_back(&mut self, elem: T) {
        self.len += 1;
        let mut node = Box::new(Node::new(elem));
        let mut old_tail = mem::replace(&mut self.tail, Raw::some(&mut *node));

        match old_tail.as_mut() {
            Some(tail) => tail.link(node),
            None => self.head = Some(node),
        }

    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().as_mut().and_then(|tail| {
            self.len -= 1;
            match tail.prev.as_mut() {
                Some(prev) => {
                    self.tail = Raw::some(prev);
                    prev.next.take().map(|node| node.elem)
                },
                None => self.head.take().map(|node| node.elem)
            }
        })
    }

    fn push_front(&mut self, elem: T) {
        self.len += 1;
        let mut new_head = Box::new(Node::new(elem));
        match self.head.take() {
            Some(head) => new_head.link(head),
            None => self.tail = Raw::some(&mut new_head)
        }
        self.head = Some(new_head);
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().and_then(|mut head| {
            // head is Box<Node<T>>
            self.len -= 1;
            head.take_next().map(|mut node| {
                node.prev = Raw::none();
                self.head = Some(node);
            });

            Some(head.elem)

        })
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            head: &self.head,
            tail: &self.tail,
            nelem: self.len,
        }
    }

}

/// 三种格式,
/// iter()   // over &T
/// iter_mut()   // over &mut T
/// into_iter()     // over T
///
/// 实现 Iterator
/// 1. 创建一个 struct 保存 iterator 状态
/// 2. 为这个 struct 实现 Iterator traid

struct Iter<'a, T: 'a> {
    head: &'a Link<T>,
    tail: &'a Raw<T>,
    nelem: usize,

}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.nelem == 0 {
            return None
        }

        self.head.as_ref().and_then(|head| {
            self.nelem -= 1;
            self.head = &head.next;
            Some(&head.elem)
        })
    }
}
fn main() {
    let mut l = LinedList::new();
    for i in 0..30 {
        l.push_back(i);
    }

    for i in l.iter() {
        println!("i = {:#?}", i);
    }
}

#[test]
fn push_pop_test() {
    let mut l = LinedList::new();
    l.push_back(30);
    assert_eq!(l.len, 1);
    l.push_back(20);
    assert_eq!(l.len, 2);
    assert_eq!(l.pop_back(), Some(20));
    assert_eq!(l.len, 1);
    assert_eq!(l.pop_back(), Some(30));
    assert_eq!(l.len, 0);
    l.push_back(22);
    assert_eq!(l.len, 1);
    assert_eq!(l.pop_back(), Some(22));
    assert_eq!(l.len, 0);
    assert_eq!(l.pop_back(), None);
    assert_eq!(l.len, 0);
    assert_eq!(l.pop_back(), None);
    assert_eq!(l.len, 0);
}

#[test]
fn push_front_test() {
    let mut list = LinedList::new();
    assert_eq!(list.len, 0);
    list.push_front(30);
    assert_eq!(list.len, 1);
    list.push_front(12);
    assert_eq!(list.len, 2);
    assert_eq!(list.pop_back(), Some(30));
    assert_eq!(list.len, 1);
    assert_eq!(list.pop_back(), Some(12));
    assert_eq!(list.len, 0);
}

#[test]
fn push_front_back_test() {
    let mut list = LinedList::new();
    assert_eq!(list.len, 0);
    list.push_front(30);
    assert_eq!(list.len, 1);
    list.push_front(12);
    assert_eq!(list.len, 2);
    list.push_back(22);
    assert_eq!(list.len, 3);
    assert_eq!(list.pop_back(), Some(22));
    assert_eq!(list.len, 2);
    assert_eq!(list.pop_back(), Some(30));
    assert_eq!(list.len, 1);
    assert_eq!(list.pop_back(), Some(12));
    assert_eq!(list.len, 0);
}

#[test]
fn pop_front_test() {
    let mut list = LinedList::new();
    list.push_front(30);
    list.push_front(12);
    list.push_back(22);
    assert_eq!(list.pop_front(), Some(12));
    assert_eq!(list.pop_front(), Some(30));
    assert_eq!(list.pop_front(), Some(22));
    assert_eq!(list.pop_front(), None);
}

