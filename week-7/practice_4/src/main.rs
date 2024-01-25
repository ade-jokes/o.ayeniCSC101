fn add(a: i32, b: i32) {
    let sum = a + b;


    println!("Sum of A and B = {}", sum );
}
fn main() {

    let mut input1 = String::new();
    println!("Enter input for parameter A:");
    io::stdin().read_line(&mut input1).expect("Failed to read line ");
    let num1 :i32=input1.trim().parse().expect("invalid input");

    let mut input2 = String::new();
    println!("Enter input for parameter B:");
    io::stdin().read_line(&mut input2).expect("Failed to read line ");
    let num2 :i32=input2.trim().parse().expect("invalid input");

    // call add function with argument
    add(a,b);
}