[![CI/CD](https://github.com/nogibjj/Ramil-Script-Command-Line-Tool-Rust/actions/workflows/ci.yaml/badge.svg)](https://github.com/nogibjj/Ramil-Script-Command-Line-Tool-Rust/actions/workflows/ci.yaml)

## Overview

This repository contains a command-line tool written in Rust that allows users to calculate monthly loan payments based on the principal amount, annual interest rate, and the number of payment periods. It provides a simple and effective way to perform financial calculations directly from the terminal using Rust. This project contain CI/CD , appropriate Testing, Command Line Interface, simple Rust script and reproducible environment for cloud platforms. 

## Features

- Calculate monthly loan payments based on user inputs.
- Modular design with separate functions for different calculations.
- Command-line interface (CLI) for easy interaction for Rust script.

## Installation

To install and run this project, ensure you have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

Clone the repository:

```bash
git clone https://github.com/nogibjj/Ramil-Script-Command-Line-Tool-Rust.git
cd Ramil-Script-Command-Line-Tool-Rust
```

Build the project:

```bash
cargo build --release
```

## Usage

Once the project is built, you can run the CLI tool using:

```bash
./target/release/your_tool_name calculate_m_payment <principal> <rate> <periods>
```

### Example

To calculate the monthly payment for a loan of $100,000 at an annual interest rate of 5% over 30 years:

```bash
./target/release/your_tool_name calculate_m_payment 100000 0.05 360
```

This command will output the monthly loan payment.

## Functions

The main functions of the tool are defined in the `lib.rs` file:

- `calculate_monthly_rate(annual_rate: f64) -> f64`: Calculates the monthly interest rate from the annual rate.
- `calculate_discount_factor(monthly_rate: f64, periods: i32) -> f64`: Computes the discount factor for the loan.
- `get_monthly_payment(principal: f64, rate: f64, periods: i32) -> f64`: Calculates the monthly loan payment.

## Testing

You can run the tests provided in the repository using:

```bash
make test
```

## Images

**Run CLI Console Output**
![Run Example Console Output](https://github.com/nogibjj/Ramil-Script-Command-Line-Tool-Rust/blob/ca094cb4c95caa351732d4c8fd9b42436ebd78e5/data/cli_run.png)

**Test Output**
![Test Example Console Output](https://github.com/nogibjj/Ramil-Script-Command-Line-Tool-Rust/blob/ca094cb4c95caa351732d4c8fd9b42436ebd78e5/data/test_run.png)

**Check Output**
![Test Example Console Output](https://github.com/nogibjj/Ramil-Script-Command-Line-Tool-Rust/blob/ca094cb4c95caa351732d4c8fd9b42436ebd78e5/data/check.png)

