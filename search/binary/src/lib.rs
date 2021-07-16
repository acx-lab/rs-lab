fn say_hello() {
    println!("{}", "hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert!(say_hello() == ());
    }
}
