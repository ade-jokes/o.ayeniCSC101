
use std::fs;
use std::io::Write;

// Writing the Drinks inside the file using the write_drinks function

fn write_drinks(file: &mut fs::File, drinks: &[&str], category: &str) {
    writeln!(file, "\n{} Drinks:", category).expect("write failed");
    for drink in drinks {
        writeln!(file, "{}", drink).expect("write failed");
    }
}

fn main() { 
    let lager = vec! ["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King","Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta","Malta Gold","Fayrouz"];

    let mut file = fs::File::create("portfolio.txt").expect("create failed");
    file.write_all("Nigerian Brewery limited.".as_bytes()).expect("write failed");

    write_drinks(&mut file, &lager, "Lager");
    write_drinks(&mut file, &stout, "Stout");
    write_drinks(&mut file, &non_alcoholic, "Non-alcoholic");

    println!("Go to file page\n");
}