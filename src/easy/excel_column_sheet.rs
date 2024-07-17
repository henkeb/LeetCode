// Solution has
// Time complexity: O(log26(column_number))
// Space complexity: O(1)
//
// Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.
//
// For example:
//
// A -> 1
// B -> 2
// C -> 3
// ...
// Z -> 26
// AA -> 27
// AB -> 28
// ...
//
const Z: i32 = 26;
const ASCII_START: u8 = 65;
struct Solution;
impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut excel: String = "".to_string();
        while column_number > 0 {
            column_number -= 1;
            excel.push(((column_number % Z) as u8 + ASCII_START) as char);
            column_number /= Z;
        }

        excel.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::convert_to_title(1), "A");
    }

    #[test]
    fn it_works2() {
        assert_eq!(Solution::convert_to_title(28), "AB");
    }

    #[test]
    fn it_works3() {
        assert_eq!(Solution::convert_to_title(701), "ZY");
    }

    #[test]
    fn it_works4() {
        assert_eq!(Solution::convert_to_title(2147483647), "FXSHRXW");
    }
}
