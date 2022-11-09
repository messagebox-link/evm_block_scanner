use log::debug;
use rand::{Rng, thread_rng};

/// Get Random number from max
pub fn random_number(max: usize) -> usize {
    debug!("[random_number] max {:?}", max);
    let _max = max as u128;
    let mut rng = thread_rng();
    let u = rng.gen_range(0..=_max - 1) as usize;
    debug!("[random_number] range value {:?}", u);
    u
}