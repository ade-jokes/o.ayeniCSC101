use std::io::Read;

fn main() {
    let mut user_role = String::new();
    println!("Enter which role you are\n");
    println!("a if administrator\n p if project manager\n e if employee\n c if customer\n v if vendor\n");
    io::stdin().read_line(&mut user_role).expect("Failed to read");
    let user_role = user_role.trim(); // Remove expect("Error") here
    match user_role {
        "a" => admin_access(),
        "p" => projectman_access(),
        "e" => employee_access(),
        "c" => customer_access(),
        "v" => vendor_access(),
        _ => println!("No access to info"), // Corrected println syntax
    }
}

fn admin_access() {
	let mut file = std::fs::File::open("globacom_db.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}",contents);
}

fn projectman_access() {
	let mut file = std::fs::File::open("project_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}",contents);
}

fn employee_access() {
	let mut file = std::fs::File::open("staff_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}",contents);
}

fn customer_access() {
	let mut file = std::fs::File::open("customer_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}",contents);
}

fn vendor_access() {
	let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}",contents);
}