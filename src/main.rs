use std::boxed::Box;

#[derive(Debug)]
struct SearchTree<'a, T: 'a> {
    left: Option<Box<&'a SearchTree<'a,T>>>,
    right: Option<Box<&'a SearchTree<'a,T>>>,
    key: T,
}
fn do_nothing() {

}
impl<'a, T> SearchTree<'a, T> where T: std::fmt::Debug + std::cmp::Eq + std::cmp::PartialOrd {
    fn inorder_tree(&self) {
        match self.left {
            Some(ref tree) => tree.inorder_tree(),
            None => do_nothing(),
        }
        println!("self.key = {:#?}", self.key);
        match self.right {
            Some(ref tree) => tree.inorder_tree(),
            None => do_nothing(),
        }
    }

    fn search_tree(&'a self, key: T ) -> Option<&'a SearchTree<'a, T>> {
        if self.key == key {
            return Some(&self);
        }

        if key < self.key {
            match self.left {
                Some(ref tree) => tree.search_tree(key),
                None => None
            }
        } else {
            match self.right {
                Some(ref tree) => tree.search_tree(key),
                None => None
            }
        }
    }
    fn minimum_tree(&'a self) -> &'a SearchTree<'a, T> {
        match self.left {
            Some(ref tree) => tree.minimum_tree(),
            None => &self
        }
    }
}

fn main() {
    let left = SearchTree {
        left: None,
        right: None,
        key: 32,
    };

    let right = SearchTree {
        left: None,
        right: None,
        key: 43,
    };

    let root = SearchTree {
        left: Some(Box::new(&left)),
        right: Some(Box::new(&right)),
        key: 40,
    };

    root.inorder_tree();
    left.inorder_tree();

    let tree = root.search_tree(43);
    match tree {
        Some(t) => println!("t.key = {:#?}", t.key),
        None => println!(" None "),

    }


    let x = create_box_value();
    println!("x = {:#?}", x);

    let y = root.minimum_tree();
    println!("minimum = {:#?}", y);
    
}

fn create_box_value<'a>() -> Box<i32> {
    Box::new(32)
}
