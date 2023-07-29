

use crate::commands::*;

pub fn week3_category() {
    let command_handler = CommandHandler::new("command", [
        Command::new("struct", _struct),
        Command::new("tuple", tuple),
        Command::new("newtype", newtype),
        Command::new("field_shorthand", field_shorthand),
        Command::new("default", default),
        Command::new("enum", _enum),
        Command::new("variant payload", variant_payload),
        Command::new("size of enum", size_of_enum),
        Command::new("size of enum with repr", size_of_enum_with_repr),
        Command::new("bits of num", bits_of_enum),
        Command::new("many options", many_options),
        Command::new("method", method),
        Command::new("method receiver", method_receiver),
        Command::new("pattern matching", pattern_matching),
        Command::new("destructure enum", destructure_enum),
        Command::new("destructure struct", destructure_struct),
        Command::new("destructure array", destructure_array),
        Command::new("destructure slice", destructure_slice),
        Command::new("match guard", match_guard),
        Command::new("health report", health_report),
    ].into_iter());

    command_handler.handle();
}

struct Person {
    name: String,
    age: u8,
}

struct NoMember;

fn _struct() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{} is {} years old", peter.name, peter.age);
    
    peter.age = 28;
    println!("{} is {} years old", peter.name, peter.age);
    
    let jackie = Person {
        name: String::from("Jackie"),
        ..peter
    };
    println!("{} is {} years old", jackie.name, jackie.age);
}

struct Point(i32, i32);

fn tuple() {
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
}

struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    todo!("Ask a rocket scientist at NASA")
}

fn set_thruster_force(force: Newtons) {
    // ...
}

fn newtype() {
    let force = compute_thruster_force();
    // set_thruster_force(force);
}

#[derive(Debug)]
struct ShortPerson {
    name: String,
    age: u8,
}

impl ShortPerson {
    fn new(name: String, age: u8) -> ShortPerson {
        ShortPerson { name, age }
    }
}

fn field_shorthand() {
    let peter = ShortPerson::new(String::from("Peter"), 27);
    println!("{peter:?}");
}

#[derive(Debug)]
struct DefaultPerson {
    name: String,
    age: u8,
}
impl Default for DefaultPerson {
    fn default() -> Self {
        Self {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}
fn default() {
    let tmp = DefaultPerson {
        ..DefaultPerson::default()
    };
    let tmp = DefaultPerson {
        name: "Sam".to_string(),
        ..DefaultPerson::default()
    };

    println!("{tmp:?}");
}

fn generate_random_number() -> i32 {
    // Implementation based on https://xkcd.com/221/
    4  // Chosen by fair dice roll. Guaranteed to be random.
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = generate_random_number();
    if random_number % 2 == 0 {
        return CoinFlip::Heads;
    } else {
        return CoinFlip::Tails;
    }
}

fn _enum() {
    println!("You got: {:?}", flip_coin());
}

enum WebEvent {
    PageLoad,                 // 페이로드가 없는 유형
    KeyPress(char),           // 튜플 구조체 유형
    Click { x: i64, y: i64 }, // 완전한 구조체 유형
}

#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad       => println!("page loaded"),
        WebEvent::KeyPress(c)    => println!("pressed '{c}'"),
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

fn variant_payload() {
    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(load);
    inspect(press);
    inspect(click);
}

use std::any::type_name;
use std::mem::{align_of, size_of};

fn dbg_size<T>() {
    println!("{}: size {} bytes, align: {} bytes",
        type_name::<T>(), size_of::<T>(), align_of::<T>());
}

enum Foo {
    A,
    B,
}

fn size_of_enum() {
    dbg_size::<Foo>();
}

#[repr(u32)]
enum Bar {
    A,  // 0
    B = 10000,
    C,  // 10001
}

fn size_of_enum_with_repr() {
    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);
    dbg_size::<Bar>();
}

use std::mem::transmute;

macro_rules! dbg_bits {
    ($e:expr, $bit_type:ty) => {
        println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
    };
}

fn bits_of_enum() {
    // TOTALLY UNSAFE. Rust provides no guarantees about the bitwise
    // representation of types.
    unsafe {
        println!("Bitwise representation of bool");
        dbg_bits!(false, u8);
        dbg_bits!(true, u8);

        println!("Bitwise representation of Option<bool>");
        dbg_bits!(None::<bool>, u8);
        dbg_bits!(Some(false), u8);
        dbg_bits!(Some(true), u8);

        println!("Bitwise representation of Option<Option<bool>>");
        dbg_bits!(Some(Some(false)), u8);
        dbg_bits!(Some(Some(true)), u8);
        dbg_bits!(Some(None::<bool>), u8);
        dbg_bits!(None::<Option<bool>>, u8);

        println!("Bitwise representation of Option<&i32>");
        dbg_bits!(None::<&i32>, usize);
        dbg_bits!(Some(&0i32), usize);
        dbg_bits!(Some(&0), usize);

        println!("Bitwise representation of Option<i32>");
        dbg_bits!(None::<i32>, usize);
        dbg_bits!(Some(32), usize);
        dbg_bits!(Some(0), usize);

        println!("Bitwise representation of i32");
        dbg_bits!(32, i32);
        dbg_bits!(0, i32);
    }
}



// use std::mem::transmute;

// macro_rules! dbg_bits {
//     ($e:expr, $bit_type:ty) => {
//         println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
//     };
// }

// Macro to wrap a value in 2^n Some() where n is the number of "@" signs.
// Increasing the recursion limit is required to evaluate this macro.
macro_rules! many_options {
    ($value:expr) => { Some($value) };
    ($value:expr, @) => {
        Some(Some($value))
    };
    ($value:expr, @ $($more:tt)+) => {
        many_options!(many_options!($value, $($more)+), $($more)+)
    };
}

fn many_options() {
    // TOTALLY UNSAFE. Rust provides no guarantees about the bitwise
    // representation of types.
    unsafe {
        assert_eq!(many_options!(false), Some(false));
        assert_eq!(many_options!(false, @), Some(Some(false)));
        assert_eq!(many_options!(false, @@), Some(Some(Some(Some(false)))));

        println!("Bitwise representation of a chain of 128 Option's.");
        dbg_bits!(many_options!(false, @@@@@@@), u8);
        dbg_bits!(many_options!(true, @@@@@@@), u8);

        println!("Bitwise representation of a chain of 256 Option's.");
        dbg_bits!(many_options!(false, @@@@@@@@), u16);
        dbg_bits!(many_options!(true, @@@@@@@@), u16);

        println!("Bitwise representation of a chain of 257 Option's.");
        dbg_bits!(many_options!(Some(false), @@@@@@@@), u16);
        dbg_bits!(many_options!(Some(true), @@@@@@@@), u16);
        dbg_bits!(many_options!(None::<bool>, @@@@@@@@), u16);
    }
}

#[derive(Debug)]
struct MethodPerson {
    name: String,
    age: u8,
}

impl MethodPerson {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn method() {
    let peter = MethodPerson {
        name: String::from("Peter"),
        age: 27,
    };
    peter.say_hello();
}

#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Race {  // No receiver, a static method
        Race { name: String::from(name), laps: Vec::new() }
    }

