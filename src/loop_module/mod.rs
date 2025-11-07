
#![allow(dead_code)]
pub fn loop_fn1() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");  
}

pub fn loop_fn2() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!!");
}

pub fn loop_fn3() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }
}

pub fn loop_fn4() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is {}", {element});
    }
}

pub fn loop_fn5() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}