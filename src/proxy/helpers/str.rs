use unicode_segmentation::UnicodeSegmentation;

pub fn str_len(value: &str) -> usize {
    value.graphemes(true).count()
}

#[cfg(test)]
mod test {
    use crate::helpers::str::str_len;

    #[test]
    fn test_str_len() {
        // core::str::String
        assert_eq!(2, "é".len()); // Actual chars: 1
        assert_eq!(13, "Halló heimur".len()); // Actual chars: 12
        assert_eq!(3, "é".len()); // Actual chars: 1
        assert_eq!(12, "hello world!".len()); // Actual chars: 12
        assert_eq!(16, "apple juice 🍏".len()); // Actual chars: 13
        assert_eq!(30, "ラウトは難しいです！".len()); // Actual chars: 10

        // core::str::Chars
        assert_eq!(1, "é".chars().count());
        assert_eq!(2, "é".chars().count());
        assert_eq!(12, "Halló heimur".chars().count());
        assert_eq!(12, "hello world!".chars().count());
        assert_eq!(13, "apple juice 🍏".chars().count());
        assert_eq!(10, "ラウトは難しいです！".chars().count());

        // unicode_segmentation::UnicodeSegmentation
        assert_eq!(1, str_len("é"));
        assert_eq!(1, str_len("é"));
        assert_eq!(12, str_len("Halló heimur"));
        assert_eq!(12, str_len("hello world!"));
        assert_eq!(13, str_len("apple juice 🍏"));
        assert_eq!(10, str_len("ラウトは難しいです！"));
    }
}