    fn add_lap(&mut self, lap: i32) {  // Exclusive borrowed read-write access to self
        self.laps.push(lap);
    }

    fn print_laps(&self) {  // Shared and read-only borrowed access to self
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {  // Exclusive ownership of self
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn method_receiver() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42);
}

fn pattern_matching() {
    let input = 'x';

    match input {
        'q'                   => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9'             => println!("Number input"),
        _                     => println!("Something else"),
    }
}

enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

fn destructure_enum() {
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}

struct StructFoo {
    x: (u32, u32),
    y: u32,
}

#[rustfmt::skip]
fn destructure_struct() {
    let foo = StructFoo { x: (1, 2), y: 3 };
    match foo {
        StructFoo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        StructFoo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        StructFoo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
}

#[rustfmt::skip]
fn destructure_array() {
    let triple = [0, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..]   => println!("First is 1 and the rest were ignored"),
        _         => println!("All elements were ignored"),
    }
}

fn destructure_slice() {
    inspect_slice(&[0, -2, 3]);
    inspect_slice(&[0, -2, 3, 4]);
}

#[rustfmt::skip]
fn inspect_slice(slice: &[i32]) {
    println!("Tell me about {slice:?}");
    match slice {
        &[0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        &[1, ..]   => println!("First is 1 and the rest were ignored"),
        _          => println!("All elements were ignored"),
    }
}

#[rustfmt::skip]
fn match_guard() {
    let pair = (2, -2);
    println!("Tell me about {pair:?}");
    match pair {
        (x, y) if x == y     => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _                    => println!("No correlation..."),
    }
}

// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self {
            name,
            age,
            height,
            visit_count: 0,
            last_blood_pressure: None
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn doctor_visits(&self) -> u32 {
        self.visit_count.try_into().unwrap()
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age
    }

    pub fn set_height(&mut self, new_height: f32) {
        self.height = new_height
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        self.visit_count += 1;

        let blood_pressure_change = match self.last_blood_pressure {
            None => None,
            Some(last_blood_pressure) => {
                if measurements.blood_pressure == last_blood_pressure {
                    None
                } else {
                    Some((measurements.blood_pressure.0 as i32 - last_blood_pressure.0 as i32, measurements.blood_pressure.1 as i32 - last_blood_pressure.1 as i32))
                }
            }
        };

        self.last_blood_pressure = Some(measurements.blood_pressure);

        let health_report = HealthReport {
            patient_name: self.name(),
            visit_count: self.doctor_visits(),
            height_change: measurements.height - self.height(),
            blood_pressure_change
        };

        return health_report;
    }
}

fn health_report() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
}

#[test]
fn test_height() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.height(), 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age(), 32);
    bob.set_age(33);
    assert_eq!(bob.age(), 33);
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.doctor_visits(), 0);
    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (120, 80),
    });
    assert_eq!(report.patient_name, "Bob");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);

    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (115, 76),
    });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
}