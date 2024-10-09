use std::collections::HashMap;

fn main() {
}

#[allow(dead_code)]
// Time Complexity: O(s+t): only iterates through t & s once
// Size Complexity O(s+t): has to store s + t completely to validate
fn is_anagram(s: String, t: String) -> bool {
    if s.len() == t.len() {
        let mut count_s: HashMap<char, i32> = HashMap::new();
        let mut count_t: HashMap<char, i32> = HashMap::new();
        for i in s.chars() {
            match count_s.get(&i) {
                None => count_s.insert(i, 1),
                Some(x) => count_s.insert(i, x + 1),
            };
        }
        for i in t.chars() {
            match count_t.get(&i) {
                None => count_t.insert(i, 1),
                Some(x) => count_t.insert(i, x + 1),
            };
        }
        if count_s == count_t {
            return true
        };

    }
    false
}


// Space Complexity O(s): has to store s + t completely to validate
// TODO: Why is this space complexity so much worse?
// fn is_anagram(s: String, t: String) -> bool {
//     if s.chars().count() == t.chars().count() {
//         let mut count: HashMap<char, i32> = HashMap::new();
//         for i in s.chars() {
//             match count.get(&i) {
//                 None => count.insert(i, 1),
//                 Some(x) => count.insert(i, x + 1),
//             };
//         }
//         for i in t.chars() {
//             match count.get(&i) {
//                 None => return false,
//                 Some(1) => count.remove(&i),
//                 Some(x) => count.insert(i, x - 1),
//             };
//         }
//         if count.is_empty() {
//             return true
//         };
//
//     }
//     false
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(is_anagram(String::from("anagram"), String::from("nagaram")), true)
    }
   
    #[test]
    fn ex2() {
        assert_eq!(is_anagram(String::from("rat"), String::from("cat")), false)
    }

    #[test] // Realized that compiler will check for type errors, but developer is responsible for
            // handling every possible variation within specified type
    fn ex3() {
        assert_eq!(is_anagram(String::from("ab"), String::from("a")), false)
    }
}
