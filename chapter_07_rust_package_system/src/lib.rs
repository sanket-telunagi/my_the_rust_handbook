mod front_of_the_house;
use front_of_the_house::front_of_the_house::serve_order;
// mod back_of_the_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::front_of_the_house::serve_order();
//     }

//     fn cook_order() {}
// }
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_the_house::serve_order();

//     // Relative path
//     front_of_the_house::serve_order();
// }

mod back_of_the_house {
    front_of_the_house::serve_order();
}
