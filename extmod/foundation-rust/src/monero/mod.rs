use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

use monero_serai::wallet::seed::{Language, Seed};

#[no_mangle]
pub unsafe extern "C" fn monero_seed_new(
    entropy: *const [u8; 32],
    res: *mut u8,
    res_len: usize,
) -> usize {
    // Create a RNG out of the entropy
    let mut rng = ChaCha20Rng::from_seed(unsafe { *entropy.as_ref().unwrap() });
    // Create a seed and acquire its entropy (whereas the prior entropy was the entropy used to get
    // this entropy)
    let seed = Seed::new(&mut rng, Language::English).entropy();
    if res_len != seed.len() {
        return 0;
    }
    unsafe {
        core::slice::from_raw_parts_mut(res, seed.len())
            .copy_from_slice(seed.as_ref())
    };
    seed.len()
}

#[no_mangle]
pub unsafe extern "C" fn monero_seed_to_string(
    seed: *const [u8; 32],
    res: *mut u8,
    res_len: usize,
) -> usize {
    // entropy here is the underlying randomness for this seed
    let Some(seed) = Seed::from_entropy(Language::English, (unsafe { *seed.as_ref().unwrap() }).into()) else {
    return 0;
  };
    let seed = seed.to_string();
    if res_len < seed.len() {
        return 0;
    }
    unsafe {
        core::slice::from_raw_parts_mut(res, seed.len())
            .copy_from_slice(seed.as_bytes())
    };
    seed.len()
}
