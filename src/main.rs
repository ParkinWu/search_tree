use std::boxed::Box;

#[derive(Debug)]
struct SearchTree<T> {
    left: Option<Box<SearchTree<T>>>,
    right: Option<Box<SearchTree<T>>>,
    key: T,
}

impl<T> SearchTree<T> where T: std::fmt::Debug + std::cmp::Eq + std::cmp::PartialOrd {
    fn inorder_tree(self) {
        self.left.map(|x| x.inorder_tree());
        println!("self.key = {:#?}", self.key);
        self.right.map(|x| x.inorder_tree());
    }

    fn search_tree(self, key: T ) -> Option<SearchTree<T>> {
        if self.key == key {
            return Some(self);
        }

        if key < self.key {
            match self.left {
                Some(tree) => tree.search_tree(key),
                None => None
            }
        } else {
            match self.right {
                Some(tree) => tree.search_tree(key),
                None => None
            }
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
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
        key: 40,
    };

//    root.inorder_tree();

    let tree = root.search_tree(43);
    match tree {
        Some(t) => println!("t.key = {:#?}", t.key),
        None => println!(" None "),

    }

}
