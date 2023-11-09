//rust program to calculate the area of the triangle of three sides
use std::io;

fn main() {
    let mut distance= String::new();
    let mut time= String::new();

    println!("Enter distance ");
    io::stdin().read_line(&mut distance).expect("Not a vaild string");
    let a:f32= distance.trim().parse().expect("Not a valid number");

 println!("Enter time: ");
    io::stdin().read_line(&mut time).expect("Not a vaild string");
    let b:f32= time.trim().parse().expect("Not a valid number");


    let k:f32 =a * 1.609;
    let speed:f32 = k / b;
   
    println!("Speed: {}",speed);

}
