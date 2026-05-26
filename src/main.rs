use std::io;

fn main() {
    let mut scores = Vec::new();

    println!("How many scores do you want to enter?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let count: usize = input.trim().parse().unwrap();

    for i in 1..=count {
        println!("Enter score {}:", i);

        let mut score_input = String::new();
        io::stdin().read_line(&mut score_input).unwrap();

        let score: f64 = score_input.trim().parse().unwrap();

        scores.push(score);
    }

    let sum: f64 = scores.iter().sum();
    let average = sum / scores.len() as f64;

    println!("Average score: {:.2}", average);

    if average >= 90.0 {
        println!("Grade: A");
    } else if average >= 80.0 {
        println!("Grade: B");
    } else if average >= 70.0 {
        println!("Grade: C");
    } else if average >= 60.0 {
        println!("Grade: D");
    } else {
        println!("Grade: F");
    }
}