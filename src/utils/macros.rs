macro_rules! cast_to_u64 {
    ( $x:expr ) => {
        u64::from($x)
    };
}

pub(crate) use cast_to_u64;
