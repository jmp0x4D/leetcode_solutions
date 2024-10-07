fn main() {
}

#[allow(dead_code)]
//Two loops -> O(n^2)
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
       for i in 0..nums.len() - 1 { // range is exclusive, -1 keeps j in bounds
        for j in i + 1..nums.len() { // i + ... prevents same number sum
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];            
            }
        }
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
