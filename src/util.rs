#[allow(unused)]
pub trait FastParse: Sized {
    fn fast_parse_counted<Bytes>(input: Bytes) -> Option<(Self, usize)>
    where
        Bytes: AsRef<[u8]>;

    fn fast_parse<Bytes>(input: Bytes) -> Option<Self>
    where
        Bytes: AsRef<[u8]>;
    // Parses, assuming all of the given bytes are valid input without checking
    fn fast_parse_unchecked<Bytes>(input: Bytes) -> Self
    where
        Bytes: AsRef<[u8]>;
}

macro_rules! uint_impl {
    ($($ty:ty),+) => {
        $(
            impl FastParse for $ty {
                fn fast_parse_counted<Bytes>(input: Bytes) -> Option<(Self, usize)>
                where
                    Bytes: AsRef<[u8]>,
                {
                    let mut num = 0;
                    let mut counted = 0;
                    for &b in input.as_ref().iter() {
                        if b.is_ascii_digit() {
                            num *= 10;
                            num += (b - b'0') as Self;
                            counted += 1;
                        } else {
                            break;
                        }
                    }

                    Some((num, counted))
                }

                fn fast_parse<Bytes>(input: Bytes) -> Option<Self>
                where
                    Bytes: AsRef<[u8]>,
                {
                    let mut num = 0;
                    for &b in input.as_ref().iter() {
                        if b.is_ascii_digit() {
                            num *= 10;
                            num += (b - b'0') as Self;
                        } else {
                            break;
                        }
                    }
                    Some(num)
                }

                fn fast_parse_unchecked<Bytes>(input: Bytes) -> Self
                where
                    Bytes: AsRef<[u8]>,
                {
                    let mut num = 0;
                    for &b in input.as_ref().iter() {
                        num *= 10;
                        num += (b - b'0') as Self;
                    }
                    num
                }
            }
        )+
    };
}

uint_impl!(u8, u16, u32, u64, u128);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_fast_parse() {
        assert_eq!(u8::fast_parse_counted("64").unwrap().0, 64);
        assert_eq!(u64::fast_parse_counted("1000020000").unwrap().0, 1000020000);
    }
}
