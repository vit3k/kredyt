mod loan;
mod utils;

use crate::utils::format_currency;
use chrono::NaiveDate;

fn main() {
    let start_date = NaiveDate::from_ymd_opt(2025, 10, 1).unwrap();

    let loan_amount = 1_076_000.0;
    let annual_interest_rate = 6.20;
    let installment_count = 12 * 30;
    let overpayment = 5000.0;

    let comparison = loan::compare_simulations(
        loan_amount,
        annual_interest_rate,
        installment_count,
        overpayment,
        start_date,
        1.0,
        24,
        3.5,
    );

    loan::display_simulation_result(&comparison.without_overpayments, "KREDYT BEZ NADPŁAT");
    println!("\n");
    loan::display_simulation_result(
        &comparison.with_overpayments,
        &format!(
            "KREDYT Z NADPŁATĄ {} ZŁ/MIESIĄC",
            format_currency(overpayment)
        ),
    );

    println!("\n=== PORÓWNANIE I KORZYŚCI ===");
    println!("Data rozpoczęcia: {}", start_date.format("%d.%m.%Y"));
    println!(
        "Data zakończenia bez nadpłat: {}",
        comparison.without_overpayments.end_date.format("%d.%m.%Y")
    );
    println!(
        "Data zakończenia z nadpłatami: {}",
        comparison.with_overpayments.end_date.format("%d.%m.%Y")
    );

    let days_difference = (comparison.without_overpayments.end_date
        - comparison.with_overpayments.end_date)
        .num_days();
    println!(
        "Wcześniejsze zakończenie o: {} dni (~{:.1} lat)",
        days_difference,
        days_difference as f64 / 365.25
    );
    println!(
        "Całkowita nadpłata: {} zł",
        format_currency(comparison.total_overpayment)
    );
    println!(
        "Oszczędność na odsetkach: {} zł",
        format_currency(comparison.interest_savings)
    );
    println!(
        "Skrócenie kredytu: {} lat {} miesięcy",
        comparison.loan_reduction_years, comparison.loan_reduction_months
    );
}
