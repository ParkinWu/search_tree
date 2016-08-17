use std::boxed::Box;

#[derive(Debug)]
struct SearchTree<T> {
    left: Option<Box<SearchTree<T>>>,
    right: Option<Box<SearchTree<T>>>,
    key: T,
}

impl<T> SearchTree<T> where T: std::fmt::Debug + std::cmp::Eq + std::cmp::PartialOrd + std::clone::Clone {
    fn inorder(&self) {
        if let Some(ref tree) = self.left {
            tree.inorder();
        }
        println!("self.key = {:#?}", self.key);
        if let Some(ref tree) = self.right {
            tree.inorder();
        }
    }

    fn search(&self, key: T ) -> Option<T> {
        if self.key == key {
            return Some(self.key.clone());
        }

        if key < self.key {
            match self.left {
                Some(ref tree) => tree.search(key),
                None => None
            }
        } else {
            match self.right {
                Some(ref tree) => tree.search(key),
                None => None
            }
        }
    }
    fn minimum(&self) -> T {
        match self.left {
            Some(ref tree) => tree.minimum(),
            None => self.key.clone()
        }
    }

    fn maximum(&self) -> T {
        match self.right {
            Some(ref tree) => tree.maximum(),
            None => self.key.clone()
        }
    }

    fn insert(&mut self, tree:SearchTree<T>) {

        if self.key >= tree.key {
            if let Some(ref mut left) = self.left {
                left.insert(tree);
            } else {
                self.left = Some(Box::new(tree));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(tree);
            } else {
                self.right = Some(Box::new(tree));
            }
        }
    }

    fn delete(&mut self, key: T) {

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


    root.inorder();

    let tree = root.search(43);
    match tree {
        Some(t) => println!("t = {:#?}", t),
        None => println!(" None "),

    }

    let min = root.minimum();
    println!("min = {:#?}", min);

    let max = root.maximum();
    println!("max = {:#?}", max);


    let mut will_insert_tree = SearchTree {
        left: None,
        right: None,
        key: 41,
    };

    root.insert(will_insert_tree);

    println!("root = {:#?}", root);

    println!("root.search_tree(41) = {:#?}", root.search(41));

}

