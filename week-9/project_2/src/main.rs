use std::io::Write;

fn main(){

    //Student Name, Matric Number, Department, Level
    let student_vec: Vec<(&str, &str, &str, u32)> = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Eletrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    let mut content = String::from("Student Name, Matric Number, Department, Level\n");

    for student in student_vec.iter(){
        
        let line = format!("{},{},{},{}\n", student.0, student.1, student.2, student.3);
        content += &line;

    }

    println!("PAU SM");
    println!("{}", content);
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all(content.as_bytes()).unwrap();

}