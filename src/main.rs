use std::boxed::Box;

#[derive(Debug)]
struct SearchTree<'a, T: 'a> {
    left: Option<Box<&'a mut SearchTree<'a,T>>>,
    right: Option<Box<&'a mut SearchTree<'a,T>>>,
    key: T,
}

impl<'a, T> SearchTree<'a, T> where T: std::fmt::Debug + std::cmp::Eq + std::cmp::PartialOrd {
    fn inorder_tree(&self) {
        if let Some(ref tree) = self.left {
            tree.inorder_tree();
        }
        println!("self.key = {:#?}", self.key);
        if let Some(ref tree) = self.right {
            tree.inorder_tree();
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

    fn maximum_tree(&'a self) -> &'a SearchTree<'a, T> {
        match self.right {
            Some(ref tree) => tree.maximum_tree(),
            None => &self
        }
    }

    fn insert_tree(&'a mut self, tree:&'a SearchTree<'a, T>) {

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
        left: Some(Box::new(&left)),
        right: Some(Box::new(&right)),
        key: 40,
    };

    root.inorder_tree();

    let tree = root.search_tree(43);
    match tree {
        Some(t) => println!("t.key = {:#?}", t.key),
        None => println!(" None "),

    }

    let min = root.minimum_tree();
    println!("min = {:#?}", min);

    let max = root.maximum_tree();
    println!("max = {:#?}", max);


    let will_insert_tree = SearchTree {
        left: None,
        right: None,
        key: 41,
    };

    root.insert_tree(&will_insert_tree);

    println!("root = {:#?}", root);

}

