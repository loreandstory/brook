/// First function
///
/// # Example
///
/// ```
/// let x = brook::first_function();
///
/// assert_eq!(5, x);
/// ```
pub fn first_function() -> i32 {
  5
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
