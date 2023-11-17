// Rust program for TEST//
// Otunene Family centre for Health care

use std::io;
fn main() {
    let name = String::new();
    let age = String::new();
    let date_of_birth = String::new();
    let email_address = String::new();
    let phone_number = String::new();
    let number_of_siblings = String::new();
    let number_of_children = String::new();
    let medical_diagnosis = String::new();
    let village_of_residence = String::new();
    

    println!("HEALTH DIAGNOSIS\t AMOUNT(N)\t VILLAGE\t DISCOUNT");
    println!("Alsheimer\t\t 1,200,000,\t Akpabom\t 20%");
    println!("Arrhythmia\t\t 550,000\t Ngbauji\t 5%", );
    println!("Chronic kidney disease   1,500,000 \t Atabrikang\t 15%", );
    println!("Diabetes\t\t 800,000\t okorobilom\t 10%", );
    println!("Arthritis\t\t 450,000\t Emeremen\t 10%", );

     println!("Welcome to Otunene Family Centre", );
     println!("please let your village_of_residence be the first letter");

      println!("What's your name please", );
      io::stdin().read_line(&mut name).expect("Not a valid string please try again");

      println!("Enter age", );
      io::stdin().read_line(&mut age).expect("Not a valid string please try again");
      let age: i32 = age.trim().parse().expect("Not a valid number");

      println!("Your date of birth", );
      io::stdin().read_line(&mut date_of_birth).expect("Not a valid string please try again");
      
      println!("Your email_address", );
      io::stdin().read_line(&mut email_address).expect("Not a valid string please try again");
      
      println!("Your phone_number", );
      io::stdin().read_line(&mut phone_number).expect("Not a valid string please try again");
      
      println!("Your number_of_siblings", );
      io::stdin().read_line(&mut number_of_siblings).expect("Not a valid string please try again");
      let number_of_siblings: i32 = number_of_siblings.trim().parse().expect("Not a valid number")

      println!("number_of_children ");
      io::stdin().read_line(&mut number_of_children).expect("Not a valid string please try again");
      let number_of_children: i32 = number_of_children.trim().parse().expect("Not a valid number");
      
      println!("medical_diagnosis");
      io::stdin().read_line(&mut medical_diagnosis).expect("Not a valid string please try again");
      
      println!("village_of_residence", );
      io::stdin().read_line(&mut village_of_residence).expect("Not a valid string please try again");

      if age < 50 && medical_diagnosis== "Alzeheimer" && village_of_residence=="Akpabom" {
        let mut amount = 1_200_00 +(20.0/100.0);
      }else if age == 30 &&number_of_siblings <= 4  &&village_of_residence == "N" &&medical_diagnosis== "Arrhythmia" {
        let amount= 550_000 +(5.0/100.0);
      } else if age <= 40 &&number_of_children <= 3 &&village_of_residence== "A" &&medical_diagnosis== "Chronic" {
        let amount = 1_500_000 + (15.0/100.0);

      } else if age <= 28 && age >= 45 number_of_children <=2 && number_of_children>=4 village_of_residence== "O" medical_diagnosis== "Diabetes" {
        let amount= 800_000 + (10.0 / 100.0);

      } else if age <= 58 && number_of_children <= 5 && number_of_siblings <= 5 &&village_of_residence == "E" medical_diagnosis== "Arthritis"{
        let amount= 450_000 +( 10.0 / 100.0)
      }

    else{
        println!("Try again later");
    }

}
