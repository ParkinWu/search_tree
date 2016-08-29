#[derive(Debug)]
struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
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
fn main() {
    let mut x = Counter::new();
    println!("x.next() = {:#?}", x.next());
    println!("x.next() = {:#?}", x.next());
    println!("x.next() = {:#?}", x.next());
    println!("x.next() = {:#?}", x.next());
    println!("x.next() = {:#?}", x.next());
    println!("x.next() = {:#?}", x.next());

}
