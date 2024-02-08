use super::HashInit;
use ledger_secure_sdk_sys::{cx_blake2b_init_no_throw, cx_blake2b_t, cx_hash_t};

#[derive(Default)]
#[allow(non_camel_case_types)]
pub struct Blake2b_256 {
    ctx: cx_blake2b_t,
}

impl HashInit for Blake2b_256 {
    fn as_ctx_mut(&mut self) -> &mut cx_hash_t {
        &mut self.ctx.header
    }

    fn as_ctx(&self) -> &cx_hash_t {
        &self.ctx.header
    }

    fn new() -> Self {
        let mut ctx: Blake2b_256 = Default::default();
        let _err = unsafe { cx_blake2b_init_no_throw(&mut ctx.ctx, 256) };
        ctx
    }
}

#[derive(Default)]
#[allow(non_camel_case_types)]
pub struct Blake2b_384 {
    ctx: cx_blake2b_t,
}

impl HashInit for Blake2b_384 {
    fn as_ctx_mut(&mut self) -> &mut cx_hash_t {
        &mut self.ctx.header
    }

    fn as_ctx(&self) -> &cx_hash_t {
        &self.ctx.header
    }

    fn new() -> Self {
        let mut ctx: Blake2b_384 = Default::default();
        let _err = unsafe { cx_blake2b_init_no_throw(&mut ctx.ctx, 384) };
        ctx
    }
}

#[derive(Default)]
#[allow(non_camel_case_types)]
pub struct Blake2b_512 {
    ctx: cx_blake2b_t,
}

impl HashInit for Blake2b_512 {
    fn as_ctx_mut(&mut self) -> &mut cx_hash_t {
        &mut self.ctx.header
    }

    fn as_ctx(&self) -> &cx_hash_t {
        &self.ctx.header
    }

    fn new() -> Self {
        let mut ctx: Blake2b_512 = Default::default();
        let _err = unsafe { cx_blake2b_init_no_throw(&mut ctx.ctx, 512) };
        ctx
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_eq_err as assert_eq;
    use crate::hash::blake2::*;
    use crate::testing::TestType;
    use testmacro::test_item as test;

    const TEST_HASH: &[u8; 29] = b"Not your keys, not your coins";

    #[test]
    fn test_hash_blake2b256() {
        let mut blake2 = Blake2b_256::new();

        let mut output: [u8; 32] = [0u8; 32];

        let ouput_size = blake2.get_size();
        assert_eq!(ouput_size, 32);

        let _ = blake2.hash(TEST_HASH, &mut output);

        let expected = [
            0xcd, 0xa6, 0x49, 0x8e, 0x2f, 0x89, 0x71, 0xe8, 0x4e, 0xd5, 0x68, 0x2e, 0x3d, 0x47,
            0x9c, 0xcc, 0x2c, 0xce, 0x7f, 0x37, 0xac, 0x92, 0x9c, 0xa0, 0xb0, 0x41, 0xb2, 0xdd,
            0x06, 0xa9, 0xf3, 0xcb,
        ];
        assert_eq!(&output, &expected);
    }

    #[test]
    fn test_hash_blake2b384() {
        let mut blake2 = Blake2b_384::new();

        let mut output: [u8; 48] = [0u8; 48];

        let ouput_size = blake2.get_size();
        assert_eq!(ouput_size, 48);

        let _ = blake2.hash(TEST_HASH, &mut output);

        let expected = [
            0x5f, 0x03, 0x04, 0x77, 0x92, 0x5e, 0x91, 0x29, 0xf9, 0xb8, 0xef, 0xf9, 0x88, 0x29,
            0x04, 0xf4, 0x4f, 0x65, 0x3b, 0xef, 0xf8, 0x21, 0xca, 0x48, 0x68, 0xa7, 0xbe, 0x46,
            0x1c, 0x45, 0x82, 0xb3, 0x3d, 0xd7, 0x7b, 0x9e, 0x91, 0x9a, 0xfe, 0x1c, 0x3b, 0xed,
            0x4b, 0x8f, 0x3c, 0x5d, 0xde, 0x53,
        ];
        assert_eq!(&output, &expected);
    }

    #[test]
    fn test_hash_blake2b512() {
        let mut blake2 = Blake2b_512::new();

        let mut output: [u8; 64] = [0u8; 64];

        let ouput_size = blake2.get_size();
        assert_eq!(ouput_size, 64);

        let _ = blake2.hash(TEST_HASH, &mut output);

        let expected = [
            0xc2, 0xe0, 0xfe, 0x8c, 0xb7, 0x83, 0x43, 0x7c, 0x8f, 0x36, 0x89, 0x48, 0xc4, 0x7a,
            0x9c, 0x7c, 0x27, 0xa3, 0xb5, 0x98, 0x7a, 0x2d, 0x1b, 0x3b, 0xab, 0x48, 0x3d, 0xd6,
            0xf6, 0x4c, 0xd1, 0x20, 0x7d, 0x72, 0x62, 0xb5, 0x35, 0xfe, 0x3f, 0x86, 0xad, 0x0c,
            0x5f, 0x33, 0x4e, 0x55, 0x07, 0x64, 0x49, 0x7c, 0x11, 0xd5, 0xbd, 0x6a, 0x44, 0x2a,
            0x9c, 0x2e, 0x6a, 0xab, 0xf9, 0x31, 0xc0, 0xab,
        ];
        assert_eq!(&output, &expected);
    }
}
