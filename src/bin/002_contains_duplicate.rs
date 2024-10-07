fn main() {
}

#[allow(dead_code)]
// fn contains_duplicate(nums: Vec<i32>) -> bool { // Exceeded Time Limit
//     let mut values_seen = vec![];
//     for i in nums {
//         if values_seen.contains(&i)  {
//             return true
//         }
//         else { values_seen.push(i); }
//     } 
//     false
// }

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut sorted_nums = nums.clone(); //cloning is faster?
        sorted_nums.sort();
    for i in 0..sorted_nums.len() - 1 {
        if sorted_nums[i] == sorted_nums[i + 1] {
            return true
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(contains_duplicate(vec![1,2,3,1]), true)
    }

    #[test]
    fn ex2() {
        assert_eq!(contains_duplicate(vec![1,2,3,4]), false)
    }

    #[test]
    fn ex3() {
        assert_eq!(contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true)
    }
}
