use {
    super::Endian,
    std::io::{self, Write},
};

macro_rules! fun {
    ($name:ident, $size:ident) => {
        fn $name(&mut self, num: $size, endian: Endian) -> Result<(), io::Error> {
            let bytes: [u8; std::mem::size_of::<$size>()] = match endian {
                Endian::Big => num.to_be_bytes(),
                Endian::Little => num.to_le_bytes(),
            };

            self.write_all(bytes.as_ref())?;

            Ok(())
        }
    };
}

/// Various utilities when working with
/// [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html),
/// currently just raw byte writing with big and little endianness.
pub trait WriteExt {
    //#region [ rgba(27, 133, 184, 0.1) ] Unsigned
    /// Writes a `usize` with the default endianness of `Endian`.
    ///
    /// A wrapper around `write_usize_with_endian` passing `Endian::default` as the
    /// endianness.
    fn write_usize(&mut self, num: usize) -> Result<(), io::Error> {
        self.write_usize_with_endian(num, Endian::default())
    }

    /// Writes a `u8` with the default endianness of `Endian`.
    ///
    /// A wrapper around `write_u8_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::WriteExt;
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_u8(5).unwrap();
    ///
    /// assert_eq!(vec![5], buff.into_inner());
    /// ```
    fn write_u8(&mut self, num: u8) -> Result<(), io::Error> {
        self.write_u8_with_endian(num, Endian::default())
    }

    /// Writes a `u16` with the default endianness of `Endian`.
    ///
    /// A wrapper around `write_u16_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::WriteExt;
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_u16(5).unwrap();
    ///
    /// assert_eq!(vec![5, 0], buff.into_inner());
    /// ```
    fn write_u16(&mut self, num: u16) -> Result<(), io::Error> {
        self.write_u16_with_endian(num, Endian::default())
    }

    /// Writes a `u32` with the default endianness of `Endian`.
    ///
    /// A wrapper around `write_u32_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::WriteExt;
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_u32(5).unwrap();
    ///
    /// assert_eq!(vec![5, 0, 0, 0], buff.into_inner());
    /// ```
    fn write_u32(&mut self, num: u32) -> Result<(), io::Error> {
        self.write_u32_with_endian(num, Endian::default())
    }

    /// Writes a `u64` with the default endianness of `Endian`.
    ///
    /// A wrapper around `write_u64_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::WriteExt;
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_u64(5).unwrap();
    ///
    /// assert_eq!(vec![5, 0, 0, 0, 0, 0, 0, 0], buff.into_inner());
    /// ```
    fn write_u64(&mut self, num: u64) -> Result<(), io::Error> {
        self.write_u64_with_endian(num, Endian::default())
    }
    //#endregion

    //#region [ rgba(174, 90, 65, 0.1) ] Signed
    /// Writes a `isize` with the default endianness of `Endian`.
    ///
    /// A wrapper around `write_isize_with_endian` passing `Endian::default` as the
    /// endianness.
    fn write_isize(&mut self, num: isize) -> Result<(), io::Error> {
        self.write_isize_with_endian(num, Endian::default())
    }

    /// Writes a `i8` with the default endianness of `Endian`.
    ///
    /// A wrapper around `write_i8_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::WriteExt;
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_i8(5).unwrap();
    ///
    /// assert_eq!(vec![5], buff.into_inner());
    /// ```
    fn write_i8(&mut self, num: i8) -> Result<(), io::Error> {
        self.write_i8_with_endian(num, Endian::default())
    }

    /// Writes a `i16` with the default endianness of `Endian`.
    ///
    /// A wrapper around `write_i16_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::WriteExt;
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_i16(5).unwrap();
    ///
    /// assert_eq!(vec![5, 0], buff.into_inner());
    /// ```
    fn write_i16(&mut self, num: i16) -> Result<(), io::Error> {
        self.write_i16_with_endian(num, Endian::default())
    }

