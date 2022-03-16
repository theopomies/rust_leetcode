pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        s.chars()
            .fold((0, 0..0), |(longest, mut range), c| {
                if let Some(pos) = &s[range.clone()].find(c) {
                    range.start += pos + 1;
                }
                range.end += 1;
                (longest.max(range.len() as i32), range)
            })
            .0
    }
}
