use std::fs::File;
use std::io::Write;

// Define the structure of the Cadbury company
struct Cadbury {
    name: String,
    shares: String,
    date: String,
    liabilities: String,
}

fn add_company(file: &mut File, cadbury: &Cadbury) {
    // Write the Cadbury company data to the file
    writeln!(file, "{}\t|{}\t|{}\t|{}\t", cadbury.name, cadbury.shares, cadbury.date, cadbury.liabilities).expect("write failed");
}

fn calculate_widths(cadbury: &[Cadbury]) -> [usize; 4] {
    // Calculate the width of each column in the output file
    let mut widths = [0; 4];
    for cadbury in cadbury {
        widths[0] = widths[0].max(cadbury.name.len());
        widths[1] = widths[1].max(cadbury.shares.len());
        widths[2] = widths[2].max(cadbury.date.len());
        widths[3] = widths[3].max(cadbury.liabilities.len());
    }
    widths
}

fn main() {
    // Define the vector of Cadbury companies
    let cadbury = vec![
        Cadbury { name: "____________".to_string(), shares: "___________".to_string(), date: "___".to_string(), liabilities: "____".to_string() },
        Cadbury { name: "Cadbury Nigeria Plc".to_string(), shares: "15,000,000".to_string(), date: "1965".to_string(), liabilities: "5,500,000".to_string()},
        Cadbury { name: "____________".to_string(), shares: "___________".to_string(), date: "___".to_string(), liabilities: "____".to_string() },
      
    ];

    // Calculate the widths of the columns in the output file
    let widths = calculate_widths(&cadbury);

    // Create the output file and add the header line
    let mut file = File::create("cadbury_companies.txt").expect("Unable to create file");
    writeln!(file, "{}{}\t|{}{}\t|{}{}\t|{}{}",
        "Name",
        " ".repeat(widths[0] - "Name".len()),
        "Shares",
        " ".repeat(widths[1] - "Shares".len()),
        "Date",
        " ".repeat(widths[2] - "Date".len()),
        "Liabilities",
        " ".repeat(widths[3] - "Liabilities".len())
    ).expect("write failed");

    // Add the Cadbury companies to the output file
    for company in &cadbury {
        add_company(&mut file, company);
    }
}