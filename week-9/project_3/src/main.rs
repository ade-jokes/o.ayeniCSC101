// Merging datasets into single output with arrays and vectors
 use std::fs;
 use std::io::Write;

 fn merge_dataset(file: &mut fs::File, e_f_c_c: &Datasets)  {
    writeln!(file, "{}\t|{}\t|{}\t|{}\t", e_f_c_c.s_n, e_f_c_c.name_of_commisioner, e_f_c_c.ministry, e_f_c_c.geopolitical_zone).expect("write failed");
 }
 fn calculate_widths(e_f_c_c: &[Datasets]) -> [usize; 4] {
    let mut widths = [0; 4];
    for e_f_c_c in e_f_c_c {
        widths[0] = widths[0].max(e_f_c_c.name_of_commisioner.len());
        widths[1] = widths[1].max(e_f_c_c.s_n.len());
        widths[2] = widths[2].max(e_f_c_c.ministry.len());
        widths[1] = widths[1].max(e_f_c_c.geopolitical_zone.len());
    }
    widths
}

fn main() {
    let e_f_c_c = vec![
        Datasets { s_n: "____".to_string(),name_of_commisioner: "___________".to_string(), ministry: "_________________".to_string(), geopolitical_zone: "____".to_string() },
        Datasets { s_n: "Aigbogun".to_string(),name_of_commisioner: "ACC10211111".to_string(), ministry: "Accounting     ".to_string(), geopolitical_zone: "300".to_string() },
        Datasets { s_n: "____".to_string(), name_of_commisioner: "___________".to_string(), ministry: "_________________".to_string(), geopolitical_zone: "____".to_string() },
        Datasets { s_n: "Adams Aliyu".to_string(), name_of_commisioner: "ECO101110101".to_string(), ministry: "Economics       ".to_string(),geopolitical_zone: "100".to_string() },
        Datasets { s_n: "____".to_string(), name_of_commisioner: "___________".to_string(), ministry: "_________________".to_string(), geopolitical_zone: "____".to_string() },
        Datasets{ s_n: "Shania Bolade".to_string(), name_of_commisioner: "CSC10328828".to_string(), ministry: "Computer Science".to_string(), geopolitical_zone: "200".to_string() },
        Datasets { s_n: "____".to_string(), name_of_commisioner: "___________".to_string(), ministry: "_________________".to_string(), geopolitical_zone: "____".to_string() },
        Datasets{ s_n: "Blanca Edemoh".to_string(),name_of_commisioner: "MEE1020202001".to_string(), ministry: "Mechanical            ".to_string(), geopolitical_zone: "100".to_string()},
        Datasets{ s_n: "_____".to_string(), name_of_commisioner: "___________".to_string(), ministry: "_________________".to_string(), geopolitical_zone: "____".to_string() },
        Datasets { s_n: "Adekunle Gold".to_string(), name_of_commisioner: "EE11020202".to_string(), ministry: "Electrical      ".to_string(), geopolitical_zone: "200".to_string() },
        Datasets { s_n: "_____".to_string(),name_of_commisioner: "___________".to_string(), ministry: "_________________".to_string(), geopolitical_zone: "____".to_string() },
      ];

      let mut file = fs::File::create("Merge Collection.txt").expect("create failed");
      file.write_all("                EFCC Documents \n".as_bytes()).expect("write failed");
  
      let _widths = calculate_widths(&e_f_c_c);
      merge_dataset(&mut file, &Datasets { s_n: "Name ".to_string(), name_of_commisioner: "Matric No.   ".to_string(), ministry: "Department            ".to_string(), geopolitical_zone: "Level".to_string() });
      println!("GO to page");
  
      for e_f_c_c in &e_f_c_c {
          merge_dataset(&mut file, e_f_c_c);
      }

}

struct Datasets {
    name_of_commisioner: String,
    s_n: String,
    ministry: String,
    geopolitical_zone: String,
}
    
