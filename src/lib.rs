use std::cmp::Ord;
use std::fmt::Debug;

pub fn merge<'a, T: Ord + Copy + Debug>(first: &[T], second: &[T]) -> Vec<T> {
    // Store sorted merge set.
    let mut c: Vec<T> = vec!();
    // Keep track of index into second set.
    let mut j = 0;
    let mut i = 0;
    while let Some(a) = first.get(i) {
        match second.get(j) {
            Some(b) => {
                if a < b {
                    c.push(*a);
                    i += 1;
                } else {
                    c.push(*b);
                    j += 1;
                }
            },
            None => {
                c.push(*a);
                i += 1;
            },
        }
    };

    if j < second.len() {
        c.extend_from_slice(&second[j..]);
    }
    c
}

pub fn mergesort<T: Ord + Copy + Debug> (items: &[T]) -> Vec<T> {
    if items.len() <= 1 {
        return Vec::from(items);
    }

    let split_point = items.len() / 2;
    let front = &items[0..split_point];
    let back = &items[split_point..];
    let sorted_front = mergesort(front);
    let sorted_back = mergesort(back);
    merge(&sorted_front, &sorted_back)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_mergesort_compiles() {
    let v = vec!(1);
    assert_eq!(mergesort(v.as_slice()), vec!(1).as_slice());
  }

  #[test]
  fn test_mergesort_sorts_set_of_two() {
    let v = vec!(2, 1);
    assert_eq!(mergesort(v.as_slice()), vec!(1, 2).as_slice());
  }

  #[test]
  fn test_mergesort_sorts_even_sizes_collection() {
    let v = vec!(4, 3, 2, 1);
    assert_eq!(mergesort(v.as_slice()), vec!(1, 2, 3, 4));
  }

  #[test]
  fn test_mergesort_sorts_odd_sizes_collection() {
    let v = vec!(5, 4, 2, 3, 1);
    assert_eq!(mergesort(v.as_slice()), vec!(1, 2, 3, 4, 5));
  }

  #[test]
  fn test_mergesort_sorts_with_duplicates() {
    let v = vec!(10, 9, 8, 7, 8, 6, 8, 4, 3);
    assert_eq!(mergesort(v.as_slice()), vec!(3, 4, 6, 7, 8, 8, 8, 9, 10));
  }

  #[test]
  fn test_mergesort_sorts_returns_empty_set_when_empty() {
    let v: Vec<i32> = vec!();
    assert_eq!(mergesort(v.as_slice()), vec!());
  }
}