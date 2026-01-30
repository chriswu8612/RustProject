#[derive(Debug)]
enum PaymentMethod {
    Cash,
    CreditCard,
    Crypto(String),
}

pub fn pay_fn() {
    let method1 = PaymentMethod::Cash;
    let method2 = PaymentMethod::CreditCard;
    let method3 = PaymentMethod::Crypto("USDC".to_string());

    println!("{:?}", method1);
    println!("{:?}", method2);
    println!("{:?}", method3);
}

pub fn opt() {
   let some_number = Some(5);
   let some_char = Some('e');

   let absent_number: Option<i32> = None;
   println!("Some number: {:?}", some_number);
   println!("Some char: {:?}", some_char);
   println!("Absent number: {:?}", absent_number);

}