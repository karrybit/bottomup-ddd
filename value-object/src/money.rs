use rust_decimal::Decimal;
use std::{marker::PhantomData, ops::Add};

struct Money<T: MoneyTrait> {
    amount: Decimal,
    currency: PhantomData<T>,
}

impl<T: MoneyTrait> Money<T> {
    fn new(amount: Decimal) -> Self {
        Self {
            amount,
            currency: PhantomData::<T>,
        }
    }
}

impl<T: MoneyTrait> Add for Money<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.amount + other.amount)
    }
}

trait MoneyTrait {}
enum JPY {}
impl MoneyTrait for JPY {}
enum USD {}
impl MoneyTrait for USD {}

#[test]
fn test_adding_money() {
    let my_money = Money::<JPY>::new(Decimal::new(1000, 10));
    let allowance = Money::<JPY>::new(Decimal::new(3000, 10));
    let result = my_money + allowance;
    assert_eq!(Decimal::new(4000, 10), result.amount);
}
