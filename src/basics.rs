// use std::slice::Iter;

use std::vec;

fn main() {
    // let mut v1 = 100;
    // v1 = v1 + 100;
    // let f1: f32 = 3.141;
    // let initial = 'T';
    // println!("{}", std::mem::size_of_val(&initial))
    // loops tutorial
    // for i in 1..=100 {
    //     println!("{i}");
    // }
    // let mut i = 0;
    // while i < 101 {
    //     println!("{}", i);
    //     i += 1;
    // }

    // let a = 2;
    // let b = 3;
    // if a > b {
    //     println!("a is greater than b")
    // } else if a >= b {
    //     println!("a is less than b")
    // } else {
    //     println!("Null")
    // }

    // loop {
    //     if i == 200 {
    //         break;
    //     }
    //     print!("{i}");
    //     i += 1;
    // }
    // Arrays, vectors and Strings
    // let mut i = 0;
    // let l: [i32; 5] = [2, 3, 4, 5, 4];
    // for i in l.into_iter() {
    // *i = *i + 100;
    //     print!("{i}");
    // }

    // println!("{}", l.contains(&100));
    // println!("{:#?}", l.as_ptr());
    // unsafe {
    //     let temp = std::ptr::read((l.as_ptr() as isize + 4 * 3 as isize) as *const u8);
    //     println!("{}", temp)
    // }
    //     println!("{i}");
    // }
    // Rust cannot declare a dynamic array
    //vectors do
    // let t = l.iter().map(|x| x + 200).collect::<Vec<i32>>();
    // println!("{:#?}", t)
    // declaring a vector
    let mut v1: Vec<i32> = vec![2, 3, 4, 5, 6];
    //we can push values in a vector like a stack
    //e,g
    // v1.push(20);
    // v1.push(100);
    // let temp = v1.pop().unwrap();

    //or
    // let v2: Vec<i32> = Vec::new();

    // let t = v1.iter().map(|x| x + 0).collect::<Vec<i32>>();
    // for i in v1.into_iter() {
    // *i += 100;
    //     println!("{}", &i);
    // }
    // for i in v1.clone().into_iter() {
    // *i += 100;
    //     println!("{}", &i);
    // }
    // println!("{:#?}", v1);
    // print!("Popped item: {temp}\n");
    //Strings

    let name = "Steven";
    /*for i in name.chars() {
        println!("{i}"); //Print multiple lines
        print!("{i}"); //Prints single line
    }*/
    let mut myname: String = name.to_string();

    //to push str to existing string

    myname.push_str(" string");
    print!("{myname}");
}
