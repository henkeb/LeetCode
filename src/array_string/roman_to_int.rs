struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut skip = false;
        for (i, current) in s.chars().enumerate() {
            if !skip {
                match (current, s.chars().nth(i + 1)) {
                    (current, Some(next))
                        if Self::str_to_value(&current.to_string())
                            < Self::str_to_value(&next.to_string()) =>
                    {
                        sum += Self::str_to_value(&format!("{current}{next}"));
                        skip = true;
                    }
                    (current, Some(next))
                        if Self::str_to_value(&current.to_string())
                            >= Self::str_to_value(&next.to_string()) =>
                    {
                        sum += Self::str_to_value(&current.to_string())
                    }
                    (current, _) => sum += Self::str_to_value(&format!("{current}")),
                }
            } else {
                skip = false;
            }
        }
        sum
    }

    fn str_to_value(substr: &str) -> i32 {
        match substr {
            "I" => 1,
            "IV" => 4,
            "V" => 5,
            "IX" => 9,
            "X" => 10,
            "XL" => 40,
            "L" => 50,
            "XC" => 90,
            "C" => 100,
            "CD" => 400,
            "D" => 500,
            "CM" => 900,
            "M" => 1000,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman3() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn roman58() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn roman1994() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
