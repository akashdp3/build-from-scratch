mod bloom_filter;

use bloom_filter::BloomFilter;

// Username Availability Checker using Bloom Filter
pub struct UsernameChecker {
    taken_usernames: BloomFilter,
}

impl UsernameChecker {
    pub fn new(expected_users: usize) -> Self {
        // Size calculation for good performance with low false positives
        let size = expected_users * 10;
        Self {
            taken_usernames: BloomFilter::new(size),
        }
    }

    pub fn register_username(&mut self, username: &str) {
        let normalized = username.to_lowercase();
        self.taken_usernames.insert(&normalized);
    }

    pub fn is_username_taken(&self, username: &str) -> bool {
        let normalized = username.to_lowercase();
        self.taken_usernames.contains(&normalized)
    }

    pub fn is_username_available(&self, username: &str) -> bool {
        !self.is_username_taken(username)
    }

    pub fn clear_all(&mut self) {
        self.taken_usernames.clear();
    }
}

fn main() {
    let mut bf = BloomFilter::new(1000);

    bf.insert("item");
    println!("item: {:?}", bf.contains("item"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_checker_all_usernames_available() {
        let checker = UsernameChecker::new(100);

        assert!(checker.is_username_available("john"));
        assert!(checker.is_username_available("alice"));
        assert!(checker.is_username_available("bob"));
    }

    #[test]
    fn test_register_single_username() {
        let mut checker = UsernameChecker::new(100);

        checker.register_username("john");

        assert!(checker.is_username_taken("john"));
        assert!(!checker.is_username_available("john"));
    }

    #[test]
    fn test_register_multiple_usernames() {
        let mut checker = UsernameChecker::new(100);

        checker.register_username("john");
        checker.register_username("alice");
        checker.register_username("bob");

        assert!(checker.is_username_taken("john"));
        assert!(checker.is_username_taken("alice"));
        assert!(checker.is_username_taken("bob"));
    }

    #[test]
    fn test_unregistered_username_is_available() {
        let mut checker = UsernameChecker::new(100);

        checker.register_username("john");

        assert!(checker.is_username_available("alice"));
        assert!(checker.is_username_available("bob"));
    }

    #[test]
    fn test_availability_inverse_of_taken() {
        let mut checker = UsernameChecker::new(100);

        checker.register_username("john");

        assert_eq!(
            checker.is_username_taken("john"),
            !checker.is_username_available("john")
        );
        assert_eq!(
            checker.is_username_taken("alice"),
            !checker.is_username_available("alice")
        );
    }

    #[test]
    fn test_false_positive_rate_low_load() {
        let mut checker = UsernameChecker::new(100);

        // Register 100 usernames
        for i in 0..100 {
            checker.register_username(&format!("user{}", i));
        }

        // Check 1000 non-existent usernames
        let mut false_positives = 0;
        for i in 100..1100 {
            if checker.is_username_taken(&format!("user{}", i)) {
                false_positives += 1;
            }
        }

        let fp_rate = false_positives as f64 / 1000.0;
        println!("False positive rate: {:.2}%", fp_rate * 100.0);

        // Should have low false positive rate
        assert!(
            fp_rate < 0.1,
            "False positive rate too high: {:.2}%",
            fp_rate * 100.0
        );
    }
}
