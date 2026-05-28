# Grade Calculator CLI

A Rust-powered command-line application that helps students quickly calculate grade averages and automatically determine final letter grades.

---

## Problem Statement

Students often calculate academic averages manually using calculators or spreadsheets. This process can become repetitive, slow, and prone to errors.

This project solves the problem by automating grade calculations directly from the command line using Rust.

---

## Features

* Input multiple scores
* Automatic average calculation
* Automatic letter grade assignment
* Lightweight and fast
* Simple command-line interface
* Cross-platform Rust application

---

## Technologies Used

* Rust
* Cargo
* GitHub Actions
* GitHub Pages

---

## How It Works

The application asks users:

1. How many scores they want to enter
2. Each score individually

It then:

* Calculates the average
* Displays the corresponding grade

---

## Example Usage

```bash
cargo run
```

Example Output:

```text
How many scores do you want to enter?
3

Enter score 1:
70

Enter score 2:
80

Enter score 3:
90

Average score: 80.00
Grade: B
```

---

## Project Structure

```text
grade-calculator-cli/
│
├── src/
│   └── main.rs
│
├── .github/
│   └── workflows/
│       └── build.yml
│
├── assets/
│   └── demo.png
│
├── Cargo.toml
├── README.md
├── LICENSE
└── index.html
```

---

## Installation

Clone the repository:

```bash
git clone https://github.com/Mosesoluwaseye/grade-calculator-cli.git
```

Move into the project directory:

```bash
cd grade-calculator-cli
```

Run the application:

```bash
cargo run
```

---

## Build Release Binary

```bash
cargo build --release
```

Compiled binaries will be generated inside:

```text
target/release/
```

---

## Target Users

This tool is designed for:

* University students
* Teachers
* Rust beginners
* Developers learning command-line applications

---

## Marketing Strategy

The application can be promoted through:

* GitHub open-source communities
* Student forums
* Rust programming communities
* Developer social media platforms
* Technical blogging platforms

---

## Monetization Strategy

Potential premium features could include:

* GPA tracking
* Semester analytics
* Export to PDF or CSV
* Grade history storage
* Cloud synchronization

---

## GitHub Actions

This project uses GitHub Actions to automatically build and cross-compile binaries for Linux platforms.

---

## GitHub Pages

A GitHub Pages website is included to market the application and explain its purpose to users.

---

## Preview

Add your application screenshot inside:

```text
assets/demo.png
```

Then it will appear below:

![Preview](assets/demo.png)

---

## Future Improvements

* Better input validation
* GPA calculation
* File export support
* Interactive CLI menus
* Semester performance tracking

---

## License

This project is licensed under the MIT License.
