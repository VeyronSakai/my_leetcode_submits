use std::collections::HashMap;

struct Logger {
    mp: HashMap<String, i32>,
}


/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 */
impl Logger {
    fn new() -> Self {
        Logger {
            mp: HashMap::new()
        }
    }

    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if !self.mp.contains_key(message.as_str()) {
            self.mp.insert(message, timestamp);
            return true;
        }

        let time = self.mp.get(message.as_str()).unwrap();
        return if timestamp < *time + 10 {
            false
        } else {
            self.mp.insert(message, timestamp);
            true
        };
    }
}

/**
 * Your Logger object will be instantiated and called as such:
 * let obj = Logger::new();
 * let ret_1: bool = obj.should_print_message(timestamp, message);
 */
#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::*;

    fn setup() {
        let logger = Logger::new();
    }

    #[test]
    fn hoge() {
        let mut logger = Logger::new();
        assert_eq!(logger.should_print_message(1, "foo".to_string()), true);
        assert_eq!(logger.should_print_message(2, "bar".to_string()), true);
        assert_eq!(logger.should_print_message(3, "foo".to_string()), false);
        assert_eq!(logger.should_print_message(8, "bar".to_string()), false);
        assert_eq!(logger.should_print_message(10, "foo".to_string()), false);
        assert_eq!(logger.should_print_message(11, "foo".to_string()), true);
    }
}
