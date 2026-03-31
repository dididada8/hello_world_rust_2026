#[allow(dead_code)]
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

pub enum Appetizer {
    Soup,
    Salad,
}

#[allow(dead_code)]
fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}

#[allow(dead_code)]
fn deliver_order() {}

#[allow(dead_code)]
fn cook_order() {}
