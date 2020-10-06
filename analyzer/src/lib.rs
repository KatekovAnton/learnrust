pub mod analyzer {

    /// Some documentation.
    /// # fn get_constant() {}
    /// # Examples
    /// ```
    /// let v = vec![1, 2, 3];
    /// assert_eq!(v[0], 1);
    /// assert_eq!(v[1], 2);
    /// assert_eq!(v[2], 3);
    ///
    /// let c = analyzer::analyzer::get_constant();
    /// assert_eq!(c, 4);
    /// ```

    pub fn get_constant() -> i32 {
        4
    }


    /// First line is a short summary describing function.
    ///
    /// The next lines present detailed documentation. Code blocks start with
    /// triple backquotes and have implicit `fn main()` inside
    /// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
    ///
    /// ```
    /// let result = analyzer::analyzer::add(2, 3);
    /// assert_eq!(result, 5);
    /// ```
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Usually doc comments may include sections "Examples", "Panics" and "Failures".
    ///
    /// The next function divides two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = analyzer::analyzer::div(10, 2);
    /// assert_eq!(result, 5);
    /// ```
    ///
    /// # Panics
    ///
    /// The function panics if the second argument is zero.
    ///
    /// ```rust,should_panic
    /// // panics on division by zero
    /// analyzer::analyzer::div(10, 0);
    /// ```
    pub fn div(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("Divide-by-zero error");
        }

        a / b
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
