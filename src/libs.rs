pub use crate::account::*;
pub use crate::accounts::*;
pub use crate::transaction::*;
pub use crate::transaction_logs::*;

use lazy_static::lazy_static;
use regex::Regex;
use rust_decimal::prelude::*;
use std::collections::HashSet;

pub type Amount = Decimal;
pub type ClientId = u16;
pub type Disputes = HashSet<TransactionId>;
pub type TransactionId = u32;

lazy_static! {
    // We can safely unwrap here as this could only panic at compile time
    static ref TRUNCATE_TO_FOUR: Regex = Regex::new(r"(^-?\d+$|^-?\d+[.]\d{0,4}).*").unwrap();
}

pub fn format(decimal: Decimal) -> String {
    TRUNCATE_TO_FOUR
        .captures(&decimal.to_string())
        .and_then(|captures| captures.get(1))
        .and_then(|decimal| Decimal::from_str(decimal.as_str()).ok())
        .map(|decimal| decimal.normalize())
        .unwrap_or(Decimal::ZERO)
        .to_string() // We can safely unwrap here as the `unwrap_or()` covers us
}

#[cfg(test)]
mod test_format {
    use super::*;

    #[test]
    fn negitive_one() {
        let formatted = format(Decimal::NEGATIVE_ONE);
        assert!(formatted == "-1");
    }

    #[test]
    fn negative_zero() {
        let mut negative_zero = Decimal::ZERO.clone();
        negative_zero.set_sign_negative(true);

        let formatted = format(negative_zero);
        assert!(formatted == "0");
    }

    #[test]
    fn zero() {
        let formatted = format(Decimal::ZERO);
        assert!(formatted == "0");
    }

    #[test]
    fn one() {
        let formatted = format(Decimal::ONE);
        assert!(formatted == "1");
    }

    #[test]
    fn negative_three_ooooh() {
        let mut negative_two_dot = Decimal::from_str("3.00000").unwrap();
        negative_two_dot.set_sign_negative(true);

        let formatted = format(negative_two_dot);
        assert!(formatted == "-3");
    }

    #[test]
    fn negative_three_ooooh_three() {
        let mut negative_two_dot = Decimal::from_str("3.00003").unwrap();
        negative_two_dot.set_sign_negative(true);

        let formatted = format(negative_two_dot);
        assert!(formatted == "-3");
    }

    #[test]
    fn negative_pi() {
        let mut negative_pi = Decimal::PI.clone();
        negative_pi.set_sign_negative(true);

        let formatted = format(negative_pi);
        assert!(formatted == "-3.1415");
    }

    #[test]
    fn negative_two_dot() {
        let mut negative_two_dot = Decimal::from_str("2.").unwrap();
        negative_two_dot.set_sign_negative(true);

        let formatted = format(negative_two_dot);
        assert!(formatted == "-2");
    }

    #[test]
    fn two_dot() {
        let formatted = format(Decimal::from_str("2.").unwrap());
        assert!(formatted == "2");
    }

    #[test]
    fn pi() {
        let formatted = format(Decimal::PI);
        assert!(formatted == "3.1415");
    }

    #[test]
    fn three_ooooh_three() {
        // acid techno!
        let formatted = format(Decimal::from_str("3.00003").unwrap());
        assert!(formatted == "3");
    }

    #[test]
    fn three_ooooh() {
        let formatted = format(Decimal::from_str("3.00000").unwrap());
        assert!(formatted == "3");
    }
}
