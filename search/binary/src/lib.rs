use std::fmt::Debug;

// List of items should be sorted.
fn search<'a, T: Ord + Clone + Debug>(items: &[T], item: T) -> Option<T> {
    if items.len() == 0 {
        return None;
    }

    let mid = items.len() / 2;
    println!("Comparing items: {:?} with search item {:?}", items[mid], item);
    if items[mid] == item {
        return Some(items[mid].clone());
    }

    if item < items[mid] {
        search(&items[..mid], item)
    } else {
        search(&items[mid+1..], item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::{Ordering, Eq, PartialEq, PartialOrd};

    #[derive(Eq, PartialOrd, Clone, Debug)]
    struct TestTuple {
        key: i32,
        value: String,
    }

    impl TestTuple {
        fn new(key: i32, value: &str) -> Self {
            Self {
                key,
                value: value.to_string(),
            }
        }
    }

    impl PartialEq for TestTuple {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    impl Ord for TestTuple {
        fn cmp(&self, other: &Self) -> Ordering {
            println!("Self: {:?}, Other: {:?}", self.key, other.key);
            self.key.cmp(&other.key)
        }
    }

    #[test]
    fn test_basic_cases() {
        let input = vec!(
            TestTuple::new(1, "Test String - 1"),
            TestTuple::new(2, "Test String - 2"),
            TestTuple::new(3, "Test String - 3"),
            TestTuple::new(4, "Test String - 4"),
            TestTuple::new(5, "Test String - 5"),
            TestTuple::new(6, "Test String - 6"));
        assert_eq!(search(&input, TestTuple::new(4, "")).unwrap().value, "Test String - 4".to_string());
        assert_eq!(search(&input, TestTuple::new(1, "")).unwrap().value, "Test String - 1".to_string());
        assert_eq!(search(&input, TestTuple::new(6, "")).unwrap().value, "Test String - 6".to_string());
        assert_eq!(search(&input, TestTuple::new(10, "")), None);
    }
}
