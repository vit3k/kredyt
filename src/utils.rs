pub fn format_currency(value: f64) -> String {
    let rounded_value = (value * 100.0).round() / 100.0;
    let formatted_value = format!("{:.2}", rounded_value);
    let parts: Vec<&str> = formatted_value.split('.').collect();
    let integer_part = parts[0];
    let fractional_part = if parts.len() > 1 { parts[1] } else { "00" };

    let mut formatted_integer = String::new();
    for (i, c) in integer_part.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            formatted_integer.push(' ');
        }
        formatted_integer.push(c);
    }

    formatted_integer = formatted_integer.chars().rev().collect();
    format!("{}{}{}", formatted_integer, ',', fractional_part)
}
