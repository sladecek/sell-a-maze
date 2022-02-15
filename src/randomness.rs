use num_bigint::BigUint;

pub struct Randomness {
    remainder : BigUint,
    consumed: BigUint
}

impl Randomness {
    pub fn new(seed: BigUint, size: BigUint)->Self {
        let remainder = seed.clone() * size + seed;
        Randomness { remainder, consumed: BigUint::from(1u32) }
    }

    pub fn get(&mut self, max: u32) -> u32 {
        let res_u32s = (self.remainder.clone() % max).to_u32_digits();
        let result = if res_u32s.len() == 0 { 0u32 } else {res_u32s[0]};
        self.remainder /= max;
        self.consumed *= max;
        result
    }
}

#[test]
fn test_getting() {
    let mut r = Randomness::new(BigUint::from(3u32), BigUint::from(8u32));
    assert_eq!(1u32, r.get(2));
    assert_eq!(1u32, r.get(2));
    assert_eq!(0u32, r.get(2));
    assert_eq!(1u32, r.get(2));
    assert_eq!(1u32, r.get(2));
    assert_eq!(0u32, r.get(2));
    assert_eq!(0u32, r.get(2));
    assert_eq!(0u32, r.get(2));
}

