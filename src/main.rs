use std::ptr::null_mut;
struct Node<T> {
    val: T,
    next: *mut Box<Node<T>>
}

struct LinkedList<T> {
    head: *mut Box<Node<T>>,
    tail: *mut Box<Node<T>>,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        return LinkedList {
            head: null_mut(),
            tail: null_mut(),
        };

    }
//    fn append(&self, val: T) {
//        let
//    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
impl Person {
    fn new() -> Person {
        return Person {
            name: "pzwu".to_string(),
            age: 25,
        };
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("drop self.name = {:#?}", self.name);
    }

}
fn main() {
    {
        let mut p: Box<Person> = Box::new(Person {
            name: "1".to_string(),
            age: 199,
        });


        let mut raw_p = &mut p as *mut Box<Person>;
        //    let ref_p = &p;
        //    let raw_p_2_ref = &mut unsafe {*raw_p};
        unsafe {
            *raw_p = Box::new(Person {
                name: "2".to_string(),
                age: 200,
            });

            println!("raw_p = {:#?}", *raw_p);

            let mut raw_p_2_ref = &mut *raw_p;
            let mut raw_p_2_ref1 = &mut *raw_p;


            let mut_borrow_p = &mut p;

            println!("mut_borrow_p = {:#?}", mut_borrow_p);
            println!("raw_p_2_ref = {:#?}", raw_p_2_ref);
            println!("raw_p_2_ref1 = {:#?}", raw_p_2_ref1);

            *raw_p_2_ref = Box::new(Person {
                name: "3".to_string(),
                age: 300,
            });
        }

        let mut raw_move_p = null_mut();

        {
            let mut move_p = p;
            raw_move_p = &mut move_p as *mut Box<Person>
        }
        //    *raw_p = Box::new(20);

        for i in 0..2 {
            let temp = Box::new(Person {
                name: "temp".to_string(),
                age: i,
            });
        }
//        let mut_borrow_p = &mut p;
        unsafe {
            println!("raw_move_p = {:#?}", *raw_move_p);
        }



//        println!("mut_borrow_p = {:#?}", mut_borrow_p);
        println!("inner end");
    }

    println!("end!");


}