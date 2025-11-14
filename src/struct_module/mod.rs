#![allow(dead_code)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

pub fn struct_fn1() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    user1.email = String::from("anotheremail@example.com");
    println!("user1 email is {0}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user2 email is {0}, username={1}", user2.email, user2.username);

}

pub fn struct_fn2() {
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area(rect1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn rect_fn() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}