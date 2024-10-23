use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Calculate the monthly payment
    CalculateMonthlyPayment {
        /// The loan principal
        principal: f64,
        /// The annual interest rate (as a decimal, e.g., 0.05 for 5%)
        rate: f64,
        /// The number of periods (months)
        periods: i32,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::CalculateMonthlyPayment { principal, rate, periods } => {
            // Ensure you have access to the loan_payment module
            let payment = loan_payment::get_monthly_payment(principal, rate, periods);
            println!("Monthly Loan Payment: {:.2}", payment);
        }
    }
}