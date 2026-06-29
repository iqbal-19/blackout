pub fn md5(input: &[u8]) -> Vec<u8> {
    use md5::{Digest, Md5};

    let mut hasher = Md5::new();

    hasher.update(input);

    hasher.finalize().to_vec()
}
