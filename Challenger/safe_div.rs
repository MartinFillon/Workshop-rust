#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: f64,
    divisor: f64,
}

pub fn safe_divide(a: f64, b: f64) -> Result<f64, DivisionError> {
    match b {
        0.0 => Err(DivisionError::DivideByZero),
        _ => match a / b {
            result if result.is_infinite() => Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: a,
                divisor: b,
            })),
            result => Ok(result),
        },
    }
}
