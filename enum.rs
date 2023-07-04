#[allow(dead_code)]
enum Fruit {
    Mango,
    Banana,
    Orange,
}

fn main() {
    let fruit = Fruit::Mango;
    println!("The color of the fruit is: {}", fruit_color(fruit));
}

fn fruit_color(fruit: Fruit) -> &'static str {
    match fruit {
        Fruit::Mango => "Yellow",
        Fruit::Banana => "Green",
        Fruit::Orange => "Orange",
    }
}
