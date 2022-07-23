use std::fmt::Write;

pub use windswept_macros::rsx;

pub trait Render: Sized {
    fn render(self) -> Result<String, std::fmt::Error> {
        let mut buf = String::with_capacity(self.size_hint() * 2);
        self.render_into(&mut buf)?;
        Ok(buf)
    }

    fn render_into(self, writer: &mut dyn Write) -> Result<(), std::fmt::Error>;

    fn size_hint(&self) -> usize;
}

impl<'s> Render for &'s str {
    #[inline]
    fn render_into(self, writer: &mut dyn Write) -> Result<(), std::fmt::Error> {
        writer.write_str(self)?;

        Ok(())
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl Render for String {
    #[inline]
    fn render_into(self, writer: &mut dyn Write) -> Result<(), std::fmt::Error> {
        writer.write_str(&self)?;

        Ok(())
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl<'r> Render for &'r String {
    #[inline]
    fn render_into(self, writer: &mut dyn Write) -> Result<(), std::fmt::Error> {
        writer.write_str(self)?;

        Ok(())
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

/// This is hidden as its an internal implementation and should mot be relied on
#[doc(hidden)]
impl<F> Render for (F, usize)
where
    F: FnOnce(&mut dyn Write) -> Result<(), std::fmt::Error>,
{
    #[inline]
    fn render_into(self, writer: &mut dyn Write) -> Result<(), std::fmt::Error> {
        (self.0)(writer)
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.1
    }
}

macro_rules! render_num {
    ($( $num:ty, )+) => {
        $(
            impl Render for $num {
                #[inline]
                fn render_into(self, writer: &mut dyn Write) -> Result<(), std::fmt::Error> {
                    writer.write_str(&self.to_string())?;

                    Ok(())
                }

                // https://stackoverflow.com/a/69298721
                #[inline]
                fn size_hint(&self) -> usize {
                    const fn inner(num: $num) -> usize {
                        const BASE: $num = 10;

                        let mut power = BASE;
                        let mut count = 1;

                        while num >= power {
                            count += 1;

                            if let Some(new_power) = power.checked_mul(BASE) {
                                power = new_power;
                            } else {
                                break;
                            }
                        }

                        count
                    }

                    inner(*self)
                }
            }
        )+
    };
}

#[rustfmt::skip]
render_num![
    isize, i8, i16, i32, i64,
    usize, u8, u16, u32, u64,
];

#[cfg(test)]
mod tests {
    use super::Render as _;

    #[test]
    fn test_num_size() {
        assert_eq!(1, 1.size_hint());
        assert_eq!(2, 10.size_hint());
        assert_eq!(3, 100.size_hint());
        assert_eq!(4, 1000.size_hint());
        assert_eq!(5, 10000.size_hint());
        assert_eq!(6, 100000.size_hint());
        assert_eq!(7, 1000000.size_hint());
        assert_eq!(8, 10000000.size_hint());
        assert_eq!(9, 100000000.size_hint());
        assert_eq!(10, 1000000000.size_hint());
    }
}
