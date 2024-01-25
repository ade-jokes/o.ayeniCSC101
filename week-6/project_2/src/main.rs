 use std::io;

fn main() {
    let mut name = String::new();

 println!("The Nigerian Reaserchers Guide (NRG) ");
 
 for  f in 1..500
  { 
         println!("You are Reasercher {} ",f );


         
        println!("Please Enter your full Name");
        io::stdin().read_line(&mut name).expect("Failed to read");


        println!("Enter the number of papers published");
        let mut number_of_papers = String::new();
        io::stdin().read_line(&mut number_of_papers).expect("Failed to read");
        let number_of_papers:i32 = number_of_papers.trim().parse().expect("Failed to input");


      
        if number_of_papers == 3 || number_of_papers == 4 || number_of_papers == 5
         {
            println!(" {} Your incentive is N500,000",name );
        }
         else if number_of_papers >= 5 && number_of_papers <=10
         {
             println!("{} Your incentive is N800,000",name );
        }
            else if number_of_papers >10 
            {
                println!("{} Your incentive is N 100,000,000",name );

            }
                 else if number_of_papers <3
                  {
                       println!("{} Your number of incentive is N100,000 ",name);
                     }
    }   
}
