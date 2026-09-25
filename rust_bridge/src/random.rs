//! Modul generatora liczb pseudolosowych zgodny binarnie z EterBase/Random.cpp
//!
//! Algorytm Park & Miller LCG (Minimal Standard):
//! a = 16807, m = 2^31 - 1 (2147483647), q = 127773, r = 2836.

use std::sync::atomic::{AtomicU32, Ordering};

static RAND_SEED: AtomicU32 = AtomicU32::new(1);

/// Ustawia ziarno generatora
pub fn srandom(seed: u32) {
    let s = if seed == 0 { 1 } else { seed };
    RAND_SEED.store(s, Ordering::SeqCst);
}

/// Czysty krok generatora Park & Miller LCG
pub fn park_miller_next(seed: u32) -> u32 {
    let current = seed as i32;
    let x = if current <= 0 { 1 } else { current };
    let hi = x / 127773;
    let lo = x % 127773;
    let mut t = 16807 * lo - 2836 * hi;
    if t <= 0 {
        t += 0x7FFFFFFF;
    }
    t as u32
}

/// Generuje 31-bitowa liczbe pseudolosowa [0, 2^31 - 1]
pub fn random() -> u32 {
    loop {
        let current = RAND_SEED.load(Ordering::Relaxed);
        let next = park_miller_next(current);
        if RAND_SEED.compare_exchange_weak(current, next, Ordering::SeqCst, Ordering::Relaxed).is_ok() {
            return next;
        }
    }
}

/// Generuje losowa liczbe zmiennoprzecinkowa [low, high)
pub fn frandom(low: f32, high: f32) -> f32 {
    let r = random() as f32 / 2147483648.0f32;
    r * (high - low) + low
}

/// Generuje liczbe calkowita z przedzialu [from, to] wlacznie
pub fn random_range(from: i32, to: i32) -> i32 {
    if from >= to {
        return from;
    }
    let span = (to - from + 1) as u32;
    let r = (random() % span) as i32;
    from + r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_park_miller_known_values() {
        // Dla seed=1, pierwsza wartosc w Park-Miller to 16807
        assert_eq!(park_miller_next(1), 16807);
        // Druga wartosc dla seed=16807
        assert_eq!(park_miller_next(16807), 282475249);
    }

    #[test]
    fn test_random_range() {
        for _ in 0..1000 {
            let val = random_range(10, 50);
            assert!(val >= 10 && val <= 50, "Wartosc {} poza zakresem [10, 50]", val);
        }
    }

    #[test]
    fn test_frandom() {
        for _ in 0..1000 {
            let f = frandom(0.5, 2.5);
            assert!(f >= 0.5 && f <= 2.5);
        }
    }
}
