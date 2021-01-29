/// An extension to
/// [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
/// allowing for inline trimming.
pub trait StringExt {
    /// Mutates a string in place with leading and trailing whitespace removed.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived
    /// Core Property `White_Space`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::StringExt;
    ///
    /// let mut test = String::from("   Hello World!   ");
    ///
    /// test.trim();
    ///
    /// assert_eq!(test, String::from("Hello World!"));
    /// ```
    fn trim(&mut self);

    /// Mutates a string in place with all suffixes that match a pattern repeatedly removed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::StringExt;
    ///
    /// let mut test = String::from("blahHello World!blah");
    ///
    /// test.trim_matches("blah");
    ///
    /// assert_eq!(test, String::from("Hello World!"));
    /// ```
    ///
    /// # Note
    ///
    /// Once the Pattern API ([#27721](https://github.com/rust-lang/rust/issues/27721)) becomes stable
    /// this will be changed to accept a pattern.
    fn trim_matches(&mut self, rem: &str);

    /// Mutates a string in place with leading whitespace removed.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived
    /// Core Property `White_Space`.
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. `start` in this context means the first
    /// position of that byte string; for a left-to-right language like English or
    /// Russian, this will be left side, and for right-to-left languages like
    /// Arabic or Hebrew, this will be the right side.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::StringExt;
    ///
    /// let mut test = String::from("   Hello World!");
    ///
    /// test.trim_start();
    ///
    /// assert_eq!(test, String::from("Hello World!"));
    /// ```
    fn trim_start(&mut self);

    /// Mutates a string in place with all suffixes that match a pattern repeatedly removed.
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. `start` in this context means the first
    /// position of that byte string; for a left-to-right language like English or
    /// Russian, this will be left side, and for right-to-left languages like
    /// Arabic or Hebrew, this will be the right side.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::StringExt;
    ///
    /// let mut test = String::from("blahHello World!");
    ///
    /// test.trim_start_matches("blah");
    ///
    /// assert_eq!(test, String::from("Hello World!"));
    /// ```
    ///
    /// # Note
    ///
    /// Once the Pattern API ([#27721](https://github.com/rust-lang/rust/issues/27721)) becomes stable
    /// this will be changed to accept a pattern.
    fn trim_start_matches(&mut self, rem: &str);

    /// Mutates a string in place with trailing whitespace removed.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived
    /// Core Property `White_Space`.
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. 'Left' in this context means the first
    /// position of that byte string; for a language like Arabic or Hebrew
    /// which are 'right to left' rather than 'left to right', this will be
    /// the _right_ side, not the left.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::StringExt;
    ///
    /// let mut test = String::from("Hello World!   ");
    ///
    /// test.trim_end();
    ///
    /// assert_eq!(test, String::from("Hello World!"));
    /// ```
    fn trim_end(&mut self);

    /// Mutates a string in place with all suffixes that match a pattern repeatedly removed.
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. 'Left' in this context means the first
    /// position of that byte string; for a language like Arabic or Hebrew
    /// which are 'right to left' rather than 'left to right', this will be
    /// the _right_ side, not the left.
    ///
    /// ```rust
    /// use fenn::StringExt;
    ///
    /// let mut test = String::from("Hello World!blah");
    ///
    /// test.trim_end_matches("blah");
    ///
    /// assert_eq!(test, String::from("Hello World!"));
    /// ```
    ///
    /// # Note
    ///
    /// Once the Pattern API ([#27721](https://github.com/rust-lang/rust/issues/27721)) becomes stable
    /// this will be changed to accept a pattern.
    fn trim_end_matches(&mut self, rem: &str);
}

impl StringExt for super::lib::String {
    fn trim(&mut self) {
        self.trim_start();
        self.trim_end();
    }

    fn trim_matches(&mut self, rem: &str) {
        self.trim_start_matches(rem);
        self.trim_end_matches(rem);
    }

    fn trim_start(&mut self) {
        while self.starts_with(char::is_whitespace) {
            self.drain(..1);
        }
    }

    fn trim_start_matches(&mut self, rem: &str) {
        while self.starts_with(rem) {
            self.drain(..rem.len());
        }
    }

    fn trim_end(&mut self) {
        while self.ends_with(char::is_whitespace) {
            self.truncate(self.len().saturating_sub(1));
        }
    }

    fn trim_end_matches(&mut self, rem: &str) {
        while self.ends_with(rem) {
            self.truncate(self.len().saturating_sub(rem.len()));
        }
    }
}
