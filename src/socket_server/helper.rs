use salsa20::{Salsa20, Key, Nonce};
use salsa20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use std::convert::TryInto;

pub fn salsa20_dec(dat: &[u8]) -> Vec<u8> {
    let key = b"Simulator Interface Packet GT7 ver 0.0";
    // Seed IV is always located here
    let oiv: &[u8] = &dat[0x40..0x44];
    let iv1 = u32::from_le_bytes(oiv.try_into().expect("Invalid IV length"));
    
    // Notice DEADBEAF, not DEADBEEF
    let iv2 = iv1 ^ 0xDEADBEAF;
    
    // Construct the full IV
    let mut iv = Vec::new();
    iv.extend(&iv2.to_le_bytes());
    iv.extend(&iv1.to_le_bytes());

    // Key and IV must be references to the `GenericArray` type.
    // Here we use the `Into` trait to convert arrays into it.
    let mut cipher = Salsa20::new(Key::from_slice(&key[..32]), Nonce::from_slice(&iv[..8]));
    
    // Decrypt the data
    let mut ddata = dat.to_vec();
    cipher.apply_keystream(&mut ddata);
    
    // Check the magic number
    let magic = u32::from_le_bytes(ddata[0..4].try_into().expect("Invalid magic length"));
    if magic != 0x47375330 {
        return Vec::new(); // Return an empty vector if the magic number doesn't match
    }
    
    ddata // Return the decrypted data
}
