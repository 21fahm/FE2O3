use std::collections::HashMap;

fn main() {
    let a: i32 = 10;
    let b: i32 = -15;

    // let c: u8 = 4;

    // let choice: bool = false;

    // let charcter: char = 'a';

    let arr: [u8; 4] = [1, 2, 3, 4];
    let other_array: [u8; 4] = [5; 4];

    println!("Hello, world!, {} {} {}", a, b, arr.len());

    println!("{:?}", arr);
    println!("{:?}", other_array);

    //tuple

    let tuple: (u8, bool, i8) = (10, true, -5);

    println!(
        "Here is the positive no.: {} here is the bool {} and here is the negative no.: {} ",
        tuple.0, tuple.1, tuple.2
    );

    //easier way

    let (a, b, c) = tuple; // destructuring
    println!(
        "Here is the positive no.: {} here is the bool {} and here is the negative no.: {} ",
        a, b, c
    );
    println!("{:?}", tuple);

    println!("{}", is_postive(8));

    //slice and array

    let arr: [u8; 4] = [1, 2, 3, 4];
    let slice = &arr[1..3];

    is_slice(arr, slice);

    //strings

    let some_string: &str = "Shawn Kimtai";
    let mut some_other_string: String = String::from("Shawn Kimtai 2.0 ");

    println!("{}", some_string);

    let string_slice = &some_string[..8];
    println!("{}", string_slice);

    some_other_string.push('a');
    some_other_string.push_str(" Mutated");

    println!("{}", some_other_string);

    //if statement
    let mut e: i32 = 0;
    if e > 0 {
        println!("Yeee");
    } else if e < 0 {
        println!("Neee");
    } else {
        println!("Just 0");
    }

    for i in arr {
        println!("{}", i);
    }
    println!("");
    while e < 10 {
        println!("{}", e);
        e += 1;
    }

    //Match -> This switch in other languages
    let v = 4;

    match v {
        0 => println!("Its zero"),
        1 | 2 => println!("Its 1 or 2!"),
        3..=4 => println!("Its from 3 to 4"),
        _ => println!("This is the deafult"),
    }

    //struct

    let bird = Animal {
        name: String::from("Shawn"),
        age: 100,
    };

    bird.print_name_age();

    println!("Name is: {} and his age is: {}", bird.name(), bird.age());

    //enums

    #[derive(Debug)]
    enum MyEnum {
        A,
        B(i32),
        C { x: i32, y: i32 },
    }

    let s: MyEnum = MyEnum::A;
    let h: MyEnum = MyEnum::B(5);
    let w: MyEnum = MyEnum::C { x: 32, y: 32 };

    println!("{:?}", s);
    println!("{:?}", h);
    println!("{:?}", w);

    if let MyEnum::B(value) = h {
        println!("{}", value);
    }
    if let MyEnum::C { x: val_x, y: val_y } = w {
        println!("{} {}", val_x, val_y);
    }

    //vec dynamic array
    let mut vec: Vec<i64> = vec![1, 2, 3, 4];
    vec.remove(vec.len() - 1);
    //also push!
    println!("{:?}", vec);

    //mapping
    let mut map = HashMap::new();

    map.insert(0, "Shawn");
    map.insert(1, "Kimtai");
    println!("{:?}", map);

    match map.get(&0) {
        Some(name) => println!("{}", name),
        _ => println!("Value does not exist"),
    }

    match map.get(&2) {
        Some(name2) => println!("{}", name2),
        _ => println!("Value does not exist"),
    }

    map.remove(&0);
    println!("{:?}", map);

    //options
    println!("{:?} {}", divide_num(32, 4), divide_num(32, 4).unwrap());

    //Error and Result
    println!("{:?}", divide_numtwo(8, 4).unwrap());
    println!("{:?}", divide_numtwo(8, 5).unwrap_or(100));
    let divide = divide_numtwo(12, 4);
    if divide.is_ok() {
        println!("{}", divide.unwrap());
    }
}

pub fn is_postive(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 //return type
}

pub fn is_slice(num: [u8; 4], slice: &[u8]) {
    println!("{:?}", slice);
    println!("{}", num[0]);
}
//options
fn divide_num(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}
//result and Error

#[derive(Debug)]

enum Error {
    ERROR1,
}
fn divide_numtwo(dividend: i32, divisor: i32) -> Result<i32, Error> {
    if dividend % divisor != 0 {
        Err(Error::ERROR1)
    } else {
        Ok(dividend / divisor)
    }
}

struct Animal {
    name: String,
    age: i32,
}

impl Animal {
    pub fn print_name_age(&self) {
        println!("{}", self.name);
        println!("{}", self.age);
    }
}

//trait-> Similar to interface

trait People {
    fn name(&self) -> &str;
    fn age(&self) -> i32 {
        21
    }
}

impl People for Animal {
    fn name(&self) -> &str {
        "Shawn"
    }
}
