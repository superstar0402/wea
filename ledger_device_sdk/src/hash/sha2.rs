use super::HashInit;
use ledger_secure_sdk_sys::{
    cx_hash_t, cx_sha224_init_no_throw, cx_sha256_init_no_throw, cx_sha256_t,
    cx_sha384_init_no_throw, cx_sha512_init_no_throw, cx_sha512_t,
};

#[derive(Default)]
pub struct Sha2_224 {
    ctx: cx_sha256_t,
}

impl HashInit for Sha2_224 {
    fn as_ctx_mut(&mut self) -> &mut cx_hash_t {
        &mut self.ctx.header
    }

    fn as_ctx(&self) -> &cx_hash_t {
        &self.ctx.header
    }

    fn new() -> Self {
        let mut ctx: Sha2_224 = Default::default();
        let _err = unsafe { cx_sha224_init_no_throw(&mut ctx.ctx) };
        ctx
    }
}

#[derive(Default)]
pub struct Sha2_256 {
    ctx: cx_sha256_t,
}

impl HashInit for Sha2_256 {
    fn as_ctx_mut(&mut self) -> &mut cx_hash_t {
        &mut self.ctx.header
    }

    fn as_ctx(&self) -> &cx_hash_t {
        &self.ctx.header
    }

    fn new() -> Self {
        let mut ctx: Sha2_256 = Default::default();
        let _err = unsafe { cx_sha256_init_no_throw(&mut ctx.ctx) };
        ctx
    }
}

#[derive(Default)]
pub struct Sha2_384 {
    ctx: cx_sha512_t,
}

impl HashInit for Sha2_384 {
    fn as_ctx_mut(&mut self) -> &mut cx_hash_t {
        &mut self.ctx.header
    }

    fn as_ctx(&self) -> &cx_hash_t {
        &self.ctx.header
    }

    fn new() -> Self {
        let mut ctx: Sha2_384 = Default::default();
        let _err = unsafe { cx_sha384_init_no_throw(&mut ctx.ctx) };
        ctx
    }
}

#[derive(Default)]
pub struct Sha2_512 {
    ctx: cx_sha512_t,
}

impl HashInit for Sha2_512 {
    fn as_ctx_mut(&mut self) -> &mut cx_hash_t {
        &mut self.ctx.header
    }

    fn as_ctx(&self) -> &cx_hash_t {
        &self.ctx.header
    }

    fn new() -> Self {
        let mut ctx: Sha2_512 = Default::default();
        let _err = unsafe { cx_sha512_init_no_throw(&mut ctx.ctx) };
        ctx
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_eq_err as assert_eq;
    use crate::hash::sha2::*;
    use crate::testing::TestType;
    use testmacro::test_item as test;

    const TEST_HASH: &[u8; 29] = b"Not your keys, not your coins";

    #[test]
    fn test_hash_sha2224() {
        let mut sha2 = Sha2_224::new();

        let mut output: [u8; 28] = [0u8; 28];

        let ouput_size = sha2.get_size();
        assert_eq!(ouput_size, 28);

        let _ = sha2.hash(TEST_HASH, &mut output);

        let expected = [
            0x5a, 0x5b, 0xea, 0xa1, 0x3f, 0x5d, 0xf3, 0xd8, 0x5a, 0xc8, 0x62, 0x44, 0x95, 0x9b,
            0xa2, 0x8e, 0xed, 0x08, 0x65, 0xa2, 0xcd, 0x10, 0xd1, 0x5c, 0xce, 0x47, 0x9a, 0x2a,
        ];
        assert_eq!(&output, &expected);
    }

    #[test]
    fn test_hash_sha2256() {
        let mut sha2 = Sha2_256::new();

        let mut output: [u8; 32] = [0u8; 32];

        let ouput_size = sha2.get_size();
        assert_eq!(ouput_size, 32);

        let _ = sha2.hash(TEST_HASH, &mut output);

        let expected = [
            0x52, 0x49, 0x2e, 0x81, 0x92, 0x16, 0xf3, 0x6b, 0x74, 0x7d, 0xd5, 0xda, 0x70, 0x3a,
            0x26, 0x60, 0x14, 0x34, 0x60, 0x42, 0x42, 0xfa, 0xb2, 0x7e, 0x85, 0x51, 0xe7, 0x82,
            0xa5, 0x11, 0x13, 0x40,
        ];
        assert_eq!(&output, &expected);
    }

    #[test]
    fn test_hash_sha2384() {
        let mut sha2 = Sha2_384::new();

        let mut output: [u8; 48] = [0u8; 48];

        let ouput_size = sha2.get_size();
        assert_eq!(ouput_size, 48);

        let _ = sha2.hash(TEST_HASH, &mut output);

        let expected = [
            0x11, 0xe3, 0xe7, 0xec, 0x0d, 0xc5, 0x81, 0x87, 0x8c, 0x35, 0xc6, 0xc8, 0x07, 0x15,
            0x65, 0x53, 0x26, 0x1d, 0xb1, 0x7e, 0x32, 0x8c, 0xf8, 0x7d, 0x37, 0xbe, 0x05, 0x35,
            0xf8, 0x45, 0x8d, 0x7c, 0xc9, 0x15, 0x74, 0xa2, 0x3f, 0x4f, 0x3e, 0x5f, 0x98, 0x23,
            0xc7, 0xaa, 0x3a, 0xff, 0xf1, 0x59,
        ];
        assert_eq!(&output, &expected);
    }

    #[test]
    fn test_hash_sha2512() {
        let mut sha2 = Sha2_512::new();

        let mut output: [u8; 64] = [0u8; 64];

        let ouput_size = sha2.get_size();
        assert_eq!(ouput_size, 64);

        let _ = sha2.hash(TEST_HASH, &mut output);

        let expected = [
            0xf0, 0xe9, 0x96, 0x75, 0x81, 0xc0, 0xdb, 0x4c, 0x8e, 0xc0, 0xeb, 0xb2, 0x53, 0xa7,
            0xff, 0x8d, 0x8a, 0x1a, 0x69, 0x06, 0xbc, 0x1b, 0x76, 0x0c, 0x23, 0x09, 0x9c, 0xc5,
            0xe4, 0xf7, 0xea, 0x19, 0x07, 0x73, 0x57, 0x07, 0x8a, 0x66, 0x6b, 0x45, 0x1c, 0xa2,
            0x32, 0xa4, 0xa7, 0x0c, 0xa1, 0x8d, 0xaa, 0x4e, 0xd0, 0x5a, 0xdd, 0x03, 0x02, 0x05,
            0x04, 0xdf, 0xdd, 0x93, 0x1d, 0x54, 0x6f, 0xfd,
        ];
        assert_eq!(&output, &expected);
    }
}
