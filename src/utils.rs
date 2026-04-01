/// Round an integer down to the nearest multiple of a given number.
pub(crate) fn round_down(number: u32, multiple: u32) -> u32 {
    number - (number % multiple)
}

/// Convert an integer to a string and pad out to the given number of zeroes.
pub(crate) fn pad(number: u32, digits: usize) -> String {
    if digits == 0 {
        return "".to_string();
    }

    format!("{:0width$}", number, width = digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounding_works() {
        assert_eq!(round_down(5, 3), 3);
        assert_eq!(round_down(21, 7), 21);
        assert_eq!(round_down(23, 7), 21);
        assert_eq!(round_down(152, 5), 150);
    }

    #[test]
    fn padding_works() {
        assert_eq!(&pad(5, 3), "005");
        assert_eq!(&pad(53, 3), "053");
        assert_eq!(&pad(505, 3), "505");
        assert_eq!(&pad(505, 2), "505");
    }
}
