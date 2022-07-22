pub mod nanoid;

pub trait Member {
    type F;
    type T;

    fn get(&self) -> &Self::T;
}

#[inline(never)]
pub fn constant_time_eq(a: &str, b: &str) -> bool {
    debug_assert!(a.len() == b.len());

    a.bytes()
        .zip(b.bytes())
        .fold(0, |acc, (a, b)| acc | (a ^ b))
        == 0
}
