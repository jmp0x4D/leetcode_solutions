use std::collections::HashSet;

fn main() {
}

#[allow(dead_code)]
//Has to traverse entire list in worst case: O(n)
fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut visited_nums: HashSet<i32> = HashSet::new();

    for i in nums {
        match visited_nums.get(&i) {
            None => visited_nums.insert(i),
            Some(_x) => return true
        };
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
