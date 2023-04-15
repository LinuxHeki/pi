use rug::{Float, ops::Pow, float::Round};
pub struct Pi {
    pub value: Float,
    pub digits: u32,
    pub precision: u32
}

impl Pi {
    pub fn new(precision: u32) -> Self {
        Self { value: Float::with_val(precision, 0.0), digits: 0, precision }
    }

    pub fn next_digit(&mut self) -> u32 {
        let s = Float::with_val(self.precision, 16);
        let s1 = Float::with_val(self.precision, 4);
        let s2 = Float::with_val(self.precision, 2);
        let s3 = Float::with_val(self.precision, 1);

        let d = self.digits * 8;
        self.value += s.pow(-(self.digits as i32)) * (s1/(d+1) - s2/(d+4) - s3.clone()/(d+5) - s3/(d+6));

        let digit = Float::with_val(self.precision, 10);
        let integer = (&self.value * digit.pow(self.digits)).to_integer_round(Round::Down).unwrap().0;
        self.digits += 1;

        (integer % 10u32).to_u32().unwrap()
    }
}