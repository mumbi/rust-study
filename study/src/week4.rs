
use crate::commands::*;

pub fn week4_category() {
    let command_handler = CommandHandler::new("command", [
        Command::new("block", block),
        Command::new("function is kind of a block", function_is_kind_of_a_block),
        Command::new("if expression", if_expression),
        Command::new("if expression value", if_expression_value),
        Command::new("if let", if_let),
        Command::new("let else", let_else),
        Command::new("while loop", while_loop),
        Command::new("while let", while_let),
        Command::new("for loop", for_loop),
        Command::new("loop", _loop),
        Command::new("match", _match),
        Command::new("break continue", break_continue),
        Command::new("option result", option_result),
        Command::new("string", string),
        Command::new("vec", vec),
        Command::new("hashmap", hashmap),
        Command::new("box", _box),
        Command::new("box list", box_list),
        Command::new("niche optimization", niche_optimization),
        Command::new("rc", rc),
        Command::new("refcell", refcell),
        Command::new("module", module),
        Command::new("module visibility", module_visibility),
    ].into_iter());

    command_handler.handle();
}

fn block() {
    let x = {
        let y = 10;
        println!("y: {y}");
        let z = {
            let w = {
                3 + 4
            };
            println!("w: {w}");
            y * w
        };
        println!("z: {z}");
        z - y
    };
    println!("x: {x}");
}

fn double(x: i32) -> i32 {
    x + x
}

fn function_is_kind_of_a_block() {
    println!("doubled: {}", double(7));
}

fn if_expression() {
    let mut x = 10;
    if x % 2 == 0 {
        x = x / 2;
    } else {
        x = 3 * x + 1;
    }
}

fn if_expression_value() {
    let mut x = 10;
    x = if x % 2 == 0 {
        x / 2
    } else {
        3 * x + 1
    };
}

fn if_let() {
    let arg = std::env::args().next();
    if let Some(value) = arg {
        println!("Program name: {value}");
    } else {
        println!("Missing name?");
    }
}

fn let_else() {
    println!("{:?}", second_word_to_upper("foo bar"));
}
 
fn second_word_to_upper(s: &str) -> Option<String> {
    let mut it = s.split(' ');
    let (Some(_), Some(item)) = (it.next(), it.next()) else {
        return None;
    };
    Some(item.to_uppercase())
}

fn while_loop() {
    let mut x = 10;
    while x != 1 {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
    }
    println!("Final x: {x}");
}

fn while_let() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
}

fn for_loop() {
    let v = vec![10, 20, 30];

    for x in v {
        println!("x: {x}");
    }
    
    for i in (0..10).step_by(2) {
        println!("i: {i}");
    }
}

fn _loop() {
    let mut x = 10;
    loop {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
        if x == 1 {
            break;
        }
    }
    println!("Final x: {x}");
}

fn _match() {
    match std::env::args().next().as_deref() {
        Some("cat") => println!("Will do cat things"),
        Some("ls")  => println!("Will ls some files"),
        Some("mv")  => println!("Let's move some files"),
        Some("rm")  => println!("Uh, dangerous!"),
        None        => println!("Hmm, no program name?"),
        _           => println!("Unknown program name!"),
    }
}

fn break_continue() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    'outer: while let Some(x) = iter.next() {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}

fn option_result() {
    let numbers = vec![10, 20, 30];
    let first: Option<&i8> = numbers.first();
    println!("first: {first:?}");

    let idx: Result<usize, usize> = numbers.binary_search(&10);
    println!("idx: {idx:?}");
}

fn string() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    println!("s3: len = {}, number of chars = {}, {:?}", s3.len(),
             s3.chars().count(), s3.chars());
}

fn vec() {
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());

    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());

    // 요소로 벡터를 초기화하는 표준 매크로입니다.
    let mut v3 = vec![0, 0, 1, 2, 3, 4];

    // 짝수 요소만 유지합니다.
    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");

    // 연속 중복을 삭제합니다.
    v3.dedup();
    println!("{v3:?}");
}

use std::collections::HashMap;

fn hashmap() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);

    if !page_counts.contains_key("Les Misérables") {
        println!("We know about {} books, but not Les Misérables.",
                 page_counts.len());
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown.")
        }
    }

    // 아무것도 찾을 수 없는 경우 .entry() 메서드를 사용하여 값을 삽입합니다.
    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        let page_count: &mut i32 = page_counts.entry(book.to_string()).or_insert(0);
        *page_count += 1;
    }

    println!("{page_counts:#?}");
}

fn _box() {
    let five = Box::new(5);
    println!("five: {}", *five);
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn box_list() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}

// #[derive(Debug)]
// enum List<T> {
//     Cons(T, Box<List<T>>),
//     Nil,
// }

fn niche_optimization() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}

use std::rc::Rc;

fn rc() {
    let mut a = Rc::new(10);
    let mut b = Rc::clone(&a);

    println!("a: {a}");
    println!("b: {b}");
}

use std::rc::{Weak};
use std::cell::{RefCell, Cell};

#[derive(Debug)]
struct Node {
    value: i64,
    parent: Option<Weak<RefCell<Node>>>,
    // parent: Option<Weak<Node>>,
    // parent: Option<Weak<Cell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
    // children: Vec<Rc<Node>>
    // children: Vec<Rc<Cell<Node>>>,
}

fn refcell() {
    let mut root = Rc::new(RefCell::new(Node {
        value: 42,
        parent: None,
        children: vec![],
    }));
    let child = Rc::new(RefCell::new(Node {
        value: 43,
        children: vec![],
        parent: Some(Rc::downgrade(&root)),
    }));
    // root.borrow_mut().children.push(child.clone());
    // root.borrow_mut().children.push(child.clone());
    let mut mut1 = root.borrow_mut();
    let mut mut2 = root.borrow_mut();    
    mut1.children.push(child.clone());
    mut2.children.push(child.clone());
    //root.children.push(child);
    // let mut root = Rc::new(RefCell::new(Node {
    //     value: 42,
    //     parent: None,
    //     children: vec![],
    // }));
    // let child = Rc::new(RefCell::new(Node {
    //     value: 43,
    //     children: vec![],
    //     parent: Some(Rc::downgrade(&root))
    // }));
    // root.borrow_mut().children.push(child);

    // println!("graph: {root:#?}");
}

// mod foo {
//     pub fn do_something() {
//         println!("In the foo module");
//     }
// }
mod foo;

mod bar {
    pub fn do_something() {
        println!("In the bar module");
    }
}

fn module() {
    foo::do_something();
    bar::do_something();
}

mod outer {
    fn private() {
        println!("outer::private");
    }

    pub fn public() {
        println!("outer::public");
    }

    mod inner {
        fn private() {
            println!("outer::inner::private");
        }

        pub fn public() {
            println!("outer::inner::public");
            super::private();
        }
    }
}

fn module_visibility() {
    outer::public();
}