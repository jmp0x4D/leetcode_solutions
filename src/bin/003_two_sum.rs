use std::collections::HashMap;

fn main() {
}

#[allow(dead_code)]
// Time: O(n) since in the worst case it has to traverse the entire list
// Space: O(n) in the worst case it has to add the entire array to the hashmap before it finds a match
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut visited_nums: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        match visited_nums.get(&(target - nums[i])) {
            // k = num, v = index in nums
            None => visited_nums.insert(nums[i], i as i32),
            Some(x) => return vec![*x, i as i32]
        };
    }
    vec![]
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn basic() {
        assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3,2,4], 6), vec![1, 2]);
    }

    #[test]
    fn no_match() {
        assert_eq!(two_sum(vec![2,7,11,15], 10), vec![])
    }
   
    #[test]
    fn sum_same_number_at_start() {
        assert_eq!(two_sum(vec![3,3], 6), vec![0, 1]);
    }

    #[test]
    fn sum_same_number_in_middle() {
        assert_eq!(two_sum(vec![2,5,5,11], 10), vec![1, 2]);
    }
}
