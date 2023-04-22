use alloc::string::ToString;

use zeroize::Zeroizing;

use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

use curve25519_dalek::{
    constants::ED25519_BASEPOINT_TABLE, edwards::EdwardsPoint, scalar::Scalar,
};

use monero_serai::{
    hash_to_scalar,
    wallet::{
        address::{AddressSpec, Network, SubaddressIndex},
        seed::{Language, Seed},
        ViewPair,
    },
};

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

unsafe fn seed_from_entropy(seed: *const [u8; 32]) -> Option<Seed> {
    Seed::from_entropy(
        Language::English,
        (unsafe { *seed.as_ref().unwrap() }).into(),
    )
}

unsafe fn view_pair_from_entropy(
    seed: *const [u8; 32],
) -> Option<(EdwardsPoint, Zeroizing<Scalar>)> {
    let seed = Seed::from_entropy(
        Language::English,
        (unsafe { *seed.as_ref().unwrap() }).into(),
    )?;

    let spend_key =
        Zeroizing::new(Scalar::from_bytes_mod_order(*seed.entropy()));
    let view_key = Zeroizing::new(hash_to_scalar(&*seed.entropy()));
    let spend_key = &ED25519_BASEPOINT_TABLE * &*spend_key;

    Some((spend_key, view_key))
}

#[no_mangle]
pub unsafe extern "C" fn monero_seed_to_string(
    seed: *const [u8; 32],
    res: *mut u8,
    res_len: usize,
) -> usize {
    // entropy here is the underlying randomness for this seed
    let Some(seed) = seed_from_entropy(seed) else {
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

#[no_mangle]
pub unsafe extern "C" fn monero_seed_to_view_pair(
    seed: *const [u8; 32],
    res: *mut u8,
    res_len: usize,
) -> usize {
    let Some((spend_key, view_key)) = view_pair_from_entropy(seed) else {
        return 0;
    };
    if res_len != 64 {
        return 0;
    }

    let spend_key = spend_key.compress().0;

    let slice = unsafe { core::slice::from_raw_parts_mut(res, 64) };
    slice[..32].copy_from_slice(&spend_key);
    slice[32..].copy_from_slice(&*Zeroizing::new(view_key.to_bytes()));

    64
}

#[no_mangle]
pub unsafe extern "C" fn monero_seed_to_subaddress(
    seed: *const [u8; 32],
    account: u32,
    address: u32,
    res: *mut u8,
    res_len: usize,
) -> usize {
    let Some((spend_key, view_key)) = view_pair_from_entropy(seed) else {
        return 0;
    };

    let addr = ViewPair::new(spend_key, view_key)
        .address(
            Network::Mainnet,
            if let Some(index) = SubaddressIndex::new(account, address) {
                AddressSpec::Subaddress(index)
            } else {
                AddressSpec::Standard
            },
        )
        .to_string();

    if res_len < addr.len() {
        return 0;
    }

    let slice = unsafe { core::slice::from_raw_parts_mut(res, addr.len()) };
    slice.copy_from_slice(addr.as_bytes());
    addr.len()
}
