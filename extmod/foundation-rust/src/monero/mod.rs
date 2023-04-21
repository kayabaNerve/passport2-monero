use rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;

use monero_serai::wallet::seed::{Language, Seed};

#[no_mangle]
pub unsafe extern "C" fn monero_seed_new(entropy: *const [u8; 32], res: *mut u8, res_len: usize) -> usize {
  let seed = Seed::new(&mut ChaCha20Rng::from_seed(unsafe { *entropy.as_ref().unwrap() }), Language::English).to_string();
  if res_len < seed.len() {
    return 0;
  }
  unsafe { core::slice::from_raw_parts_mut(res, seed.len()).copy_from_slice(seed.as_bytes()) };
  seed.len()
}
