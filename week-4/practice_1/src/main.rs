fn main() {
    let name = "aisha lawal";
    let uni:&str = "pan-atlantic university";
    let addr:&str= "km 52 lekki-epe expressway, ibeju lekki, lagos";
    println!("name: {}",name);
    println!("University: {:?}, \n address: {}", uni,addr );

    let department:&'static str = "Computer science";
    let school:&'static str= "School of Science and Technology";
    println!("Department:{}, \nschool: {}",department, school );
}
