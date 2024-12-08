use std::io;

type Candidate = (String, u32, u32); // name, age, years of experience

fn main() {
    let mut candidates: Vec<Candidate> = Vec::new();
    loop {
    let choice = read_line("Choose option:
    1. Add new candidate
    2. Remove candidate
    3. List candidates
    4. Exit").parse().expect("Failed to parse option");
        match choice {
            1 => add_candidate(&mut candidates),
            2 => remove_candidate(&mut candidates),
            3 => list_candidates(&candidates),
            4 => break,
            _ => println!("Invalid option, please select from 1-4\n"),
        };
    }
    if candidates.is_empty() {
        println!("No candidates added");
    } else {
        candidates.sort_by(|a, b| b.2.cmp(&a.2));
        list_candidates(&candidates);
        println!("Candidate with most experience: {}, {} years of experience", candidates[0].0, candidates[0].2);
    }
}

fn add_candidate(candidates: &mut Vec<Candidate>) {
    let name = read_line("Enter name:");
    let age: u32 = read_line("Enter age:").parse().expect("Failed to parse age");
    let years_of_experience: u32 = read_line("Enter years of experience:").parse().expect("Failed to parse experience");
    candidates.push((name, age, years_of_experience));
    println!("Successfully added candidate");
}

fn remove_candidate(candidates: &mut Vec<Candidate>) {
    let remove_no: usize = read_line("Enter candidate number to remove:").parse().expect("Failed to parse candidate number");
    let removed_candidate = candidates.remove(remove_no - 1);
    println!("Removed candidate {remove_no}: {}", removed_candidate.0);
}

fn list_candidates(candidates: &[Candidate]) {
    println!("\nList of candidates:\n");
    for (i, candidate) in candidates.iter().enumerate() {
        println!("{}, {}, {} years old, {} years of experience", i + 1, candidate.0, candidate.1, candidate.2);
    }
}

fn read_line(hint: &str) -> String {
    println!("{}", hint);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().to_string();
}