fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_from_u32() {
        assert_eq!(Output::from(1), Output::Number(1));
        assert_eq!(Output::from(3), Output::Crackle);
        assert_eq!(Output::from(5), Output::Pop);
        assert_eq!(Output::from(15), Output::CracklePop);
        assert_eq!(Output::from(1).to_string(), "1");
        assert_eq!(Output::from(3).to_string(), "Crackle");
        assert_eq!(Output::from(5).to_string(), "Pop");
        assert_eq!(Output::from(15).to_string(), "CracklePop");
    }

    #[test]
    fn test_output_display() {
        assert_eq!(Output::from(1), Output::Number(1));
        assert_eq!(Output::from(3), Output::Crackle);
        assert_eq!(Output::from(5), Output::Pop);
        assert_eq!(Output::from(15), Output::CracklePop);
        assert_eq!(Output::from(1).to_string(), "1");
        assert_eq!(Output::from(3).to_string(), "Crackle");
        assert_eq!(Output::from(5).to_string(), "Pop");
        assert_eq!(Output::from(15).to_string(), "CracklePop");
    }
}
