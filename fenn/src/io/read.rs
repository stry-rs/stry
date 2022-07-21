use {
    super::Endian,
    std::io::{self, Read},
};

macro_rules! fun {
    ($name:ident, $size:ident) => {
        fn $name(&mut self, endian: Endian) -> Result<$size, io::Error> {
            let mut buff: [u8; std::mem::size_of::<$size>()] = [0; std::mem::size_of::<$size>()];

            self.read_exact(&mut buff)?;

            Ok(match endian {
                Endian::Big => $size::from_be_bytes(buff),
                Endian::Little => $size::from_le_bytes(buff),
            })
        }
    };
}

/// Various utilities when working with
/// [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html),
/// currently just raw byte reading with big and little endianness.
pub trait ReadExt {
    //#region [ rgba(27, 133, 184, 0.1) ] Unsigned
    /// Reads a `usize` with the default endianness of `Endian`.
    ///
    /// A wrapper around `read_usize_with_endian` passing `Endian::default` as
    /// the endianness.
    fn read_usize(&mut self) -> Result<usize, io::Error> {
        self.read_usize_with_endian(Endian::default())
    }

    /// Reads a `u8` with the default endianness of `Endian`.
    ///
    /// A wrapper around `read_u8_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::ReadExt;
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5]);
    ///
    /// assert_eq!(5u8, buff.read_u8().unwrap());
    /// ```
    fn read_u8(&mut self) -> Result<u8, io::Error> {
        self.read_u8_with_endian(Endian::default())
    }

    /// Reads a `u16` with the default endianness of `Endian`.
    ///
    /// A wrapper around `read_u16_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::ReadExt;
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5, 0]);
    ///
    /// assert_eq!(5u16, buff.read_u16().unwrap());
    /// ```
    fn read_u16(&mut self) -> Result<u16, io::Error> {
        self.read_u16_with_endian(Endian::default())
    }

    /// Reads a `u32` with the default endianness of `Endian`.
    ///
    /// A wrapper around `read_u32_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::ReadExt;
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5, 0, 0, 0]);
    ///
    /// assert_eq!(5u32, buff.read_u32().unwrap());
    /// ```
    fn read_u32(&mut self) -> Result<u32, io::Error> {
        self.read_u32_with_endian(Endian::default())
    }

    /// Reads a `u64` with the default endianness of `Endian`.
    ///
    /// A wrapper around `read_u64_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::ReadExt;
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5, 0, 0, 0, 0, 0, 0, 0]);
    ///
    /// assert_eq!(5u64, buff.read_u64().unwrap());
    /// ```
    fn read_u64(&mut self) -> Result<u64, io::Error> {
        self.read_u64_with_endian(Endian::default())
    }
    //#endregion

    //#region [ rgba(174, 90, 65, 0.1) ] Signed
    /// Reads a `isize` with the default endianness of `Endian`.
    ///
    /// A wrapper around `read_isize_with_endian` passing `Endian::default` as
    /// the endianness.
    fn read_isize(&mut self) -> Result<isize, io::Error> {
        self.read_isize_with_endian(Endian::default())
    }

    /// Reads a `i8` with the default endianness of `Endian`.
    ///
    /// A wrapper around `read_i8_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::ReadExt;
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5]);
    ///
    /// assert_eq!(5i8, buff.read_i8().unwrap());
    /// ```
    fn read_i8(&mut self) -> Result<i8, io::Error> {
        self.read_i8_with_endian(Endian::default())
    }

    /// Reads a `i16` with the default endianness of `Endian`.
    ///
    /// A wrapper around `read_i16_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::ReadExt;
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5, 0]);
    ///
    /// assert_eq!(5i16, buff.read_i16().unwrap());
    /// ```
    fn read_i16(&mut self) -> Result<i16, io::Error> {
        self.read_i16_with_endian(Endian::default())
    }

    /// Reads a `i32` with the default endianness of `Endian`.
    ///
    /// A wrapper around `read_i32_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::ReadExt;
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5, 0, 0, 0]);
    ///
    /// assert_eq!(5i32, buff.read_i32().unwrap());
    /// ```
    fn read_i32(&mut self) -> Result<i32, io::Error> {
        self.read_i32_with_endian(Endian::default())
    }

    /// Reads a `i64` with the default endianness of `Endian`.
    ///
    /// A wrapper around `read_i64_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::ReadExt;
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5, 0, 0, 0, 0, 0, 0, 0]);
    ///
    /// assert_eq!(5i64, buff.read_i64().unwrap());
    /// ```
    fn read_i64(&mut self) -> Result<i64, io::Error> {
        self.read_i64_with_endian(Endian::default())
    }
    //#endregion

    //#region [ rgba(85, 158, 131, 0.1) ] Float
    /// Reads a `f32` with the default endianness of `Endian`.
    ///
    /// A wrapper around `read_f32_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::ReadExt;
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![0, 0, 160, 64]);
    ///
    /// assert_eq!(5f32, buff.read_f32().unwrap());
    /// ```
    fn read_f32(&mut self) -> Result<f32, io::Error> {
        self.read_f32_with_endian(Endian::default())
    }

    /// Reads a `f64` with the default endianness of `Endian`.
    ///
    /// A wrapper around `read_f64_with_endian` passing `Endian::default` as the
    /// endianness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::ReadExt;
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![0, 0, 0, 0, 0, 0, 20, 64]);
    ///
    /// assert_eq!(5f64, buff.read_f64().unwrap());
    /// ```
    fn read_f64(&mut self) -> Result<f64, io::Error> {
        self.read_f64_with_endian(Endian::default())
    }
    //#endregion

    //#region [ rgba(27, 133, 184, 0.1) ] Unsigned
    /// Read a `usize` with the specified endianness.
    ///
    /// See: [read_usize](#method.read_usize)
    fn read_usize_with_endian(&mut self, endian: Endian) -> Result<usize, io::Error>;

    /// Read a `u8` with the specified endianness.
    ///
    /// See: [read_u8](#method.read_u8)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, ReadExt};
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5]);
    ///
    /// assert_eq!(5u8, buff.read_u8_with_endian(Endian::Little).unwrap());
    /// ```
    fn read_u8_with_endian(&mut self, endian: Endian) -> Result<u8, io::Error>;

    /// Read a `u16` with the specified endianness.
    ///
    /// See: [read_u16](#method.read_u16)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, ReadExt};
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5, 0]);
    ///
    /// assert_eq!(5u16, buff.read_u16_with_endian(Endian::Little).unwrap());
    /// ```
    fn read_u16_with_endian(&mut self, endian: Endian) -> Result<u16, io::Error>;

    /// Read a `u32` with the specified endianness.
    ///
    /// See: [read_u32](#method.read_u32)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, ReadExt};
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5, 0, 0, 0]);
    ///
    /// assert_eq!(5u32, buff.read_u32_with_endian(Endian::Little).unwrap());
    /// ```
    fn read_u32_with_endian(&mut self, endian: Endian) -> Result<u32, io::Error>;

    /// Read a `u64` with the specified endianness.
    ///
    /// See: [read_u64](#method.read_u64)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, ReadExt};
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5, 0, 0, 0, 0, 0, 0, 0]);
    ///
    /// assert_eq!(5u64, buff.read_u64_with_endian(Endian::Little).unwrap());
    /// ```
    fn read_u64_with_endian(&mut self, endian: Endian) -> Result<u64, io::Error>;
    //#endregion

    //#region [ rgba(174, 90, 65, 0.1) ] Signed
    /// Read a `isize` with the specified endianness.
    ///
    /// See: [read_isize](#method.read_isize)
    fn read_isize_with_endian(&mut self, endian: Endian) -> Result<isize, io::Error>;

    /// Read a `i8` with the specified endianness.
    ///
    /// See: [read_i8](#method.read_i8)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, ReadExt};
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5]);
    ///
    /// assert_eq!(5i8, buff.read_i8_with_endian(Endian::Little).unwrap());
    /// ```
    fn read_i8_with_endian(&mut self, endian: Endian) -> Result<i8, io::Error>;

    /// Read a `i16` with the specified endianness.
    ///
    /// See: [read_i16](#method.read_i16)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, ReadExt};
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5, 0]);
    ///
    /// assert_eq!(5i16, buff.read_i16_with_endian(Endian::Little).unwrap());
    /// ```
    fn read_i16_with_endian(&mut self, endian: Endian) -> Result<i16, io::Error>;

    /// Read a `i32` with the specified endianness.
    ///
    /// See: [read_i32](#method.read_i32)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, ReadExt};
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5, 0, 0, 0]);
    ///
    /// assert_eq!(5i32, buff.read_i32_with_endian(Endian::Little).unwrap());
    /// ```
    fn read_i32_with_endian(&mut self, endian: Endian) -> Result<i32, io::Error>;

    /// Read a `i64` with the specified endianness.
    ///
    /// See: [read_i64](#method.read_i64)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, ReadExt};
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![5, 0, 0, 0, 0, 0, 0, 0]);
    ///
    /// assert_eq!(5i64, buff.read_i64_with_endian(Endian::Little).unwrap());
    /// ```
    fn read_i64_with_endian(&mut self, endian: Endian) -> Result<i64, io::Error>;
    //#endregion

    //#region [ rgba(85, 158, 131, 0.1) ] Float
    /// Read a `f32` with the specified endianness.
    ///
    /// See: [read_f32](#method.read_f32)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, ReadExt};
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![0, 0, 160, 64]);
    ///
    /// assert_eq!(5f32, buff.read_f32_with_endian(Endian::Little).unwrap());
    /// ```
    fn read_f32_with_endian(&mut self, endian: Endian) -> Result<f32, io::Error>;

    /// Read a `f64` with the specified endianness.
    ///
    /// See: [read_f64](#method.read_f64)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::{Endian, ReadExt};
    /// use std::io::{Cursor, Read};
    ///
    /// let mut buff = Cursor::new(vec![0, 0, 0, 0, 0, 0, 20, 64]);
    ///
    /// assert_eq!(5f64, buff.read_f64_with_endian(Endian::Little).unwrap());
    /// ```
    fn read_f64_with_endian(&mut self, endian: Endian) -> Result<f64, io::Error>;
    //#endregion
}

impl<T> ReadExt for T
where
    T: Read,
{
    //#region [ rgba(27, 133, 184, 0.1) ] Unsigned
    fun!(read_usize_with_endian, usize);
    fun!(read_u8_with_endian, u8);
    fun!(read_u16_with_endian, u16);
    fun!(read_u32_with_endian, u32);
    fun!(read_u64_with_endian, u64);
    //#endregion

    //#region [ rgba(174, 90, 65, 0.1) ] Signed
    fun!(read_isize_with_endian, isize);
    fun!(read_i8_with_endian, i8);
    fun!(read_i16_with_endian, i16);
    fun!(read_i32_with_endian, i32);
    fun!(read_i64_with_endian, i64);
    //#endregion

    //#region [ rgba(85, 158, 131, 0.1) ] Float
    fun!(read_f32_with_endian, f32);
    fun!(read_f64_with_endian, f64);
    //#endregion
}
