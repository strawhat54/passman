use argonautica::{Hasher, Verifier};
use magic_crypt::MagicCryptTrait;
use std::fs;

pub fn encrypt_item(master: &str, pass: &str) -> String {
    let mc = new_magic_crypt!(master, 256);
    mc.encrypt_str_to_base64(pass)
}

pub fn decrypt_item(master: &str, pass: &str) -> String {
    let mc = new_magic_crypt!(master, 256);
    mc.decrypt_base64_to_string(pass).unwrap()
}

pub fn authenticate(pass: &str, key_location: &std::path::PathBuf) -> bool {
    let master_hash = fs::read_to_string(key_location).unwrap();
    let mut verifier = Verifier::default();
    verifier
        .with_hash(&master_hash)
        .with_password(pass)
        .verify()
        .unwrap()
}

pub fn encrypt_master(pass: &str) -> String {
    let mut hasher = Hasher::default();
    hasher.opt_out_of_secret_key(true);
    hasher.with_password(pass).hash().unwrap()
}
