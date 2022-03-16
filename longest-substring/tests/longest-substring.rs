use longest_substring::Solution;

#[test]
fn example1() {
    let s = "abcabcbb".to_owned();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
}

#[test]
fn example2() {
    let s = "bbbbb".to_owned();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
}

#[test]
fn example3() {
    let s = "pwwkew".to_owned();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
}
