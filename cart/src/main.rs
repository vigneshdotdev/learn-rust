use crate::sales::cart::Item;

pub mod sales;

fn main() {
    let item = Item {
        // item_id: 234234234,
        // item_name: String::from("Apple"),
        // price: 20.0,
        // quantity: 1.0
    };

    println!("Hello, world! -> finally i can able to understand about modules and submodules. Here is the item from cart which is a submodule of sales: {:?}",item);
}
