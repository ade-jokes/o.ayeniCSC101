use std::fs;
use std::io::Write;

fn add_student(file: &mut fs::File, student: &Student) {
    writeln!(file, "{}\t|{}\t|{}\t|{}\t", student.name, student.matric_number, student.department, student.level).expect("write failed");
}

fn calculate_widths(students: &[Student]) -> [usize; 4] {
    let mut widths = [0; 4];
    for student in students {
        widths[0] = widths[0].max(student.name.len());
        widths[1] = widths[1].max(student.matric_number.len());
        widths[2] = widths[2].max(student.department.len());
        widths[1] = widths[1].max(student.level.len());
    }
    widths
}


fn main() {
    let students = vec![
        Student { name: "____________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Student { name: "Oluchi mordi".to_string(), matric_number: "ACC10211111".to_string(), department: "Accounting     ".to_string(), level: "300".to_string() },
        Student { name: "____________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Student { name: "Adams Aliyu".to_string(), matric_number: "ECO101110101".to_string(), department: "Economics       ".to_string(), level: "100".to_string() },
        Student { name: "____________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Student { name: "Shania Bolade".to_string(), matric_number: "CSC10328828".to_string(), department: "Computer Science".to_string(), level: "200".to_string() },
        Student { name: "____________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Student { name: "Blanca Edemoh".to_string(), matric_number: "MEE1020202001".to_string(), department: "Mechanical            ".to_string(), level: "100".to_string()},
        Student { name: "____________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
        Student { name: "Adekunle Gold".to_string(), matric_number: "EE11020202".to_string(), department: "Electrical      ".to_string(), level: "200".to_string() },
        Student { name: "____________".to_string(), matric_number: "___________".to_string(), department: "_________________".to_string(), level: "____".to_string() },
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