    /// Writes a `i32` with the default endianness of `Endian`.
    ///
    /// A wrapper around `write_i32_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::WriteExt;
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_i32(5).unwrap();
    ///
    /// assert_eq!(vec![5, 0, 0, 0], buff.into_inner());
    /// ```
    fn write_i32(&mut self, num: i32) -> Result<(), io::Error> {
        self.write_i32_with_endian(num, Endian::default())
    }

    /// Writes a `i64` with the default endianness of `Endian`.
    ///
    /// A wrapper around `write_i64_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::WriteExt;
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_i64(5).unwrap();
    ///
    /// assert_eq!(vec![5, 0, 0, 0, 0, 0, 0, 0], buff.into_inner());
    /// ```
    fn write_i64(&mut self, num: i64) -> Result<(), io::Error> {
        self.write_i64_with_endian(num, Endian::default())
    }
    //#endregion

    //#region [ rgba(85, 158, 131, 0.1) ] Float
    /// Writes a `f32` with the default endianness of `Endian`.
    ///
    /// A wrapper around `write_f32_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::WriteExt;
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_f32(5.0).unwrap();
    ///
    /// assert_eq!(vec![0, 0, 160, 64], buff.into_inner());
    /// ```
    fn write_f32(&mut self, num: f32) -> Result<(), io::Error> {
        self.write_f32_with_endian(num, Endian::default())
    }

    /// Writes a `f64` with the default endianness of `Endian`.
    ///
    /// A wrapper around `write_f64_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::WriteExt;
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_f64(5.0).unwrap();
    ///
    /// assert_eq!(vec![0, 0, 0, 0, 0, 0, 20, 64], buff.into_inner());
    /// ```
    fn write_f64(&mut self, num: f64) -> Result<(), io::Error> {
        self.write_f64_with_endian(num, Endian::default())
    }
    //#endregion

    //#region [ rgba(27, 133, 184, 0.1) ] Unsigned
    /// Writes a `u8` with the specified endianness.
    ///
    /// See: [write_usize](#method.write_usize)
    fn write_usize_with_endian(&mut self, num: usize, endian: Endian) -> Result<(), io::Error>;

    /// Writes a `u8` with the specified endianness.
    ///
    /// See: [write_u8](#method.write_u8)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, WriteExt};
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_u8_with_endian(5, Endian::Little).unwrap();
    ///
    /// assert_eq!(vec![5], buff.into_inner());
    /// ```
    fn write_u8_with_endian(&mut self, num: u8, endian: Endian) -> Result<(), io::Error>;

    /// Writes a `u16` with the specified endianness.
    ///
    /// See: [write_u16](#method.write_u16)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, WriteExt};
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_u16_with_endian(5, Endian::Little).unwrap();
    ///
    /// assert_eq!(vec![5, 0], buff.into_inner());
    /// ```
    fn write_u16_with_endian(&mut self, num: u16, endian: Endian) -> Result<(), io::Error>;

    /// Writes a `u32` with the specified endianness.
    ///
    /// See: [write_u32](#method.write_u32)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, WriteExt};
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_u32_with_endian(5, Endian::Little).unwrap();
    ///
    /// assert_eq!(vec![5, 0, 0, 0], buff.into_inner());
    /// ```
    fn write_u32_with_endian(&mut self, num: u32, endian: Endian) -> Result<(), io::Error>;

    /// Writes a `u64` with the specified endianness.
    ///
    /// See: [write_u64](#method.write_u64)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, WriteExt};
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_u64_with_endian(5, Endian::Little).unwrap();
    ///
    /// assert_eq!(vec![5, 0, 0, 0, 0, 0, 0, 0], buff.into_inner());
    /// ```
    fn write_u64_with_endian(&mut self, num: u64, endian: Endian) -> Result<(), io::Error>;
    //#endregion

