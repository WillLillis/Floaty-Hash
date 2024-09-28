use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::usize;

const F32_ERROR_TOLERANCE: f32 = 0.00001;
const F32_WIDTH: usize = 32;
const F32_EXPONENT_BITS: usize = 8;
const F32_EXPONENT_BIAS: usize = 127;
const F32_MANTISA_BITS: usize = 23;

#[derive(Debug, Copy, Clone)]
struct F32Wrapper {
    inner: f32,
}

impl PartialEq for F32Wrapper {
    fn eq(&self, other: &Self) -> bool {
        (self.inner - other.inner).abs() <= F32_ERROR_TOLERANCE
    }
}

impl Eq for F32Wrapper {}

impl F32Wrapper {
    fn new(val: f32) -> Self {
        F32Wrapper { inner: val }
    }

    fn to_bits(self) -> u32 {
        self.inner.to_bits()
    }

    #[allow(dead_code)]
    fn to_bin_str(self) -> String {
        let mut s = String::new();
        let sign_bit = self.sign_bit();
        s += &format!("0b{}", if sign_bit { 1 } else { 0 });

        for bit in self.exponent_bits() {
            s += &format!("{}", if bit { 1 } else { 0 });
        }
        for bit in self.mantissa_bits() {
            s += &format!("{}", if bit { 1 } else { 0 });
        }
        s
    }

    fn sign_bit(self) -> bool {
        (self.to_bits() & (1 << (F32_WIDTH - 1))) != 0
    }

    fn exponent_bits(self) -> [bool; F32_EXPONENT_BITS] {
        let bits = self.to_bits();
        let mut bit_selector = 1 << (F32_WIDTH - 1 - 1);
        let mut mantissa_bits = [false; F32_EXPONENT_BITS];

        for bit in mantissa_bits.iter_mut().take(F32_EXPONENT_BITS) {
            *bit = (bits & bit_selector) != 0;
            bit_selector >>= 1;
        }
        mantissa_bits
    }

    fn mantissa_bits(self) -> [bool; F32_MANTISA_BITS] {
        let bits = self.to_bits();
        let mut bit_selector = 1 << (F32_WIDTH - 1 - 1 - F32_EXPONENT_BITS);
        let mut mantissa_bits = [false; F32_MANTISA_BITS];

        for bit in mantissa_bits.iter_mut().take(F32_MANTISA_BITS) {
            *bit = (bits & bit_selector) != 0;
            bit_selector >>= 1;
        }
        mantissa_bits
    }
}

impl Hash for F32Wrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Positive and negative zero are the same value
        if self.inner != 0.0 {
            let sign_bit = self.sign_bit();
            sign_bit.hash(state);
        }

        let exponent_bits = self.exponent_bits();
        exponent_bits.hash(state);
        let mut multiplicative_exponent: f64 = {
            let mut raw_exp: i32 = 0;
            for i in 0..f32::DIGITS {
                if exponent_bits[i as usize] {
                    raw_exp += 2i32.pow(i);
                }
            }
            let exp = raw_exp - F32_EXPONENT_BIAS as i32;
            2.0f64.powi(exp)
        };

        let mantissa_bits = self.mantissa_bits();
        let mut final_mantissa = [false; F32_MANTISA_BITS];
        for i in 0usize..F32_MANTISA_BITS {
            if mantissa_bits[i] && multiplicative_exponent >= F32_ERROR_TOLERANCE.into() {
                final_mantissa[i] = true;
            }
            multiplicative_exponent /= 2.0;
        }
        final_mantissa.hash(state);
    }
}

