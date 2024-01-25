use std::io;
use std::fs;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    let mut username = String::new();
    let mut password = String::new();
    let mut company = String::new();


   
    // Username should be at least 4 letters of the company's name.
    loop {
        //Companies info

         println!("Please typein your Company's name");
         println!("Choose the following Companies;
         Cadbury(1)
         Champion(2)
         Dangote(3)
         Flour Milk(4)
         Nestle(5)
         Unliver(6)
         Honeywell(7)
         Nigerian Breweries(8)");
    io::stdin().read_line(&mut company).expect("Invalid input");
    let a:f32= company.trim().parse().expect("Not a valid number");

        //Username info
        println!("Please Enter your Username");
        io::stdin().read_line(&mut username).expect("Invalid username");
        let mut username = username.trim().to_lowercase();

        if username.len() <= 3 || username.len() >= 8 {
            println!("This username is not accepted");
            continue;
        } else if !username.chars().all(|c| c.is_ascii_alphabetic()) {
            println!("This is not a valid Username. Let it contain letters");
            continue;
        }
        break;
    }

    println!("Enter your password: ");
    // The password should contain:
    // - Letters between A-Z
    // - Letters between a-z
    // - Numbers between 0-9
    // - No uppercase
    // - No special character (!@#$%^&*()-+?/\|{}[:\"<>,.;)

    io::stdin().read_line(&mut password).expect("Invalid password");
    let mut password = password.trim().to_string();

    if password.len() < 8 || password.len() > 20 {
        println!("This password is not accepted. It should have between 8 and 20 characters.");
        return;
    }

    let lowercase = password.chars().filter(|c| c.is_lowercase()).count();
    let uppercase = password.chars().filter(|c| c.is_uppercase()).count();
    let numbers = password.chars().filter(|c| c.is_digit(10)).count();
    let special_characters = password.chars().filter(|c| !c.is_alphanumeric()).count();

    if lowercase == 0 || uppercase == 0 || numbers == 0 || special_characters == 0 {
        println!("This password is not accepted. It should have at least one lowercase letter, 
        one uppercase letter, one number, and one special character.");
        return;
    }

    println!("Your username and password have been accepted.");
                    
    //if statement
   if let a = 1 {
    //cadbury info
    let company = vec![
        Company{ name: "____________".to_string(), date: "___________".to_string(), shares: "___________"
        .to_string(), liabilities: "_______".to_string() },
        Company{ name: "Cadbury Nigeria Plc".to_string(), date: "1965".to_string(), shares: "15,000,000"
        .to_string(), liabilities: "5,500,000".to_string() },
        Company{ name: "____________".to_string(), date: "___________".to_string(), shares: "__________"
        .to_string(), liabilities: "________".to_string() },
    ];
    let mut file = fs::File::create("cadbury.txt").expect("create failed");
    file.write_all("Nigerian market.".as_bytes()).expect("write failed");
    
    let _widths = calculate_widths(&company);
    add_company(&mut file, &Company { name: "Name ".to_string(), date: "Date founded".to_string(),
     shares: "shares".to_string(), liabilities: "Liabilitites".to_string() });
    println!("GO to page");
    
    for company in &company {
        add_company(&mut file, company);
    }


   }
   else if let a = 2 {
    let company = vec![
        //champion info
        Company{ name: "____________".to_string(), date: "___________".to_string(), shares: "___________"
        .to_string(), liabilities: "_______".to_string() },
        Company{ name: "Champion Nigeria Plc".to_string(), date: "1974".to_string(), shares: "25,000,000"
        .to_string(), liabilities: "8,000,000".to_string() },
        Company{ name: "____________".to_string(), date: "___________".to_string(), shares: "__________".
        to_string(), liabilities: "________".to_string() },
    ];
    let mut file = fs::File::create("Champion beweries.txt").expect("create failed");
    file.write_all("Nigerian market".as_bytes()).expect("write failed");

    let _widths = calculate_widths(&company);
    add_company(&mut file, &Company { name: "Name ".to_string(), date: "Date founded".to_string(),
    shares: "shares".to_string(), liabilities: "Liabilitites".to_string() });
   println!("GO to page");

    for company in &company {
        add_company(&mut file, company);
    }


            }           
            else if let a = 3 {
                //dangote info
                let company = vec![
                    Company{ name: "____________".to_string(), date: "___________".to_string(), 
                    shares: "___________".to_string(), liabilities: "_______".to_string() },
                    Company{ name: "Dangote Suger Nigeria Plc".to_string(), date: "1970".to_string(),
                     shares: "18,000,000".to_string(), liabilities: "10,000,000".to_string() },
                    Company{ name: "____________".to_string(), date: "___________".to_string(), 
                    shares: "__________".to_string(), liabilities: "________".to_string() },
                ];
                let mut file = fs::File::create("Dangote suger.txt").expect("create failed");
                file.write_all("Nigerian market.".as_bytes()).expect("write failed");

                let _widths = calculate_widths(&company);
    add_company(&mut file, &Company { name: "Name ".to_string(), date: "Date founded".to_string(),
    shares: "shares".to_string(), liabilities: "Liabilitites".to_string() });
   println!("GO to page");

    for company in &company {
        add_company(&mut file, company);
    }

            
            }
            else if let a = 4 {
                //flour mills info
                let company = vec![
                    Company{ name: "____________".to_string(), date: "___________".to_string(), 
                    shares: "___________".to_string(), liabilities: "_______".to_string() },
                    Company{ name: "Flour Mills Nigeria Plc".to_string(), date: "1960".to_string(),
                     shares: "32,000,000".to_string(), liabilities: "4,000,000".to_string() },
                    Company{ name: "____________".to_string(), date: "___________".to_string(), 
                    shares: "__________".to_string(), liabilities: "________".to_string() },
                ];
                let mut file = fs::File::create("Flour mills.txt").expect("create failed");
    file.write_all("Nigerian market.".as_bytes()).expect("write failed");
    let _widths = calculate_widths(&company);
    add_company(&mut file, &Company { name: "Name ".to_string(), date: "Date founded".to_string(),
    shares: "shares".to_string(), liabilities: "Liabilitites".to_string() });
   println!("GO to page");

    for company in &company {
        add_company(&mut file, company);
    }


            }
            else if let a =5 {
                //Nestle info
                let company = vec![
                    Company{ name: "____________".to_string(), date: "___________".to_string(),
                     shares: "___________".to_string(), liabilities: "_______".to_string() },
                    Company{ name: "Nestle Nigeria Plc".to_string(), date: "1961".to_string(), 
                    shares: "8,000,000".to_string(), liabilities: "1,500,000".to_string() },
                    Company{ name: "____________".to_string(), date: "___________".to_string(),
                     shares: "__________".to_string(), liabilities: "________".to_string() },
                ];
                let mut file = fs::File::create("Nestle.txt").expect("create failed");
    file.write_all("Nigerian market.".as_bytes()).expect("write failed");

    let _widths = calculate_widths(&company);
    add_company(&mut file, &Company { name: "Name ".to_string(), date: "Date founded".to_string(),
     shares: "shares".to_string(), liabilities: "Liabilitites".to_string() });
    println!("GO to page");

     for company in &company {
        add_company(&mut file, company);
    }
    }

            else if let a = 6 {
                //unilever info
                let company = vec![
                    Company{ name: "____________".to_string(), date: "___________".to_string(), 
                    shares: "___________".to_string(), liabilities: "_______".to_string() },
                    Company{ name: "Unilever Nigeria Plc".to_string(), date: "1923".to_string(),
                     shares: "37,000,000".to_string(), liabilities: "11,000,000".to_string() },
                    Company{ name: "____________".to_string(), date: "___________".to_string(), 
                    shares: "__________".to_string(), liabilities: "________".to_string() },
                ];
                let mut file = fs::File::create("Unilever.txt").expect("create failed");
    file.write_all("Nigerian market.".as_bytes()).expect("write failed");

    let _widths = calculate_widths(&company);
    add_company(&mut file, &Company { name: "Name ".to_string(), date: "Date founded".to_string(),
     shares: "shares".to_string(), liabilities: "Liabilitites".to_string() });
    println!("GO to page");

    for company in &company {
        add_company(&mut file, company);
    }

            }
            else if let a = 7 {
                //honey well info
                let company = vec![
                    Company{ name: "____________".to_string(), date: "___________".to_string(),
                     shares: "___________".to_string(), liabilities: "_______".to_string() },
                    Company{ name: "Honeywell Plc".to_string(), date: "1906".to_string(),
                     shares: "34,000,000".to_string(), liabilities: "9,000,000".to_string() },
                    Company{ name: "____________".to_string(), date: "___________".to_string(), 
                    shares: "__________".to_string(), liabilities: "________".to_string() },
                ];
                let mut file = fs::File::create("Honeywell.txt").expect("create failed");
    file.write_all("Nigerian market.".as_bytes()).expect("write failed");
    let _widths = calculate_widths(&company);
    add_company(&mut file, &Company { name: "Name ".to_string(), date: "Date founded".to_string(),
     shares: "shares".to_string(), liabilities: "Liabilitites".to_string() });
    println!("GO to page");

    for company in &company {
        add_company(&mut file, company);
    }

            }
            else if let a = 8 {
                //nigerian bereweries info
                let company = vec![
                    Company{ name: "____________".to_string(), date: "___________".to_string(), 
                    shares: "___________".to_string(), liabilities: "_______".to_string() },
                    Company{ name: "Nigerian Bewereies Plc".to_string(), date: "1946".to_string(),
                     shares: "30,000,000".to_string(), liabilities: "12,000,000".to_string() },
                    Company{ name: "____________".to_string(), date: "___________".to_string(),
                     shares: "__________".to_string(), liabilities: "________".to_string() },
                ];
                let mut file = fs::File::create("Nigerian berewries.txt").expect("create failed");
    file.write_all("Nigerian market.".as_bytes()).expect("write failed");
    let _widths = calculate_widths(&company);
    add_company(&mut file, &Company { name: "Name ".to_string(), date: "Date founded".to_string(),
     shares: "shares".to_string(), liabilities: "Liabilitites".to_string() });
    println!("GO to page");
    for company in &company {
        add_company(&mut file, company);
    }

            }
            else {
                println!("You cannot access the file without your Company's name");
            }

}          
fn add_company(file: &mut fs::File, company: &Company) {
    writeln!(file, "{}\t|{}\t|{}\t|{}\t", company.name, company.date, company.shares, company.liabilities)
    .expect("write failed");
}

fn calculate_widths(company: &[Company]) -> [usize; 4] {
    let mut widths = [0; 4];
    for company in company {
        widths[0] = widths[0].max(company.name.len());
        widths[1] = widths[1].max(company.date.len());
        widths[2] = widths[2].max(company.shares.len());
        widths[1] = widths[1].max(company.liabilities.len());
    }
    widths

}

struct Company {
    name: String,
    date: String,
    shares: String,
    liabilities: String,
}