fn main() {
    //Creat an empty vector "City"
    let mut city :Vec<String> = Vec::new();

    //Print City Vector
    println!("The city Vector has element {}",city.len());

    //push new elements into
    let mut input1 = String::new();
    println!("How many cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Failed to input");
    let city_num:i32 = input1.trim().parse().expect("Failed to input");
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter City {}",count+1);
        std::io::stdin().read_line(&mut input2).expect("Failed to input");
        let new_city:String = input2.trim().parse().expect("Failed to input");
        city.push(new_city);

    }
    print!("Your preferred cities are:\n ");
    let mut count=1;
    //loop to iterate elements in vector
    for i in &city{
        println!("{} {}",count, i);
        count+=1;
}

}
    
