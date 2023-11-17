// quadratic equation
 use std::io;
fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();


     println!("Enter First value: ");
    io::stdin().read_line(&mut a).expect("Not a vaild string");
    let a:f32= a.trim().parse().expect("Not a valid number");

    println!("Enter Second value: ");
    io::stdin().read_line(&mut b).expect("Not a vaild string");
    let b:f32= b.trim().parse().expect("Not a valid number");

    println!("Enter third value: ");
    io::stdin().read_line(&mut c).expect("Not a vaild string");
    let c:f32= c.trim().parse().expect("Not a valid number");
    
 
    let d:f32 =(b.powf(2.0) - (4.0*a*c));
     
     if d >0.0 {
        let x= ((0.0-b) -  d.sqrt() / (2.0 * a ) );
        let y= ((0.0 - b) + d.sqrt() / (2.0 * a) );

        println!("The roots of the equation is {x} , {y}");
    } else if d ==0.0 {
        let q = ((0.0 -b) / (2.0 * a)); 
        println!("The root of the equation is:{q}", );

        } else {
            println!("This is not a real root");
            }
   
}
