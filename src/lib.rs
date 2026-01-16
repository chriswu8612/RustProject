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
}