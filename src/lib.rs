// src/lib.rs

// Function to calculate the monthly interest rate
pub fn calculate_monthly_rate(annual_rate: f64) -> f64 {
    annual_rate / 12.0
}

// Function to calculate the discount factor for the loan
pub fn calculate_discount_factor(monthly_rate: f64, periods: i32) -> f64 {
    if monthly_rate == 0.0 {
        1.0  // If there's no interest rate, the discount factor doesn't apply
    } else {
        1.0 - (1.0 + monthly_rate).powi(-periods)
    }
}

// Function to calculate the monthly loan payment
pub fn get_monthly_payment(principal: f64, rate: f64, periods: i32) -> f64 {
    let monthly_rate = calculate_monthly_rate(rate);
    
    // Handle the case where interest rate is zero
    if rate == 0.0 {
        principal / periods as f64
    } else {
        let discount_factor = calculate_discount_factor(monthly_rate, periods);
        (principal * monthly_rate) / discount_factor
    }
}

// Unit tests for the above functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_monthly_rate() {
        // Test with a typical annual rate
        assert_eq!(calculate_monthly_rate(0.12), 0.01);

        // Test with a 0% annual rate
        assert_eq!(calculate_monthly_rate(0.0), 0.0);

        // Test with a negative annual rate
        assert_eq!(calculate_monthly_rate(-0.12), -0.01);
    }

    #[test]
    fn test_calculate_discount_factor() {
        // Test with a typical input
        let discount_factor = calculate_discount_factor(0.01, 12);

        // Test with zero monthly rate (discount factor should be 1.0)
        assert_eq!(calculate_discount_factor(0.0, 12), 1.0);
    }

    #[test]
    fn test_get_monthly_payment() {
        // Test with typical inputs
        let payment = get_monthly_payment(100000.0, 0.05, 360);
        assert!((payment - 536.82).abs() < 0.01);  // Margin of error

        // Test with zero interest rate (adjusted logic)
        assert_eq!(get_monthly_payment(100000.0, 0.0, 360), 277.77777777777777);  // Principal / periods
    }
}