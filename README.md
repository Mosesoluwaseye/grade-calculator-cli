# Grade Calculator CLI

A Rust-powered command-line application that reads student scores from a CSV file, converts them to percentages using different grading scales, calculates the average score, assigns a letter grade, and reports grade distribution.

---

## Problem Statement

Instructors and students often store grades in spreadsheets rather than entering them manually one at a time. Calculating averages, converting scores from different point scales, and determining grade distributions can be repetitive and error-prone.

This project automates the process by reading grades from a CSV file, supporting multiple maximum point scales, calculating averages, assigning letter grades, and reporting grade distributions.

---

## Features

* Read scores from a CSV file
* Support multiple grading scales (60, 70, 80, 90, and 100 points)
* Convert raw scores to percentages automatically
* Calculate average score
* Assign letter grades (A–F)
* Report grade distribution
* Lightweight and fast
* Simple command-line interface
* Cross-platform Rust application

---

## Technologies Used

* Rust
* CSV Crate
* Cargo
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

Create a file named `grades.csv` in the project directory:

```text
55
58
49
54
```

Each line should contain a single score.

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
* Conditional Logic
* Looping and Iteration
* Data Processing
* Command-Line Application Development
* Git Version Control
* GitHub Actions
* GitHub Pages
* Technical Documentation

---

## Project Structure

```text
grade_calculator_cli/
├── .github/
│   └── workflows/
│       └── rust.yml
├── src/
│   └── main.rs
├── grades.csv
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
└── index.html
```

---

## Live Demo

Project Website:

https://mosesoluwaseye.github.io/grade-calculator-cli/

GitHub Repository:

https://github.com/Mosesoluwaseye/grade-calculator-cli

---

## License

This project is licensed under the MIT License.
