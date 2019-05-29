use secp256k1::{verify, Message, Signature, PublicKey, PublicKeyFormat};
use blake2_rfc::blake2b::Blake2b;
use super::ckb_load_tx_hash;

pub fn verify_sighash_all(binary_pubkey_hash: &[u8],
                              binary_pubkey: &[u8],
                              binary_signature: &[u8]) -> i32
{
    if binary_pubkey.len() != 33 {
        return -60;
    }
    if binary_pubkey_hash.len() != 20 {
        return -61;
    }

    let mut state = Blake2b::default();
    state.update(binary_pubkey);
    let ret = state.finalize();
    let hash = ret.as_bytes();
    for i in 0..20 {
        if binary_pubkey_hash[i] != hash[i] {
            return -62;
        }
    }

    let mut tx_hash = [0u8;32];
    let mut len: u64 = 32;
    unsafe {
        if ckb_load_tx_hash(&mut tx_hash, &mut len, 0) != 0 {
            return -63;
        }
    }
    let message = Message::parse_slice(&tx_hash).unwrap();
    let signature = Signature::parse_der(binary_signature).unwrap();
    let pubkey = PublicKey::parse_slice(binary_pubkey, Some(PublicKeyFormat::Compressed)).unwrap();

    if !verify(&message, &signature, &pubkey) {
        return -64;
    }
    return 0;
}