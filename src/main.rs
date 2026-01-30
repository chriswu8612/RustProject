// use RustProject::{back_of_house::{eat_at_restaurant2, fix_incorrect_order}, eat_at_restaurant};
mod pay;
use pay::pay_fn;

// next: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
fn main() {
   pay_fn();
}



