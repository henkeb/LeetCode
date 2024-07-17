// Solution has
// Time complexity: O(log(n))
// Space complexity: O(1)
//
// A web developer needs to know how to design a web page's size. So, given a specific rectangular web page’s area, your job by now is to design a rectangular web page, whose length L and width W satisfy the following requirements:
//
//     The area of the rectangular web page you designed must equal to the given target area.
//     The width W should not be larger than the length L, which means L >= W.
//     The difference between length L and width W should be as small as possible.
//
// Return an array [L, W] where L and W are the length and width of the web page you designed in sequence.
struct Solution;
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let w = (1..=(area as f64).sqrt().floor() as i32)
            .rev()
            .find(|val| area % val == 0)
            .unwrap();
        let l = area / w;
        vec![l, w]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::construct_rectangle(37), vec![37, 1]);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::construct_rectangle(122122), vec![427, 286]);
    }
}
