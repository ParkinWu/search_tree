use std::boxed::Box;
use std::collections::HashMap;
extern crate time;



#[derive(Debug)]
struct SearchTree<T> {
    left: Option<Box<SearchTree<T>>>,
    right: Option<Box<SearchTree<T>>>,
    key: T,
}

impl<T> SearchTree<T> where T: std::fmt::Debug + std::cmp::Eq + std::cmp::PartialOrd + std::clone::Clone {
    /// 中序遍历节点,只是打印
    fn inorder(&self) {
        if let Some(ref tree) = self.left {
            tree.inorder();
        }
        println!("self.key = {:#?}", self.key);
        if let Some(ref tree) = self.right {
            tree.inorder();
        }
    }

    /// 查找节点
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

    /// 取出最小节点的 key
    fn minimum(&self) -> T {
        match self.left {
            Some(ref tree) => tree.minimum(),
            None => self.key.clone()
        }
    }

    /// 取出最大节点的 key
    fn maximum(&self) -> T {
        match self.right {
            Some(ref tree) => tree.maximum(),
            None => self.key.clone()
        }
    }
    /// 插入一个节点
    fn insert(&mut self, val: T) {
        if self.key >= val {
            if let Some(ref mut left) = self.left {
                left.insert(val);
            } else {
                self.left = Some(Box::new(SearchTree {
                    left: None,
                    right: None,
                    key: val,

                }));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(val);
            } else {
                self.right = Some(Box::new(SearchTree {
                    left: None,
                    right: None,
                    key: val,

                }));
            }
        }
    }


    /// 删除一个节点
    fn delete(&mut self, key: T) {

    }

}

fn main() {
    let mut root = SearchTree {
        left: None,
        right: None,
        key: 10000001,
    };
    let mut hash_map = HashMap::new();
    for i in 1000..100000000 {
        hash_map.insert(i, i);
//        root.insert(i);
    }
    println!("insert end");
    let mut start = timestamp();
    let result = hash_map.keys().filter(|&&x| (x >= 1000000 - 1)).collect::<Vec<_>>();
    let mut end = timestamp();
    println!("end - start = {:#?}", end - start);

    start = timestamp();
    let max = root.maximum();
    end = timestamp();
    println!("end - start = {:#?}", end - start);
    println!("max = {:#?}", max);
}

fn timestamp() -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0 );
    mills

}



