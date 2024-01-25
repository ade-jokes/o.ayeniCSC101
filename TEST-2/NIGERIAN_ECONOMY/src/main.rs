use std::fs;
use std::io::Write;

fn add_company(file: &mut fs::File, company: &Company) {
    writeln!(file, "{}\t|{}\t|{}\t|{}\t", company.name, company.shares, company.liabilities, company.date).expect("write failed");
}

fn calculate_widths(company: &[Company]) -> [usize; 5] {
    let mut widths = [0; 5];
    for company in company {
        widths[0] = widths[0].max(student.name.len());
        widths[1] = widths[1].max(student.matric_number.len());
        widths[2] = widths[2].max(student.department.len());
        widths[1] = widths[1].max(student.level.len());
    }
    widths
}


fn main() {
    let students = vec![
        Company { name: "___________________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Company { name: "Cadbury Nigeria Plc".to_string(), matric_number: "ACC10211111".to_string(), department: "Accounting     ".to_string(), level: "300".to_string() },
        Company { name: "___________________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Company { name: "Champion breweries Plc".to_string(), matric_number: "ECO101110101".to_string(), department: "Economics       ".to_string(), level: "100".to_string() },
        Company { name: "___________________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Company { name: "Dangote Suger Refinery Plc.".to_string(), matric_number: "CSC10328828".to_string(), department: "Computer Science".to_string(), level: "200".to_string() },
        Company { name: "___________________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Company { name: "Flour Mills Nigeria Plc".to_string(), matric_number: "MEE1020202001".to_string(), department: "Mechanical            ".to_string(), level: "100".to_string()},
        Company { name: "___________________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Company { name: "Nestle Nigeria Plc".to_string(), matric_number: "EE11020202".to_string(), department: "Electrical      ".to_string(), level: "200".to_string() },
        Company { name: "__________________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Company { name: "Unilever Nigeria Plc".to_string(), matric_number: "EE11020202".to_string(), department: "Electrical      ".to_string(), level: "200".to_string() },
        Company { name: "__________________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Company { name: "Honeywell Nigeria Plc".to_string(), matric_number: "EE11020202".to_string(), department: "Electrical      ".to_string(), level: "200".to_string() },
        Company { name: "__________________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Company { name: "Nigerian Breweries Plc".to_string(), matric_number: "EE11020202".to_string(), department: "Electrical      ".to_string(), level: "200".to_string() },
        Company { name: "__________________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
    ];
 


    let mut file = fs::File::create("PAU SMIS.txt").expect("create failed");
    file.write_all("                PAU SMIS\n".as_bytes()).expect("write failed");

    let _widths = calculate_widths(&students);
    add_student(&mut file, &Student { name: "Name          ".to_string(), matric_number: "Matric No.   ".to_string(), department: "Department            ".to_string(), level: "Level".to_string() });
    println!("GO to page");

    for student in &students {
        add_student(&mut file, student);
    }
}

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: String,
}