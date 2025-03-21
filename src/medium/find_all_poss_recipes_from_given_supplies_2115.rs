use std::collections::{HashMap, HashSet};

// You have information about n different recipes. You are given a string array recipes and a 2D string array ingredients. The ith recipe has the name recipes[i], and you can create it if you have all the needed ingredients from ingredients[i]. A recipe can also be an ingredient for other recipes, i.e., ingredients[i] may contain a string that is in recipes.
//
// You are also given a string array supplies containing all the ingredients that you initially have, and you have an infinite supply of all of them.
//
// Return a list of all the recipes that you can create. You may return the answer in any order.
//
// Note that two recipes may contain each other in their ingredients.
//
//
//
// Example 1:
//
// Input: recipes = ["bread"], ingredients = [["yeast","flour"]], supplies = ["yeast","flour","corn"]
// Output: ["bread"]
// Explanation:
// We can create "bread" since we have the ingredients "yeast" and "flour".
// Example 2:
//
// Input: recipes = ["bread","sandwich"], ingredients = [["yeast","flour"],["bread","meat"]], supplies = ["yeast","flour","meat"]
// Output: ["bread","sandwich"]
// Explanation:
// We can create "bread" since we have the ingredients "yeast" and "flour".
// We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".
// Example 3:
//
// Input: recipes = ["bread","sandwich","burger"], ingredients = [["yeast","flour"],["bread","meat"],["sandwich","meat","bread"]], supplies = ["yeast","flour","meat"]
// Output: ["bread","sandwich","burger"]
// Explanation:
// We can create "bread" since we have the ingredients "yeast" and "flour".
// We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".
// We can create "burger" since we have the ingredient "meat" and can create the ingredients "bread" and "sandwich".
//
//
// Constraints:
//
// n == recipes.length == ingredients.length
// 1 <= n <= 100
// 1 <= ingredients[i].length, supplies.length <= 100
// 1 <= recipes[i].length, ingredients[i][j].length, supplies[k].length <= 10
// recipes[i], ingredients[i][j], and supplies[k] consist only of lowercase English letters.
// All the values of recipes and supplies combined are unique.
// Each ingredients[i] does not contain any duplicate values.
struct Solution;
impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut supplies: HashSet<String> = HashSet::from_iter(supplies);
        let mut missing_ingredients: HashMap<String, Vec<String>> = HashMap::new();
        let mut output = Vec::new();
        for i in 0..recipes.len() {
            let mut can_be_made = true;
            for ingredient in ingredients[i].iter() {
                if !supplies.contains(ingredient) {
                    can_be_made = false;
                    missing_ingredients
                        .entry(recipes[i].clone())
                        .and_modify(|ingredients| ingredients.push(ingredient.clone()))
                        .or_insert(vec![ingredient.clone()]);
                }
            }
            let mut can_now_be_made_queue: Vec<String> = Vec::new();
            if can_be_made {
                can_now_be_made_queue.push(recipes[i].clone());
                supplies.insert(recipes[i].clone());
                output.push(recipes[i].clone());
            }
            while let Some(ingredient) = can_now_be_made_queue.pop() {
                let mut to_be_removed = Vec::new();
                for (k, v) in missing_ingredients.iter_mut() {
                    if let Some(idx) = v.iter().position(|x| *x == ingredient) {
                        v.remove(idx);
                    }

                    if v.is_empty() {
                        to_be_removed.push(k.clone());
                    }
                }
                for k in to_be_removed.iter() {
                    missing_ingredients.remove(k);
                    output.push(k.clone());
                    supplies.insert(k.clone());
                    can_now_be_made_queue.push(k.clone());
                }
            }
        }
        output
    }
}
