// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


mod front_office {
    pub mod add_hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table(){
            super::serving::take_order();
        }
    }

    pub mod serving {
        pub fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}


fn eat_at_restaurant(){
    // absolute path
    crate::front_office::add_hosting::add_to_waitlist();

    //relative path
    front_office::add_hosting::add_to_waitlist();
}
