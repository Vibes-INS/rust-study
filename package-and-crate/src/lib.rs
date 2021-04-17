pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }

        fn seat_at_table() {
            println!("seat_at_table");
        }
    }

    mod serving {
        fn take_order() {
            println!("take_order")
        }
        fn server_order() {
            println!("server_order")
        }
        fn take_payment() {
            println!("take_payment")
        }
    }
}

mod back_of_hose {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_hose::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
