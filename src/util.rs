pub trait FastParse: Sized {
    fn fast_parse<Bytes>(input: Bytes) -> Option<(Self, usize)>
    where
        Bytes: AsRef<[u8]>;
}

macro_rules! uint_impl {
    ($($ty:ty),+) => {
        $(
            impl FastParse for $ty {
                fn fast_parse<Bytes>(input: Bytes) -> Option<(Self, usize)>
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
        assert_eq!(u8::fast_parse("64").unwrap().0, 64);
        assert_eq!(u64::fast_parse("1000020000").unwrap().0, 1000020000);
    }
}
