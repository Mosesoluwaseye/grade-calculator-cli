# Grade Calculator CLI

A Rust-powered command-line application that reads student data from a CSV file, converts scores using different grading scales, calculates averages, assigns letter grades, and displays grade distributions with a bar chart.

---

## Problem Statement

Students and instructors often work with grade lists stored in spreadsheets or CSV files rather than entering grades manually. Calculating averages and determining final grades can be repetitive and error-prone.

This project automates the process by reading grades from a CSV file, converting them to percentages based on a selected grading scale, calculating the average, assigning a letter grade, and reporting the grade distribution.

---

## Features

* Read grades from a CSV file
* Support student IDs or email addresses
* Support grading scales of 60, 70, 80, 90, and 100 points
* Calculate average scores automatically
* Convert scores to percentages
* Assign letter grades automatically
* Report grade distribution (A, B, C, D, F)
* Display a grade distribution bar chart
* Simple command-line interface
* Built with Rust

---

## Technologies Used

* Rust
* Cargo
* CSV Crate
* Git
* GitHub
* GitHub Actions
* GitHub Pages

---

## Installation

Clone the repository:

```bash
git clone https://github.com/Mosesoluwaseye/grade-calculator-cli.git
```

Navigate into the project directory:

```bash
cd grade-calculator-cli
```

Run the application:

```bash
cargo run
```

---

## CSV Format

The CSV file should contain two columns:

1. Student ID or Email
2. Points

Example using student IDs:

```csv
student_id,points
1001,54
1002,48
1003,60
1004,37
1005,55
```

Example using email addresses:

```csv
email,points
student1@example.com,54
student2@example.com,48
student3@example.com,60
student4@example.com,37
student5@example.com,55
```

Save the file as:

```text
grades.csv
```

---

## How to Use

1. Create a `grades.csv` file using the format shown above.
2. Place the file in the project directory.
3. Run the application:

```bash
cargo run
```

4. Enter the maximum possible points when prompted:

```text
Enter maximum points (60, 70, 80, 90, 100):
```

5. The program will:

   * Read student records from the CSV file
   * Convert scores to percentages
   * Calculate the class average
   * Assign a letter grade
   * Display the grade distribution
   * Display a grade distribution bar chart

---

## Example Output

```text
Enter maximum points (60, 70, 80, 90, 100):
60

Student: 1001 | Score: 54 | Percentage: 90.00%
Student: 1002 | Score: 48 | Percentage: 80.00%
Student: 1003 | Score: 60 | Percentage: 100.00%
Student: 1004 | Score: 37 | Percentage: 61.67%
Student: 1005 | Score: 55 | Percentage: 91.67%

Average score: 84.67
Grade: B

Grade Distribution
A: 3
B: 1
C: 0
D: 1
F: 0

Bar Chart
A | *** (3)
B | *   (1)
C |     (0)
D | *   (1)
F |     (0)
```

---

## Skills Demonstrated

* Rust Programming
* File Handling
* CSV Processing
* User Input Handling
* Data Processing
* Conditional Logic
* Command-Line Application Development
* Git Version Control
* Technical Documentation

---

## Project Structure

```text
grade_calculator_cli/
├── src/
│   └── main.rs
├── grades.csv
├── Cargo.toml
├── Cargo.lock
├── README.md
└── .github/
    └── workflows/
```

---

## Live Demo

Project Website:

https://mosesoluwaseye.github.io/grade-calculator-cli/

GitHub Repository:

https://github.com/Mosesoluwaseye/grade-calculator-cli

---

## Professor Feedback Implemented

* Added support for CSV file input
* Added support for student IDs or email addresses
* Added grading scales for 60, 70, 80, 90, and 100 points
* Added grade distribution reporting
* Added a grade distribution bar chart
* Expanded project documentation and usage instructions
* Removed the need for manual score entry

---

## License

This project is licensed under the MIT License.
