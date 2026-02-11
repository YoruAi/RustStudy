fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new(); // omitted type if compiler can infer

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50); // insert
    scores.insert(String::from("Yellow"), 25); // overwrite
    let score = scores.entry(String::from("Yellow")).or_insert(50); // insert if not exist
    *score += 1;

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied();
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let (best_team_name, _) = scores.iter().max_by_key(|&(_, v)| v).expect("empty");
    let dept = "Google".to_string();
    let employee = "Bob".to_string();
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    company
        .entry(dept.clone())
        .or_insert_with(Vec::new)
        .push(employee.clone());
    let employees = if let Some(employees) = company.get(&dept) {
        let mut sorted = employees.clone();
        sorted.sort();
        Some(sorted)
    } else {
        None
    };
}
