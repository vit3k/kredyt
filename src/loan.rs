use chrono::{Months, NaiveDate};

use crate::utils::format_currency;

#[derive(Debug, Clone)]
pub struct InstallmentInfo {
    pub installment_number: u32,
    pub installment_amount: f64,
    // pub interest_rate: f64,
    pub interest: f64,
    pub principal: f64,
    // pub overpayment: f64,
    // pub remaining_balance: f64,
    // pub date: Option<NaiveDate>,
}

// #[derive(Debug)]
// pub struct InterestRateChange {
//     pub installment: u32,
//     pub old_interest_rate: f64,
//     pub new_interest_rate: f64,
//     pub new_installment: f64,
//     pub remaining_balance: f64,
//     pub remaining_installments: u32,
// }

#[derive(Debug)]
pub struct SimulationResult {
    pub installments: Vec<InstallmentInfo>,
    // pub interest_rate_changes: Vec<InterestRateChange>,
    pub total_interest: f64,
    pub actual_installment_count: u32,
    pub loan_duration_years: u32,
    pub loan_duration_months: u32,
    pub loan_amount: f64,
    pub initial_interest_rate: f64,
    pub final_interest_rate: f64,
    pub monthly_overpayment: f64,
    pub end_date: NaiveDate,
}

#[derive(Debug)]
pub struct SimulationComparison {
    pub with_overpayments: SimulationResult,
    pub without_overpayments: SimulationResult,
    pub interest_savings: f64,
    pub loan_reduction_years: u32,
    pub loan_reduction_months: u32,
    pub total_overpayment: f64,
}

fn calculate_annuity_payment(p: f64, r: f64, n: u32) -> f64 {
    if r == 0.0 {
        return p / n as f64;
    }
    let rn = (1.0 + r).powi(n as i32);
    p * r * rn / (rn - 1.0)
}

pub fn simulate_loan(
    loan_amount: f64,
    annual_interest_rate: f64,
    installment_count: u32,
    initial_overpayment: f64,
    start_date: NaiveDate,
    interest_rate_decrease: f64,
    decrease_frequency: u32, // in months
    minimum_interest_rate: f64,
) -> SimulationResult {
    let mut r = annual_interest_rate / 100.0 / 12.0; // monthly interest rate
    let mut installment = calculate_annuity_payment(loan_amount, r, installment_count);
    let mut current_balance = loan_amount;
    let mut total_interest = 0.0;
    let mut current_interest_rate = annual_interest_rate;

    let mut installments: Vec<InstallmentInfo> = Vec::new();
    // let mut interest_rate_changes: Vec<InterestRateChange> = Vec::new();
    let mut actual_installment_count = 0;
    for current_installment in 1..=installment_count {
        if current_installment % decrease_frequency == 0 && interest_rate_decrease != 0.0 && current_interest_rate > minimum_interest_rate {
            // let old_interest_rate = current_interest_rate;
            current_interest_rate = minimum_interest_rate.max(current_interest_rate - interest_rate_decrease);
            r = current_interest_rate / 100.0 / 12.0;
            let remaining_installments = installment_count - current_installment + 1;
            installment = calculate_annuity_payment(current_balance, r, remaining_installments);

            // interest_rate_changes.push(InterestRateChange {
            //     installment: current_installment,
            //     old_interest_rate,
            //     new_interest_rate: current_interest_rate,
            //     new_installment: installment,
            //     remaining_balance: current_balance,
            //     remaining_installments,
            // });
        }
        let interest = current_balance * r;
        let principal = f64::min(installment - interest, current_balance);
        current_balance -= principal;
        let overpayment = initial_overpayment.min(current_balance);
        current_balance -= overpayment;

        let installment_info = InstallmentInfo {
            installment_number: current_installment,
            installment_amount: installment,
            // interest_rate: current_interest_rate,
            interest,
            principal,
            // overpayment,
            // remaining_balance: current_balance,
            // date: Some(start_date + Months::new(current_installment - 1)),
        };

        installments.push(installment_info);
        total_interest += interest;
        actual_installment_count = current_installment;

        if current_balance == 0.0 {
            break;
        }
    }

    SimulationResult {
        installments,
        // interest_rate_changes,
        total_interest,
        actual_installment_count,
        loan_duration_years: actual_installment_count / 12,
        loan_duration_months: actual_installment_count % 12,
        loan_amount,
        initial_interest_rate: annual_interest_rate,
        final_interest_rate: current_interest_rate,
        monthly_overpayment: initial_overpayment,
        end_date: start_date + Months::new(actual_installment_count - 1),
    }
}

pub fn compare_simulations(
    loan_amount: f64,
    annual_interest_rate: f64,
    installment_count: u32,
    initial_overpayment: f64,
    start_date: NaiveDate,
    interest_rate_decrease: f64,
    decrease_frequency: u32,
    minimum_interest_rate: f64,
) -> SimulationComparison {
    let with_overpayments = simulate_loan(
        loan_amount,
        annual_interest_rate,
        installment_count,
        initial_overpayment,
        start_date,
        interest_rate_decrease,
        decrease_frequency,
        minimum_interest_rate,
    );

    let without_overpayments = simulate_loan(
        loan_amount,
        annual_interest_rate,
        installment_count,
        0.0,
        start_date,
        interest_rate_decrease,
        decrease_frequency,
        minimum_interest_rate,
    );

    let interest_savings = without_overpayments.total_interest - with_overpayments.total_interest;
    let reduction_months = without_overpayments.actual_installment_count - with_overpayments.actual_installment_count;
    let total_overpayment = initial_overpayment * with_overpayments.actual_installment_count as f64;

    SimulationComparison {
        with_overpayments,
        without_overpayments,
        interest_savings,
        loan_reduction_years: reduction_months / 12,
        loan_reduction_months: reduction_months % 12,
        total_overpayment,
    }
}

pub fn display_simulation_result(result: &SimulationResult, title: &str) {
    println!("=== {} ===", title);
    println!("Kwota kredytu: {} zł", format_currency(result.loan_amount));
    println!("Nadpłata miesięczna: {} zł", format_currency(result.monthly_overpayment));
    println!("Rzeczywista liczba rat: {}", result.actual_installment_count);
    println!("Czas kredytu: {} lat {} miesięcy", result.loan_duration_years, result.loan_duration_months);
    println!("Data zakończenia: {}", result.end_date.format("%d.%m.%Y"));
    println!("Początkowe oprocentowanie: {:.2}%", result.initial_interest_rate);
    println!("Końcowe oprocentowanie: {:.2}%", result.final_interest_rate);
    println!("Suma odsetek: {} zł", format_currency(result.total_interest));

    // Display first and last 3 installments
    println!("\nPierwsze 3 raty:");
    for installment in result.installments.iter().take(3) {
        println!(
            "  Rata {:>3}: {} zł (odsetki: {} zł, kapitał: {} zł)",
            installment.installment_number,
            format_currency(installment.installment_amount),
            format_currency(installment.interest),
            format_currency(installment.principal)
        );
    }

    println!("Ostatnie 3 raty:");
    let start_index = result.installments.len().saturating_sub(3);
    for installment in result.installments.iter().skip(start_index) {
        println!(
            "  Rata {:>3}: {} zł (odsetki: {} zł, kapitał: {} zł)",
            installment.installment_number,
            format_currency(installment.installment_amount),
            format_currency(installment.interest),
            format_currency(installment.principal)
        );
    }
}
