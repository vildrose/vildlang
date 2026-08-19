use vildrose_example::example::Example;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let example = Example::new("test".to_string());
        assert_eq!(example.get(), "test");
    }

    #[test]
    fn test_set_string() {
        let mut example = Example::new("test".to_string());
        example.set("new".to_string());
        assert_eq!(example.get(), "new");
    }
}
