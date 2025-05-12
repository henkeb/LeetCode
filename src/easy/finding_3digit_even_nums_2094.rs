// You are given an integer array digits, where each element is a digit. The array may contain duplicates.
//
// You need to find all the unique integers that follow the given requirements:
//
// The integer consists of the concatenation of three elements from digits in any arbitrary order.
// The integer does not have leading zeros.
// The integer is even.
// For example, if the given digits were [1, 2, 3], integers 132 and 312 follow the requirements.
//
// Return a sorted array of the unique integers.
//
//
//
// Example 1:
//
// Input: digits = [2,1,3,0]
// Output: [102,120,130,132,210,230,302,310,312,320]
// Explanation: All the possible integers that follow the requirements are in the output array.
// Notice that there are no odd integers or integers with leading zeros.
// Example 2:
//
// Input: digits = [2,2,8,8,2]
// Output: [222,228,282,288,822,828,882]
// Explanation: The same digit can be used as many times as it appears in digits.
// In this example, the digit 8 is used twice each time in 288, 828, and 882.
// Example 3:
//
// Input: digits = [3,7,5]
// Output: []
// Explanation: No even integers can be formed using the given digits.
//
//
// Constraints:
//
// 3 <= digits.length <= 100
// 0 <= digits[i] <= 9
struct Solution;
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut d_map: HashMap<usize, usize> = HashMap::new();
        digits.iter().for_each(|val| {
            d_map
                .entry(*val as usize)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        let mut digits: Vec<usize> = Vec::new();
        let mut output = Vec::new();

        for mut i in (100..=998).step_by(2) {
            let num = i;
            digits.clear();
            let mut d_map_copy = d_map.clone();
            while i != 0 {
                digits.push(i % 10);
                i /= 10;
            }
            let mut ok_num = true;
            for digit in digits.iter() {
                if d_map_copy.contains_key(&digit) {
                    d_map_copy.entry(*digit).and_modify(|count| {
                        if *count != 0 {
                            *count -= 1;
                        } else {
                            ok_num = false;
                        }
                    });
                } else {
                    ok_num = false;
                    break;
                }
            }
            if ok_num {
                output.push(num as i32);
            }
        }
        output
    }
}