// This kind of works but not really, there are definitely plenty of
// cases where this just breaks silently
fn main() {
    let mut floats = HashSet::new();

    println!("Let's try a quick demo of this cursed idea!");

    let f1 = F32Wrapper::new(42.0);
    let f2 = F32Wrapper::new(42.0 + F32_ERROR_TOLERANCE * 2.0);
    println!("Inserting {} and {}...", f1.inner, f2.inner);
    floats.insert(f1);
    floats.insert(f2);
    println!("Number of items in the hash set: {}", floats.len());

    println!("Ok well that makes sense, let's clear out those values");
    floats.clear();

    println!("How about this?");
    let f3 = F32Wrapper::new(42.0);
    let f4 = F32Wrapper::new(42.0 + F32_ERROR_TOLERANCE / 2.0);
    floats.insert(f3);
    floats.insert(f4);
    println!("Inserting {} and {}...", f3.inner, f4.inner);
    println!(
        "(Note that |{} - {}| = {} < {})",
        f3.inner,
        f4.inner,
        (f3.inner - f4.inner).abs(),
        F32_ERROR_TOLERANCE
    );
    println!(
        "Number of items in the hash set: {} (I'm so sorry)",
        floats.len()
    );
}

#[test]
fn it_treats_pos_and_neg_zero_the_same() {
    let pos_zero = F32Wrapper::new(0.0);
    let neg_zero = F32Wrapper::new(-0.0);

    let mut set = HashSet::new();
    set.insert(pos_zero);
    set.insert(neg_zero);

    assert!(set.len() == 1);
}
#[test]
fn it_treats_close_pos_numbers_as_the_same_1() {
    let num_1 = F32Wrapper::new(42.0);
    let num_2 = F32Wrapper::new(42.0 - F32_ERROR_TOLERANCE / 2.0);

    let mut set = HashSet::new();
    set.insert(num_1);
    set.insert(num_2);

    assert!(set.len() == 1);
}
#[test]
fn it_treats_close_pos_numbers_as_the_same_2() {
    let num_1 = F32Wrapper::new(42.0);
    let num_2 = F32Wrapper::new(42.0 + F32_ERROR_TOLERANCE / 2.0);

    let mut set = HashSet::new();
    set.insert(num_1);
    set.insert(num_2);

    assert!(set.len() == 1);
}
#[test]
fn it_treats_close_neg_numbers_as_the_same_2() {
    let num_1 = F32Wrapper::new(-42.0);
    let num_2 = F32Wrapper::new(-42.0 - F32_ERROR_TOLERANCE / 2.0);

    let mut set = HashSet::new();
    set.insert(num_1);
    set.insert(num_2);

    assert!(set.len() == 1);
}
#[test]
fn it_treats_close_neg_numbers_as_the_same_1() {
    let num_1 = F32Wrapper::new(-42.0);
    let num_2 = F32Wrapper::new(-42.0 + F32_ERROR_TOLERANCE / 2.0);

    let mut set = HashSet::new();
    set.insert(num_1);
    set.insert(num_2);

    assert!(set.len() == 1);
}
#[test]
fn it_treats_non_close_pos_numbers_as_different_1() {
    let num_1 = F32Wrapper::new(42.0);
    let num_2 = F32Wrapper::new(42.0 - F32_ERROR_TOLERANCE * 2.0);

    let mut set = HashSet::new();
    set.insert(num_1);
    set.insert(num_2);

    assert!(set.len() == 2);
}
#[test]
fn it_treats_non_close_neg_numbers_as_different_2() {
    let num_1 = F32Wrapper::new(-42.0);
    let num_2 = F32Wrapper::new(-42.0 + F32_ERROR_TOLERANCE * 2.0);

    let mut set = HashSet::new();
    set.insert(num_1);
    set.insert(num_2);

    assert!(set.len() == 2);
}
#[test]
fn it_treats_non_close_neg_numbers_as_different_1() {
    let num_1 = F32Wrapper::new(-42.0);
    let num_2 = F32Wrapper::new(-42.0 - F32_ERROR_TOLERANCE * 2.0);

    let mut set = HashSet::new();
    set.insert(num_1);
    set.insert(num_2);

    assert!(set.len() == 2);
}
#[test]
fn it_treats_non_close_pos_numbers_as_different_2() {
    let num_1 = F32Wrapper::new(42.0);
    let num_2 = F32Wrapper::new(42.0 + F32_ERROR_TOLERANCE * 2.0);

    let mut set = HashSet::new();
    set.insert(num_1);
    set.insert(num_2);

    assert!(set.len() == 2);
}
