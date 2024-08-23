// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given a string expression representing an expression of fraction addition and subtraction, return the calculation result in string format.
//
// The final result should be an irreducible fraction. If your final result is an integer, change it to the format of a fraction that has a denominator 1. So in this case, 2 should be converted to 2/1.
use core::fmt::Display;

#[derive(Clone, Copy, Debug)]
struct Fraction {
    pub numerator: i32,
    pub denominator: i32,
    pub sign: i32,
}
impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.sign * self.numerator, self.denominator)
    }
}
impl Fraction {
    pub fn new(numerator: i32, denominator: i32, sign: i32) -> Self {
        let gcd = Self::gcd(numerator, denominator);
        Fraction {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
            sign,
        }
    }
    fn gcd(numerator: i32, denominator: i32) -> i32 {
        if denominator == 0 {
            numerator
        } else {
            Self::gcd(denominator, numerator % denominator)
        }
    }
    fn lcm(a: i32, b: i32) -> i32 {
        a * b / Self::gcd(a, b)
    }
    fn add(&self, other: &Self) -> Self {
        let lcm = Self::lcm(self.denominator, other.denominator);
        let new_num = self.sign * self.numerator * (lcm / self.denominator)
            + other.sign * other.numerator * (lcm / other.denominator);
        let sign = if new_num < 0 { -1 } else { 1 };
        Self::new(new_num.abs(), lcm, sign)
    }
}
struct Solution;
impl Solution {
    pub fn fraction_addition(mut expression: String) -> String {
        let mut fractions: Vec<Fraction> = Vec::new();
        match expression.chars().nth(0) {
            None => return "0".to_string(),
            Some('-') => (),
            Some(_) => expression.insert(0, '+'),
        }

        for (i, ch) in expression.chars().enumerate() {
            if ch == '/' {
                // get numerator and sign
                let mut j = i - 1;
                let mut sign = 0;
                let mut numerator = 0i32;
                while let Some(c) = expression.chars().nth(j) {
                    match c {
                        '+' => {
                            sign = 1;
                            break;
                        }
                        '-' => {
                            sign = -1;
                            break;
                        }
                        '0'..='9' => {
                            numerator +=
                                10i32.pow((i - j - 1) as u32) * c.to_digit(10).unwrap() as i32;
                        }
                        _ => (),
                    }
                    j -= 1;
                }
                // get denominator
                let mut denominator = 0i32;
                j = i + 1;
                while let Some(c) = expression.chars().nth(j) {
                    match c {
                        '0'..='9' => {
                            denominator = denominator * 10 + c.to_digit(10).unwrap() as i32;
                        }
                        _ => break,
                    }
                    j += 1;
                }
                fractions.push(Fraction::new(numerator, denominator, sign));
            }
        }
        fractions
            .iter()
            .fold(Fraction::new(0, 1, 1), |acc, x| acc.add(x))
            .to_string()
    }
}

// Example 1:
//
// Input: expression = "-1/2+1/2"
// Output: "0/1"
//
// Example 2:
//
// Input: expression = "-1/2+1/2+1/3"
// Output: "1/3"
//
// Example 3:
//
// Input: expression = "1/3-1/2"
// Output: "-1/6"
