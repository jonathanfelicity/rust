mod var {
    pub struct About {
        pub is_good: bool,
        pub name: String,
    }
}

use var::About;

fn main() {
    let a = About {
        is_good: true,
        name: "John".to_string(), // Replace "John" with the desired name.
    };
    println!("my name is {}", a.name);
}
