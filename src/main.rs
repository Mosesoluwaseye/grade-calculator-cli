use std::io;

fn main() {
    let mut scores = Vec::new();

    println!("Enter maximum points (60, 70, 80, 90, 100):");

    let mut max_input = String::new();
    io::stdin().read_line(&mut max_input).unwrap();

    let max_points: f64 = max_input.trim().parse().unwrap();

    println!("How many scores do you want to enter?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let count: usize = input.trim().parse().unwrap();

    for i in 1..=count {
        println!("Enter score {}:", i);

        let mut score_input = String::new();
        io::stdin().read_line(&mut score_input).unwrap();

        let score: f64 = score_input.trim().parse().unwrap();

        let percentage = (score / max_points) * 100.0;

        scores.push(percentage);
    }

    let sum: f64 = scores.iter().sum();
    let average = sum / scores.len() as f64;

    println!("Average score: {:.2}", average);

    let mut a_count = 0;
    let mut b_count = 0;
    let mut c_count = 0;
    let mut d_count = 0;
    let mut f_count = 0;

    if average >= 90.0 {
        println!("Grade: A");
        a_count += 1;
    } else if average >= 80.0 {
        println!("Grade: B");
        b_count += 1;
    } else if average >= 70.0 {
        println!("Grade: C");
        c_count += 1;
    } else if average >= 60.0 {
        println!("Grade: D");
        d_count += 1;
    } else {
        println!("Grade: F");
        f_count += 1;
    }

    println!("\nGrade Distribution");
    println!("A: {}", a_count);
    println!("B: {}", b_count);
    println!("C: {}", c_count);
    println!("D: {}", d_count);
    println!("F: {}", f_count);
}
