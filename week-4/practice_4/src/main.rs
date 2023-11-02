fn main() {
    
    let fullname= "Chibudum John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";


    let mut school = "School of science". to_string();
    //push sring
    school.push_str(" and Technology");

    println!(" My name is: {}",fullname);
    // check lenght
    println!("The lenght my fullname is: {}",fullname.len());
    println!("I am a student of {} department",department);
    println!("{}",school);
    println!("{}",uni );
}
