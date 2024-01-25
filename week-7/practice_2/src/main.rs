fn checker() {
    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch:char = input.trim().parse().expect("Invalid input");

    if ch>= '0' && ch <= '9'
    {
        println!("character '{}' is a digit",ch);
        }
        else {
            println!("character '{}' is not a digit",ch);

        }
    }
    fn main() {
        //calling function
        println!("Welcome! this  program checks whether a character variablr contains a digit or not");
        checker();
    }
}
