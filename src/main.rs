use std::boxed::Box;

#[derive(Debug)]
struct SearchTree<T> {
    left: Option<Box<SearchTree<T>>>,
    right: Option<Box<SearchTree<T>>>,
    key: T,
}

impl<T> SearchTree<T> where T: std::fmt::Debug {
    fn inorder_tree(self) {
        self.left.map(|x| x.inorder_tree());
        println!("self.key = {:#?}", self.key);
        self.right.map(|x| x.inorder_tree());
    }
}

fn main() {
    println!("Hello, world!");
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
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
        key: 40,
    };
    root.inorder_tree();

}
