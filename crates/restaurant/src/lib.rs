mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist;

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    add_to_waitlist();

    // 点一份夏季早餐，黑麦吐司
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意，想要小麦吐司
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 下面这行如果取消注释将无法编译；我们不允许
    // 查看或修改餐食附带的时令水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}
