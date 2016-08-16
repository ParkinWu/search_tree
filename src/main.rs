use std::boxed::Box;

#[derive(Debug)]
struct SearchTree<T> {
    left: Option<Box<SearchTree<T>>>,
    right: Option<Box<SearchTree<T>>>,
    key: T,
}

impl<T> SearchTree<T> where T: std::fmt::Debug + std::cmp::Eq + std::cmp::PartialOrd + std::clone::Clone {
    fn inorder_tree(&self) {
        if let Some(ref tree) = self.left {
            tree.inorder_tree();
        }
        println!("self.key = {:#?}", self.key);
        if let Some(ref tree) = self.right {
            tree.inorder_tree();
        }
    }

    fn search_tree(&self, key: T ) -> Option<T> {
        if self.key == key {
            return Some(self.key.clone());
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
    fn minimum_tree(&self) -> T {
        match self.left {
            Some(ref tree) => tree.minimum_tree(),
            None => self.key.clone()
        }
    }

    fn maximum_tree(&self) -> T {
        match self.right {
            Some(ref tree) => tree.maximum_tree(),
            None => self.key.clone()
        }
    }

    fn insert_tree(&mut self, tree:SearchTree<T>) {

        if self.key <= tree.key {
            if let Some(ref mut left) = self.left {
                left.insert_tree(tree);
            } else {
                self.left = Some(Box::new(tree));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert_tree(tree);
            } else {
                self.right = Some(Box::new(tree));
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

    let mut root = SearchTree {
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
        key: 40,
    };


    root.inorder_tree();

    let tree = root.search_tree(43);
    match tree {
        Some(t) => println!("t = {:#?}", t),
        None => println!(" None "),

    }

    let min = root.minimum_tree();
    println!("min = {:#?}", min);

    let max = root.maximum_tree();
    println!("max = {:#?}", max);


    let mut will_insert_tree = SearchTree {
        left: None,
        right: None,
        key: 41,
    };

    root.insert_tree(will_insert_tree);

    println!("root = {:#?}", root);

    println!("root.search_tree(40) = {:#?}", root.search_tree(41));

}

