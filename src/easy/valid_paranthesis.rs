// Solution has
// Time complexity: O(n)
// Space complexity: O(n)

// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
// An input string is valid if:
//
//     Open brackets must be closed by the same type of brackets.
//     Open brackets must be closed in the correct order.
//     Every close bracket has a corresponding open bracket of the same type.
fn check_if_open(c: &char) -> bool {
    *c == ')' || *c == '}' || *c == ']'
}

fn check_matching(closing: &char, open: &char) -> bool {
    matches!((closing, open), ('{', '}') | ('[', ']') | ('(', ')'))
}

struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                open if check_if_open(&open) => match stack.pop() {
                    Some(closing) => {
                        if check_matching(&closing, &open) {
                            continue;
                        }
                        return false;
                    }
                    _ => return false,
                },
                _ => stack.push(c),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_valid("(]".to_string()));
    }
}
