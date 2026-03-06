pub fn repeat(count: isize, positive: &str, negative: &str) -> String {
    if count >= 0 {
        positive.repeat(count.abs() as usize)
    } else {
        negative.repeat(count.abs() as usize)
    }
}
