use std::io::Write;

fn main(){

    let lager_vec: Vec<&str> = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout_vec: Vec<&str> = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic_vec: Vec<&str> = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];
    
    save_vec("lager.txt", &lager_vec);
    save_vec("stout.txt", &stout_vec);
    save_vec("non_alcoholic.txt", &non_alcoholic_vec);

}

fn save_vec(name: &str, vec: &Vec<&str>){

    let mut file = std::fs::File::create(name).unwrap();
    for &item in vec.iter() {
        file.write_all(format!("{}\n", item).as_bytes()).unwrap();

    }
    
    println!("Saved {}", name);

}