//Registration of PAU voting
use std::io;


fn main() {
    let mut first_name = String::new();
    let mut sur_name = String::new();
    let mut email = String::new();
    let mut department =String::new();
    let mut input2 = String::new();
    let mut level = String::new(); // 100l - 400l
    let mut state_of_origin = String::new();
        println!("Hello there,");
    for y in 0..150
    {

        println!("ENTER YOUR FIRST NAME");
        io::stdin().read_line(&mut first_name).expect("Failed to input");



        println!("ENTER YOUR LAST NAME");
        io::stdin().read_line(&mut sur_name).expect("Failed to input");


        println!("ENTER YOUR EMAIL (advised school email)");
        io::stdin().read_line(&mut email).expect("Failed to input");

        println!("ENTER YOUR DEPARTMENT");
        io::stdin().read_line(&mut department).expect("Failed to input");

        println!("ARE YOU A CLASS REPRESENTATIVE? ");
        io::stdin().read_line(&mut input2).expect("Failed to input");
        let class_rep:bool = input2.trim().parse().expect("Not Valid");

        println!("Enter your level");
        io::stdin().read_line(&mut level).expect("Failed to input");
        let level:i32 = level.trim().parse().expect("Failed to input");

        println!("Enter your state of origin");
        io::stdin().read_line(&mut state_of_origin).expect("Failed to input");


        println!("What is your CGPA");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read");
        let input1:f32 = input1.trim().parse().expect("Failed to input");

         println!("YOU ARE THE {} CANDIDATE", y );


        if class_rep == false && input1 >= 4.00 && level == 200 ||level ==300 || level == 400
        { 
            println!("{} 
                      {} 
                      {} 
                      {} 
                      {} 
                      {}
                      You can vote",first_name,sur_name, email, department, level, state_of_origin);
        }
        else 
        { println!("Sorry, You cannot vote"); } }
         

}
