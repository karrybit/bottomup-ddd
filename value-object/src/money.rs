use rust_decimal::Decimal;
use std::ops::Add;

struct Money {
    amount: Decimal,
    currency: String,
}

impl Money {
    fn new(amount: Decimal, currency: String) -> Self {
        if currency.is_empty() {
            panic!("currency is empty")
        } else {
            Self { amount, currency }
        }
    }
}

impl Add for Money {
    type Output = Self;
    fn add(self, other: Money) -> Self::Output {
        if self.currency != other.currency {
            panic!(
                "invalid currency. self: {}, other: {}",
                self.currency, other.currency
            )
        }
        let new_amount = self.amount + other.amount;
        Self::new(new_amount, self.currency)
    }
}

#[test]
fn test_adding_money() {
    let my_money = Money::new(Decimal::new(1000, 10), "JPY".to_string());
    let allowance = Money::new(Decimal::new(3000, 10), "JPY".to_string());
    let result = my_money + allowance;
    assert_eq!(Decimal::new(4000, 10), result.amount);
}
