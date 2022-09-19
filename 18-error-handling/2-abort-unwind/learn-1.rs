// rustc learn-1.rs -C panic=abort
fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {panic!("This is not your party. Run!!!!");}
        else {println!("Some refreshing {} is all I need.", beverage);}
    }
    else { println!("Some refreshing {} is all I need.", beverage); }
}

fn main() {
    drink("water");
    drink("lemonade");
}
