use super::F;
use zerocopy::AsBytes;

pub trait OffsetToBytes {
    const IS_SIGNED: bool;
    fn min_as_fr() -> F;
    fn offset_to_bytes(&self) -> Vec<u8>;
}

impl OffsetToBytes for u8 {
    const IS_SIGNED: bool = false;

    fn min_as_fr() -> F {
        F::from(0)
    }

    fn offset_to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl OffsetToBytes for i16 {
    const IS_SIGNED: bool = true;

    fn min_as_fr() -> F {
        F::from(i16::MIN)
    }

    fn offset_to_bytes(&self) -> Vec<u8> {
        let shifted = self.wrapping_sub(i16::MIN);
        shifted.to_le_bytes().to_vec()
    }
}

impl OffsetToBytes for i32 {
    const IS_SIGNED: bool = true;

    fn min_as_fr() -> F {
        F::from(i32::MIN)
    }

    fn offset_to_bytes(&self) -> Vec<u8> {
        let shifted = self.wrapping_sub(i32::MIN);
        shifted.to_le_bytes().to_vec()
    }
}

impl OffsetToBytes for i64 {
    const IS_SIGNED: bool = true;

    fn min_as_fr() -> F {
        F::from(i64::MIN)
    }

    fn offset_to_bytes(&self) -> Vec<u8> {
        let shifted = self.wrapping_sub(i64::MIN);
        shifted.to_le_bytes().to_vec()
    }
}

impl OffsetToBytes for i128 {
    const IS_SIGNED: bool = true;

    fn min_as_fr() -> F {
        F::from(i128::MIN)
    }

    fn offset_to_bytes(&self) -> Vec<u8> {
        let shifted = self.wrapping_sub(i128::MIN);
        shifted.to_le_bytes().to_vec()
    }
}

impl OffsetToBytes for bool {
    const IS_SIGNED: bool = false;

    fn min_as_fr() -> F {
        F::from(false)
    }

    fn offset_to_bytes(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

impl OffsetToBytes for u64 {
    const IS_SIGNED: bool = false;

    fn min_as_fr() -> F {
        F::from(0)
    }

    fn offset_to_bytes(&self) -> Vec<u8> {
        let bytes = self.to_le_bytes();
        bytes.to_vec()
    }
}

impl OffsetToBytes for [u64; 4] {
    const IS_SIGNED: bool = false;

    fn min_as_fr() -> F {
        F::from(0)
    }

    fn offset_to_bytes(&self) -> Vec<u8> {
        let slice = self.as_bytes();
        slice.to_vec()
    }
}
