mod front_of_house;

pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant3() {
    hosting::add_to_waitlist();
}

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // meal.seasonal_fruit = String::from("blueberries")
// }
