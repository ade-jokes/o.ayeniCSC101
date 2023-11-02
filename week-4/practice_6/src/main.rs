fn main() {
    
    let n1 = "Elctrical" .to_string();
    let n2 = "Electronic". to_string();
    let n3 = "Engineering".to_string();
    let n4 = n1 + &n2 + &n3; // n2 & n3 reference is passed

    // about Electrical/Electronic
    println!("\nThe {} is informed by the aspiration to train electrical/electronic engineering professionals in areas of design, building and maintainance of electrical control systems",n4);

    let w1 = "computer".to_string();
    let w2 ="Science".to_string();
    let w3 = w1 +&w2; //w2 reference is passed
    println!( );
    println!("{} is aimed at developing competent, creative, innovaitive, entrepreneural and ethically-minded persons,
        capablle of creating value in diverse fields pf Computer Science",w3);

}
