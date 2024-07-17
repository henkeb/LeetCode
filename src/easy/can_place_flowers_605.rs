// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
struct Solution;
impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut planted_flowers = 0;
        for i in 0..flowerbed.len() {
            if flowerbed[i] == 0 {
                let left = i.checked_sub(1);
                let right = flowerbed.get(i + 1);
                match (flowerbed.get(left.unwrap_or(flowerbed.len())), right) {
                    (None, None) => {
                        if flowerbed[i] == 0 {
                            planted_flowers += 1;
                            break;
                        }
                    }
                    (None, Some(right)) => {
                        if *right == 0 {
                            planted_flowers += 1;
                            flowerbed[i] = 1;
                        }
                    }
                    (Some(left), None) => {
                        if *left == 0 {
                            planted_flowers += 1;
                            flowerbed[i] = 1;
                        }
                    }
                    (Some(left), Some(right)) => {
                        if *left == 0 && *right == 0 {
                            planted_flowers += 1;
                            flowerbed[i] = 1;
                        }
                    }
                }
            }
        }
        planted_flowers >= n
    }
}
