// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// You are given a 0-indexed array of strings details. Each element of details provides information about a given passenger compressed into a string of length 15. The system is such that:
//
//     The first ten characters consist of the phone number of passengers.
//     The next character denotes the gender of the person.
//     The following two characters are used to indicate the age of the person.
//     The last two characters determine the seat allotted to that person.
//
// Return the number of passengers who are strictly more than 60 years old.
struct Solution;
impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .filter_map(|info| info[11..=12].parse::<i32>().ok())
            .filter(|age| *age > 60)
            .count() as i32
    }
}
