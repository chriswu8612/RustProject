use RustProject::{back_of_house::{eat_at_restaurant2, fix_incorrect_order}, eat_at_restaurant};

// next: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
fn main() {
   eat_at_restaurant();
   eat_at_restaurant2();
   fix_incorrect_order();
}



