use std::fmt;

#[derive(Debug, PartialEq)]
enum Output {
    Crackle,
    Pop,
    CracklePop,
    Number(u32),
}

impl From<u32> for Output {
    fn from(num: u32) -> Self {
        if num % 3 == 0 && num % 5 == 0 {
            Output::CracklePop
        } else if num % 3 == 0 {
            Output::Crackle
        } else if num % 5 == 0 {
            Output::Pop
        } else {
            Output::Number(num)
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Output::Crackle => write!(f, "Crackle"),
            Output::Pop => write!(f, "Pop"),
            Output::CracklePop => write!(f, "CracklePop"),
            Output::Number(n) => write!(f, "{}", n),
        }
    }
}

fn main() {
    for num in 1..=100 {
        println!("{}", Output::from(num));
    }
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