    //#region [ rgba(174, 90, 65, 0.1) ] Signed
    /// Writes a `isize` with the specified endianness.
    ///
    /// See: [write_isize](#method.write_isize)
    fn write_isize_with_endian(&mut self, num: isize, endian: Endian) -> Result<(), io::Error>;

    /// Writes a `i8` with the specified endianness.
    ///
    /// See: [write_i8](#method.write_i8)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, WriteExt};
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_i8_with_endian(5, Endian::Little).unwrap();
    ///
    /// assert_eq!(vec![5], buff.into_inner());
    /// ```
    fn write_i8_with_endian(&mut self, num: i8, endian: Endian) -> Result<(), io::Error>;

    /// Writes a `i16` with the specified endianness.
    ///
    /// See: [write_i16](#method.write_i16)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, WriteExt};
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_i16_with_endian(5, Endian::Little).unwrap();
    ///
    /// assert_eq!(vec![5, 0], buff.into_inner());
    /// ```
    fn write_i16_with_endian(&mut self, num: i16, endian: Endian) -> Result<(), io::Error>;

    /// Writes a `i32` with the specified endianness.
    ///
    /// See: [write_i32](#method.write_i32)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, WriteExt};
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_i32_with_endian(5, Endian::Little).unwrap();
    ///
    /// assert_eq!(vec![5, 0, 0, 0], buff.into_inner());
    /// ```
    fn write_i32_with_endian(&mut self, num: i32, endian: Endian) -> Result<(), io::Error>;

    /// Writes a `i64` with the specified endianness.
    ///
    /// See: [write_i64](#method.write_i64)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, WriteExt};
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_i64_with_endian(5, Endian::Little).unwrap();
    ///
    /// assert_eq!(vec![5, 0, 0, 0, 0, 0, 0, 0], buff.into_inner());
    /// ```
    fn write_i64_with_endian(&mut self, num: i64, endian: Endian) -> Result<(), io::Error>;
    //#endregion

    //#region [ rgba(85, 158, 131, 0.1) ] Float
    /// Writes a `f32` with the specified endianness.
    ///
    /// See: [write_f32](#method.write_f32)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, WriteExt};
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_f32_with_endian(5.0, Endian::Little).unwrap();
    ///
    /// assert_eq!(vec![0, 0, 160, 64], buff.into_inner());
    /// ```
    fn write_f32_with_endian(&mut self, num: f32, endian: Endian) -> Result<(), io::Error>;

    /// Writes a `f64` with the specified endianness.
    ///
    /// See: [write_f64](#method.write_f64)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, WriteExt};
    /// use std::io::{Cursor, Write};
    ///
    /// let mut buff = Cursor::new(vec![]);
    ///
    /// buff.write_f64_with_endian(5.0, Endian::Little).unwrap();
    ///
    /// assert_eq!(vec![0, 0, 0, 0, 0, 0, 20, 64], buff.into_inner());
    /// ```
    fn write_f64_with_endian(&mut self, num: f64, endian: Endian) -> Result<(), io::Error>;
    //#endregion
}

impl<T> WriteExt for T
where
    T: Write,
{
    //#region [ rgba(27, 133, 184, 0.1) ] Unsigned
    fun!(write_usize_with_endian, usize);
    fun!(write_u8_with_endian, u8);
    fun!(write_u16_with_endian, u16);
    fun!(write_u32_with_endian, u32);
    fun!(write_u64_with_endian, u64);
    //#endregion

    //#region [ rgba(174, 90, 65, 0.1) ] Signed
    fun!(write_isize_with_endian, isize);
    fun!(write_i8_with_endian, i8);
    fun!(write_i16_with_endian, i16);
    fun!(write_i32_with_endian, i32);
    fun!(write_i64_with_endian, i64);
    //#endregion

    //#region [ rgba(85, 158, 131, 0.1) ] Float
    fun!(write_f32_with_endian, f32);
    fun!(write_f64_with_endian, f64);
    //#endregion
}
