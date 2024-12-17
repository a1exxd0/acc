pub mod lib {
    /// Adds two integers of specific type `i32`
    ///
    /// # Examples
    /// ```rust
    /// use acc::lib::add_two;
    /// let x = add_two(1, 2);
    /// ```
    pub fn add_two(a: i32, b: i32) -> i32 {
        a + b
    }
}
