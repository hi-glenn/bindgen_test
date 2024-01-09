// 使用include! 宏将生成的绑定直接转储到 crate 项目的入口中src/lib.rs
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[test]
fn test_create_pubkey() {
    // secp256k1返回公钥
    let mut pubkey: secp256k1_pubkey = secp256k1_pubkey {
        data: [0; 64],
    };
    let prikey: u8 = 1;

    unsafe {
        let context = secp256k1_context_create(SECP256K1_CONTEXT_SIGN);
        assert!(!context.is_null());
        let ret = secp256k1_ec_pubkey_create(& *context, &mut pubkey, &prikey);
        assert_eq!(ret, 1);
    }
}