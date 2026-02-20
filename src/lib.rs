use std::fs::File;
mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {
            println!("Added to wait list");
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // Relative path
    front_of_house::hosting::add_to_wait_list();
}

pub fn open_greeting_file() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}")
    };
}

fn deliver_order() {
    println!("delivered order");
}

pub mod back_of_house {
    use crate::back_of_house;

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    pub fn cook_order() {
        println!("cook order");
    }

    pub fn eat_at_restaurant2() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
    }

    pub fn printvec() {
        let v = vec![1,2,3,4,5];

        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element"),
        }
    }

    pub fn loop_thru_vec() {
        let v = vec!(100, 32, 57);
        for i in &v {
            println!("{i}");
        }
    }

    pub fn mutvec() {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }

        for i in &v {
            println!("mut {i}")
        }
    }
}