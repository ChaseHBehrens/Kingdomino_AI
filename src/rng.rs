use std::sync::atomic::{ AtomicU64, Ordering };
use std::time::{ SystemTime, UNIX_EPOCH };


static COUNTER: AtomicU64 = AtomicU64::new(0);


/// Initializes the rng with a seed
pub fn init() {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    COUNTER.store(seed, Ordering::Relaxed);
}


/// Initializes the rng to a given seed
pub fn seed(seed: u64) {
    COUNTER.store(seed, Ordering::Relaxed);
}


/// Returns a random number below the given maximum value
pub fn rng(max: u32) -> u32 {
    if max == 0 { return 0; }
    let state = COUNTER.fetch_add(0xA0761D6478BD642F, Ordering::Relaxed);    
    let t = (state as u128) * ((state ^ 0xE7037ED1A0B428DB) as u128);
    let mixed = ((t >> 64) as u64) ^ (t as u64);
    (((mixed as u32 as u64) * max as u64) >> 32) as u32
}
