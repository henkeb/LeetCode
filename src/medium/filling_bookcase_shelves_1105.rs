// Solution has
// Time complexity: O(n²)
// Space complexity: O(n)
//
// You are given an array books where books[i] = [thicknessi, heighti] indicates the thickness and height of the ith book. You are also given an integer shelfWidth.
//
// We want to place these books in order onto bookcase shelves that have a total width shelfWidth.
//
// We choose some of the books to place on this shelf such that the sum of their thickness is less than or equal to shelfWidth, then build another level of the shelf of the bookcase so that the total height of the bookcase has increased by the maximum height of the books we just put down. We repeat this process until there are no more books to place.
//
// Note that at each step of the above process, the order of the books we place is the same order as the given sequence of books.
//
//     For example, if we have an ordered list of 5 books, we might place the first and second book onto the first shelf, the third book on the second shelf, and the fourth and fifth book on the last shelf.
//
// Return the minimum possible height that the total bookshelf can be after placing shelves in this manner.

struct Solution;
impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp = vec![i32::MAX; books.len() + 1];
        //base case
        dp[0] = 0;
        for i in 1..=books.len() {
            let mut current_shelf_thickness = 0;
            let mut current_shelf_height = 0;
            for j in (0..i).rev() {
                let current_book_thickness = books[j][0];
                let current_book_height = books[j][1];
                if current_shelf_thickness + current_book_thickness > shelf_width {
                    break;
                }
                current_shelf_thickness += current_book_thickness;
                current_shelf_height = current_shelf_height.max(current_book_height);

                let current_arrangement_height = dp[j] + current_shelf_height;
                dp[i] = dp[i].min(current_arrangement_height);
            }
        }
        dp[books.len()]
    }
}
