use std::fs::File;
use std::io;

fn main() {
    println!("Enter maximum points (60, 70, 80, 90, 100):");

    let mut max_input = String::new();
    io::stdin().read_line(&mut max_input).unwrap();

    let max_points: f64 = max_input.trim().parse().unwrap();

    let file = File::open("grades.csv").unwrap();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut scores = Vec::new();

    let mut a_count = 0;
    let mut b_count = 0;
    let mut c_count = 0;
    let mut d_count = 0;
    let mut f_count = 0;

    for result in rdr.records() {
        let record = result.unwrap();

        let score: f64 = record[0].parse().unwrap();

        let percentage = (score / max_points) * 100.0;

        scores.push(percentage);

        if percentage >= 90.0 {
            a_count += 1;
        } else if percentage >= 80.0 {
            b_count += 1;
        } else if percentage >= 70.0 {
            c_count += 1;
        } else if percentage >= 60.0 {
            d_count += 1;
        } else {
            f_count += 1;
        }
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

    println!("\nGrade Distribution");
    println!("A: {}", a_count);
    println!("B: {}", b_count);
    println!("C: {}", c_count);
    println!("D: {}", d_count);
    println!("F: {}", f_count);
}
