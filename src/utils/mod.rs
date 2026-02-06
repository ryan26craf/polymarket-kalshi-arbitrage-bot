use rust_decimal::Decimal;

pub fn format_percentage(value: Decimal) -> String {
    format!("{:.2}%", value * Decimal::from(100))
}

pub fn format_currency(value: Decimal) -> String {
    format!("${:.2}", value)
}

pub fn calculate_roi(profit: Decimal, investment: Decimal) -> Decimal {
    if investment.is_zero() {
        Decimal::ZERO
    } else {
        (profit / investment) * Decimal::from(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_percentage() {
        let value = Decimal::try_from(0.05).unwrap();
        assert_eq!(format_percentage(value), "5.00%");
    }

    #[test]
    fn test_format_currency() {
        let value = Decimal::try_from(123.45).unwrap();
        assert_eq!(format_currency(value), "$123.45");
    }

    #[test]
    fn test_calculate_roi() {
        let profit = Decimal::try_from(50.0).unwrap();
        let investment = Decimal::try_from(1000.0).unwrap();
        let roi = calculate_roi(profit, investment);
        assert_eq!(roi, Decimal::try_from(5.0).unwrap());
    }
}
