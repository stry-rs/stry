use {
    crate::models::Id,
    arrayvec::ArrayString,
    rand::{rngs::StdRng, Rng as _, SeedableRng as _},
    std::panic,
};

/// How long an entity `Id` is, recommended 6 or above.
///
/// # Note
///
/// This is only used in the [`Id`] type as its length parameter and with the
/// [`nanoid`] output length.
///
/// [`Id`]: crate::models::Id
pub const SIZE: usize = 6;

const LEN: usize = 54;
const MASK: usize = LEN.next_power_of_two() - 1;
const STEP: usize = 8 * SIZE / 5;

static ALPHABET: [char; LEN] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
    'm', 'n', 'p', 'q', 'r', 's', 't', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Returns a new [`Id`] generated using [nanoid](https://github.com/ai/nanoid) with a custom alphabet.
///
/// Customized version of [the Rust version](https://github.com/nikolay-govorov/nanoid).
pub fn nanoid() -> Option<Id> {
    let mut id = ArrayString::<{ SIZE }>::new();

    loop {
        // `SeedableRng::from_entropy` can panic if getrandom fails, not sure
        // the situation in which that could happen but I want to catch it just incase.
        let mut rng = panic::catch_unwind(StdRng::from_entropy).ok()?;
        let mut bytes = [0u8; STEP];

        rng.fill(&mut bytes[..]);

        for &byte in &bytes {
            let byte = byte as usize & MASK;

            if ALPHABET.len() > byte {
                id.push(ALPHABET[byte]);

                if id.len() == SIZE {
                    return Some(Id(id));
                }
            }
        }
    }
}
