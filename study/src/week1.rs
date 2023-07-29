// mod commands;
use crate::commands::*;

pub fn week1_category() {
    let command_handler = CommandHandler::new("command", [
        Command::new("hello world", week1_hello_wolrd),
        Command::new("small_example", week1_small_example),
        Command::new("raw_string", week1_raw_string),
        Command::new("byte_string", week1_byte_string),
        Command::new("array", week1_array),
        Command::new("tuple", week1_tuple),
        Command::new("reference", week1_reference),
        Command::new("slice", week1_slice),
        Command::new("string", week1_string),
        Command::new("function", week1_function),
        Command::new("method", week1_method),
        Command::new("overloading", week1_overloading as fn()),
        Command::new("convert", week1_convert),
        Command::new("array_for", week1_array_for),
        Command::new("matrix", week1_matrix),
    ].into_iter());

    command_handler.handle();

    // handle_command("command", commands);
}

fn week1_hello_wolrd()
{
    println!("Hello 🌍!");
}

fn week1_small_example() {
    let mut x: i32 = 6;  // 가변 변수 할당(binding)입니다.
    print!("{x}");       // printf와 같은 출력을 위한 매크로 입니다.
    while x != 1 {       // 표현식에 괄호는 없습니다.
        if x % 2 == 0 {  // 다른 언어와 같은 수학연산식이 사용됩니다.
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();
}

fn week1_raw_string() {
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}

fn week1_byte_string() {
    println!("{:?}", b"abc");
    println!("{:?}", &[97, 98, 99]);
}

fn week1_array() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a);
}

fn week1_tuple() {
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);
}

fn week1_reference() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");
}

// fn week1_dangling_reference() {
//     let ref_x: &i32;
//     {
//         let x: i32 = 10;
//         ref_x = &x;
//     }
//     println!("ref_x: {ref_x}");
// }

fn week1_slice() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
}

fn week1_string() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
    
    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}

fn week1_function() {
    print_fizzbuzz_to(20);
}

fn is_divisible(n: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }
    n % divisor == 0
}

fn fizzbuzz(n: u32) -> String {
    let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
    let buzz = if is_divisible(n, 5) { "buzz" } else { "" };
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }
    format!("{fizz}{buzz}")
}

fn print_fizzbuzz_to(n: u32) {
    for i in 1..=n {
        println!("{}", fizzbuzz(i));
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn week1_method() {
    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
}

fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}

fn week1_overloading() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn week1_convert() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));
}

fn week1_array_for() {
    let array = [10, 20, 30];
    print!("Iterating over array:");
    for n in array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            transposed[j][i] = matrix[i][j];
        }
    }

    return transposed;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            print!(" {}", matrix[i][j]);
        }

        println!();
    }

}

fn week1_matrix() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}