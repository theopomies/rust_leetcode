use two_sum::Solution;

#[test]
fn example1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    assert_eq!(&result, &[0, 1]);
}

#[test]
fn example2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    assert_eq!(&result, &[1, 2]);
}

#[test]
fn example3() {
    let nums = vec![3, 3];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    assert_eq!(&result, &[0, 1]);
}
