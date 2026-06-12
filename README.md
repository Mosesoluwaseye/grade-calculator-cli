# Grade Calculator CLI

A Rust-powered command-line application that calculates grade averages from a CSV file, converts scores using different grading scales, assigns letter grades, and reports grade distributions.

---

## Problem Statement

Students and instructors often work with grade lists stored in spreadsheets or CSV files rather than entering grades manually. Calculating averages and determining final grades can be repetitive and error-prone.

This project automates the process by reading grades from a CSV file, converting them to percentages based on a selected grading scale, calculating the average, assigning a letter grade, and reporting the grade distribution.

---
x
## Features

* Read grades from a CSV file
* Support grading scales of 60, 70, 80, 90, and 100 points
* Calculate average scores automatically
* Convert scores to percentages
* Assign letter grades automatically
* Report grade distribution (A, B, C, D, F)
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

Install dependencies and run:

```bash
cargo run
```

---

## CSV Format

Create a file named:

```text
grades.csv
```

Example:

```csv
54
56
52
```

Each line represents a student's score.

---

## Example Usage

```text
Enter maximum points (60, 70, 80, 90, 100):
60

Average score: 90.00
Grade: A

Grade Distribution
A: 2
B: 1
C: 0
D: 0
F: 0
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

* Added support for grading scales of 60, 70, 80, 90, and 100 points
* Added CSV file input support
* Added grade distribution reporting
* Removed the need for manual score entry

---

## License

This project is licensed under the MIT License.

