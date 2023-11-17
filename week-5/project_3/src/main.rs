/*
Rust program to display food order
*/
use std::io;

fn main() {
    let input = io::stdin();
    let mut total:f64 = 0.0;

    println!("\t\tMenu\t\t\t Price ");
    println!("\tP= Poundo Yam/Eddinkaiko soup\t-N3,200 ");
    println!("\tF= Fried Rice & Chicken\t\t-N3,000");
    println!("\tA= Amala & Ewedu Soup\t\t-N2,500");
    println!("\tE= Eba & Egusi Soup\t\t-N2,000");
    println!("\tW= White rice & Stew\t\t-N2500");
   

   println!("Choose your order;");
   println!("Please Choose your orders by the letter");
   println!("You can choose how much you like");
   println!("Type Q to quit");


   //programming for the Orders
   loop {
        let mut food =String::new();
        input.read_line(&mut food).expect("Not a vaild Order");
        let food = food.trim();


        if food == "p"{
            total += 3_200.0
        }
        else if food == "f"{
            total += 3_000.0
        }
        else if food == "a"{
            total += 2_500.0
        }
        else if food == "e"{
            total += 2_000.0
        }
        else if food == "w" {
            total += 2_500.0
        }
        else if food == "q"{
            break;
        }
        else {
            println!("Sorry we don't have those in our menu ");
            continue;
        }
   }
   if total > 10_000.0 {


     let new_total:f64 = total- ((5.0 - 100.0)*total);

     println!("Your balance is: {} Because your amount is more than 10,000 we are giving you a free 5% discount ",new_total);
     println!("Thank you for ordering"); 
        }
        else {
            println!("Your balance is: {:?}",total );
            println!("Thank you for Ordering");
        }



}
