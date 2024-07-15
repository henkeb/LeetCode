// Solution has
// Time complexity: O(m*n)
// Space complexity: O(m*n)
//
// An image smoother is a filter of the size 3 x 3 that can be applied to each cell of an image by rounding down the average of the cell and the eight surrounding cells (i.e., the average of the nine cells in the blue smoother). If one or more of the surrounding cells of a cell is not present, we do not consider it in the average (i.e., the average of the four cells in the red smoother).
struct Solution;
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // top left corner first (x,y) x col, y row
        const POINTS: [(i32, i32); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
        ];
        let y_len = img.len() as i32;
        let x_len = img[0].len() as i32;
        let mut smoothed_img: Vec<Vec<i32>> = Vec::new();
        for (y, row) in img.iter().enumerate().map(|(y, val)| (y as i32, val)) {
            let mut smoothed_row: Vec<i32> = Vec::new();
            for (x, val) in row.iter().enumerate().map(|(x, val)| (x as i32, val)) {
                let (mut sum, mut count): (i32, i32) = (*val, 1);
                for (dx, dy) in POINTS.iter() {
                    if (x + dx >= 0 && x + dx < x_len) && (y + dy >= 0 && y + dy < y_len) {
                        sum += img[(y + dy) as usize][(x + dx) as usize];
                        count += 1;
                    }
                }
                smoothed_row.push(sum / count);
            }
            smoothed_img.push(smoothed_row);
        }
        smoothed_img
    }
}
