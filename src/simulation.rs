use std::arch::x86_64::_pdep_u64;
use crate::rng::rng;

/// Returns the index of a domino drawn at random
pub fn simulated_deal(contents: u64) -> u8 {
    let k = rng(contents.count_ones());
    #[cfg(target_feature = "bmi2")]
    unsafe { 
        _pdep_u64(1u64 << k, contents).trailing_zeros() as u8
    }
    #[cfg(not(target_feature = "bmi2"))]
    {
        let mut m = contents;
        for _ in 0..k { m &= m - 1; }
        m.trailing_zeros() as u8
    }
}

