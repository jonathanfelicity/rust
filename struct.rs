#[allow(dead_code)]


// Unit struct
struct Void;

// tuple struct
struct Location(f32, f32);


// C like struc
struct User{
    name: String,
    age: i8,
    mail: String, 
    balance: f32

}

fn main(){
    let _value = Void;
    let _jos = Location(45.6, 444.5);

    let user = User{
        name:String::from("Jonathan Felicity"),
        age:23, 
        mail: String::from("jonathanfelicity@fake.com"),
        balance:45.7
    };

    println!("{}", user.name);
    println!("{}", user.age);
    println!("{}", user.mail);
    println!("{}", user.balance);

}