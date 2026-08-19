//! Example for vildrose crates

/// A simple struct that holds a string.
pub struct Example {
    s: String,
}

impl Example {
    /// Creates a new `Example` with the given string.
    pub const fn new(s: String) -> Self {
        Self { s }
    }

    /// Returns a reference to the string.
    pub fn get(&self) -> &str {
        &self.s
    }

    /// Sets the string to the given value.
    pub fn set(&mut self, s: String) {
        self.s = s;
    }
}
