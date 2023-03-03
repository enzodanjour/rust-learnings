#![deny(clippy::all)]

const MY_PC: u8 = 2;

fn main() {
    //snakeCase with default
    let first_name = "enzo"; // let variable cannot be changed
                             //first_name="enzorium"; //example of imutability
                             //let mut age = 23;
                             //age = 24;
    let age = 23u8;
    println!("Hello, {}!, {} is your age", first_name, age);
    //age ="enzo"; typed error
    // let money = 72_000_000;
    // let red = 0xFA;
    // let rgb= 0xFF0000;
    // let distance_in_km =5.5;

    let distance1 = 5.5;
    let distance2 = 3.5;
    let distance3 = 3.6;
    let total_distance = distance1 + distance2 + distance3;
    println!("{}", total_distance);

    // let data ="Foo";
    // let name=10;
    // {
    //     let data=data.to_string();
    // }

    println!("My pc is{}", MY_PC);

    let word = "Golden Eagle";
    println!("{}",word.ends_with("gla"));
}
