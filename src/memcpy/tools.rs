use blake2::{Blake2b, Digest};

pub fn bytes(count: usize) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut seed = Vec::from(b"coconut".as_ref());
    loop {
        let mut hasher = Blake2b::new();
        hasher.input(&seed);
        let new = hasher.result();
        buf.extend(&new);
        seed = new.to_vec();
        if buf.len() >= count {
            break;
        }
    }

    buf.truncate(count);

    buf
}
