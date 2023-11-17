use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();


  println!(" Are you experienced in this field? (Yes / No) : ");
     io::stdin().read_line(&mut experience).expect("Are you sure this is a string? ");
    let experience= experience.trim();

     println!("Please input your age: ");
     io::stdin().read_line(&mut age).expect("Not a vaild string");
    let age:i32= age.trim().parse().expect("Not a valid number");


    if experience == "Yes"{
       
        if age >= 40 {
            let a:i32 = 1_560_000;
            println!("Your incentive will be {a} ");
        }
        else if age <=30 && age >=40 {
            let b:i32 = 1_480_000;
            println!("Your incentive will be {b}");
        } 
        else if age >= 29 {
            let c:i32 = 1_300_000;
            println!(" Your incentive will be {c}");
        } 
    } else if experience == "No" {
            let d:i32 = 100_000;
            println!("Your incentive will be {d}",);
    } else {
        println!("Invalid entry!");
    }
}
