/// This function returns 5
///
/// # Example
/// ```
/// # use doc_example::get_5;
///
/// let result = get_5();
/// assert_eq!(result, 5);
/// ```
pub fn get_5() -> u64 {
    5
}

/// This function returns 6
///
/// # Example
/// ```
/// # use doc_example::get_6;
///
/// let result = get_6();
/// assert_eq!(result, 5); // This will fail
/// ```
pub fn get_6() -> u64 {
   6 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result1 = get_5();
        assert_eq!(result1, 5);
        let result2 = get_6();
        assert_eq!(result2, 6);
    }
}
