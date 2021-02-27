use generic_array::arr;
use generic_array::GenericArray;
use sha2::Digest;
use sha2::Sha256;
use std::convert::TryInto;
use std::io::Write;
use std::mem;
use std::u8;
use typenum::{U16, U2, U32};

const OPTIX_KEY: &'static [u8] =
    b"-3343556356fgfgfdessss-(--9355-489795-2333354:[]}}{[]523552%GWEf";

const REMAINDER_KEY: [u8; 7] = [164, 195, 147, 255, 203, 161, 184];

pub(crate) fn decode_ptx<'a>(
    content: &'a mut [u8],
    optix_salt: &[u8],
    vendor_salt: &[u8],
    vendor_key: &[u8],
) -> &'a mut [u8] {
    let content = normalize(content);
    let concatenated_key = [&vendor_key[..], &OPTIX_KEY[..]].concat();
    let hashed_key = sha256(&concatenated_key[..]);
    let hexed_key = to_hex_string(hashed_key);
    let session_key_input = [&optix_salt[..], &hexed_key[..], &vendor_salt[..]].concat();
    let mut hashed_session_key = sha256(&session_key_input[..]);
    let reduced_key = reduce_key(&mut hashed_session_key);
    decode(content, &*reduced_key);
    decode_remainder(content);
    content
}

fn decode(content: &mut [u8], reduced_key: &GenericArray<u8, U16>) {
    for i in 0..content.len() / 8 {
        let block = &mut content[i * 8..i * 8 + 8];
        unsafe { decrypt_block(block.try_into().unwrap(), reduced_key) };
    }
}

fn decode_remainder(content: &mut [u8]) {
    let remainer_start = (content.len() / 8) * 8;
    for i in 0..content.len() % 8 {
        content[remainer_start + i] = content[remainer_start + i] ^ REMAINDER_KEY[i];
    }
}

fn sha256(content: &[u8]) -> GenericArray<u8, U32> {
    let mut hasher = Sha256::new();
    hasher.update(content);
    hasher.finalize()
}

fn to_hex_string(hash: GenericArray<u8, U32>) -> [u8; 64] {
    let mut result = [0u8; 64];
    for (idx, c) in hash.into_iter().enumerate() {
        write!(&mut result[idx * 2..idx * 2 + 2], "{:02x}", c).unwrap();
    }
    result
}

fn reduce_key<'a>(content: &'a mut GenericArray<u8, U32>) -> &'a mut GenericArray<u8, U16> {
    for i in 0usize..16 {
        content[i] = content[i].wrapping_add(content[i + 16]);
    }
    GenericArray::from_mut_slice(&mut content.as_mut_slice()[..16])
}

unsafe fn decrypt_block(block: &mut [u8; 8], key: &GenericArray<u8, U16>) {
    let delta: u32 = 0x9E3779B9;
    let mut sum: u32 = 0xE3779B90;
    let v = mem::transmute::<[u8; 8], [u32; 2]>(*block);
    let mut v0 = v[0];
    let mut v1 = v[1];
    let key = key.as_ptr() as *const u32;
    let k0 = *key.offset(0);
    let k1 = *key.offset(1);
    let k2 = *key.offset(2);
    let k3 = *key.offset(3);
    for _ in 0..16 {
        v1 = v1.wrapping_sub(
            ((v0 << 4).wrapping_add(k2)) ^ (v0.wrapping_add(sum)) ^ ((v0 >> 5).wrapping_add(k3)),
        );
        v0 = v0.wrapping_sub(
            ((v1 << 4).wrapping_add(k0)) ^ (v1.wrapping_add(sum)) ^ ((v1 >> 5).wrapping_add(k1)),
        );
        sum = sum.wrapping_sub(delta)
    }
    *block = std::mem::transmute::<GenericArray<u32, U2>, [u8; 8]>(arr![u32; v0, v1]);
}

fn normalize<'a>(content: &'a mut [u8]) -> &'a mut [u8] {
    let mut to = 0;
    let mut from = 8;
    loop {
        if from >= content.len() {
            break;
        }
        let mut c = content[from];
        if c == 1 {
            from += 1;
            c = content[from] - 1;
        }
        content[to] = c;
        from += 1;
        to += 1;
    }
    &mut content[0..to]
}
