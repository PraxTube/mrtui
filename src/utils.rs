pub fn div_up(number: usize, divider: usize) -> usize {
    let result = number / divider;

    if number % divider == 0 {
        return result;
    } else {
        return result + 1;
    }
}
