
use crate::commands::*;

pub fn week2_category() {
    let command_handler = CommandHandler::new("command", [
        Command::new("type inference", week2_type_inference),
        Command::new("underline", week2_underline),
        Command::new("const", week2_const),
        Command::new("static", week2_static),
        Command::new("scope_shadowing", week2_scope_shadowing),
        Command::new("shadowing", week2_shadowing),
        Command::new("memory_layout", week2_memory_layout),
        Command::new("ownership", week2_ownership),
        Command::new("move", week2_move),
        Command::new("move with function", week2_move_with_function),
        Command::new("copy", week2_copy),
        Command::new("clone", week2_clone),
        Command::new("borrow", week2_borrow),
        Command::new("share", week2_share),
        Command::new("lifetime_with_function", week2_lifetime_with_function),
        Command::new("lifetime_with_struct", week2_lifetime_with_struct),
        Command::new("library", week2_library)
    ].into_iter());

    command_handler.handle();
}

fn week2_type_inference() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn week2_underline() {
    let mut v = Vec::new();
    v.push((10, false));
    v.push((20, true));
    println!("v: {v:?}");

    let vv = v.iter().collect::<std::collections::HashSet<_>>();
    println!("vv: {vv:?}");
}

const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn week2_const() {
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    return digest;
}

static BANNER: &str = "Welcome to RustOS 3.14";

fn week2_static() {
    println!("{BANNER}");
}

fn week2_scope_shadowing() {
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}

fn week2_shadowing() {
    let a = 1;
    let b = &a;
    let a = a + 1;
    println!("{a} {b}");
}

fn week2_memory_layout() {    
    let mut s1 = String::from("Hello");
    s1.push(' ');
    s1.push_str("world");
    // DON'T DO THIS AT HOME! For educational purposes only.
    // String provides no guarantees about its layout, so this could lead to
    // undefined behavior.
    unsafe {
        let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("ptr = {ptr:#x}, len = {len}, capacity = {capacity}");
    }    
}

struct Point(i32, i32);

fn week2_ownership() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    // println!("y: {}", p.1);
}

fn week2_move() {
    let s1: String = String::from("Hello!");
    let s2 = s1;
    println!("s2: {s2}");
}

fn week2_move_with_function() {
    let name = String::from("Alice");
    say_hello(name);
}

fn say_hello(name: String) {
    println!("Hello {name}");
}

#[derive(Copy, Clone, Debug)]
struct CopyPoint(i32, i32);

fn week2_copy() {
    let p1 = CopyPoint(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}

#[derive(Clone, Debug)]
struct ClonePoint(i32, i32, String);

fn week2_clone() {
    let p1 = ClonePoint(3, 4, String::from("string"));
    let p2 = p1.clone();
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}

#[derive(Debug)]
struct DebugPoint(i32, i32);

fn add(p1: &DebugPoint, p2: &DebugPoint) -> DebugPoint {
    DebugPoint(p1.0 + p2.0, p1.1 + p2.1)
}

fn week2_borrow() {
    let p1 = DebugPoint(3, 4);
    let p2 = DebugPoint(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");
}

fn week2_share() {
    let mut a: i32 = 10;
    let b: &i32 = &a;

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    // println!("b: {b}");
}

fn left_most<'a>(p1: &'a DebugPoint, p2: &'a DebugPoint) -> &'a DebugPoint {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn week2_lifetime_with_function() {
    let p1 = DebugPoint(10, 10);
    let p2 = DebugPoint(20, 20);
    let p3: &DebugPoint = left_most(&p1, &p2);
    println!("left-most point: {:?}", p3);
}

#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
    println!("Bye {text}!");
}

fn week2_lifetime_with_struct() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    // erase(text);
    println!("{fox:?}");
    println!("{dog:?}");
}

struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Update the `self` parameter to
// indicate the method's required level of ownership over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    fn new() -> Library {
        Library{ books: Vec::new() }
    }

    fn len(&self) -> usize {
       self.books.len()
    }

    fn is_empty(&self) -> bool {
       self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
       self.books.push(book)
    }

    fn print_books(&self) {
       for book in &self.books {
        println!("{} {}", book.title, book.year)
       }
    }

    fn oldest_book(&self) -> Option<&Book> {
        let mut oldest_book: Option<&Book> = None;

        for book in &self.books {
            if oldest_book.is_none() || book.year < oldest_book.unwrap().year {
                oldest_book = Some(book);
            }
        }

        oldest_book
    }
}

// This shows the desired behavior. Uncomment the code below and
// implement the missing methods. You will need to update the
// method signatures, including the "self" parameter! You may
// also need to update the variable bindings within main.
fn week2_library() {
    let mut library = Library::new();

    println!("The library is empty: library.is_empty() -> {}", library.is_empty());
    
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    
    println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());
    
    
    library.print_books();
    
    match library.oldest_book() {
       Some(book) => println!("The oldest book is {}", book.title),
       None => println!("The library is empty!"),
    }
    
    println!("The library has {} books", library.len());
    library.print_books();
}