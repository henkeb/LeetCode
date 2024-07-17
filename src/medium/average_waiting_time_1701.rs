// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// There is a restaurant with a single chef. You are given an array customers, where customers[i] = [arrivali, timei]:
//
//     arrivali is the arrival time of the ith customer. The arrival times are sorted in non-decreasing order.
//     timei is the time needed to prepare the order of the ith customer.
//
// When a customer arrives, he gives the chef his order, and the chef starts preparing it once he is idle. The customer waits till the chef finishes preparing his order. The chef does not prepare food for more than one customer at a time. The chef prepares food for customers in the order they were given in the input.
//
// Return the average waiting time of all customers. Solutions within 10-5 from the actual answer are considered accepted.
struct Solution;
impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut total_wait_time: f64 = 0.0;
        let mut time: f64 = 0.0;
        for (arrival, prep_time) in customers
            .iter()
            .map(|customer| (customer[0] as f64, customer[1] as f64))
        {
            if time < arrival {
                time = arrival;
            }

            time += prep_time;

            total_wait_time += time - arrival;
        }
        total_wait_time / customers.len() as f64
    }
}
