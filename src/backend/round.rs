

pub trait RoundD {
    fn roundd(self, decimal: i32) -> Self;
}

impl RoundD for f64 {
    fn roundd(self, decimal: i32) -> Self {
        let factor = 10f64.powi(decimal);
        (self * factor).round() / factor
    }
}