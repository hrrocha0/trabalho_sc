use aes::{Block, Key};
use common::file;

#[test]
fn test_cipher() {
    let plaintext = file::read_file("tests/files/plaintext.txt").content;
    let expected = file::read_file("tests/files/ciphertext.bin").content;
    let iv: Block = [0x00; 16];

    let key: Key = [
        0x00, 0x01, 0x02, 0x03,
        0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0a, 0x0b,
        0x0c, 0x0d, 0x0e, 0x0f,
    ];
    let ciphertext = aes::ctr::cipher(&plaintext, &key, &iv, 10);

    assert_eq!(expected, ciphertext);
}

#[test]
fn test_decipher() {
    let ciphertext = file::read_file("tests/files/ciphertext.bin").content;
    let expected = file::read_file("tests/files/plaintext.txt").content;
    let initial_vector: Block = [0x00; 16];

    let key: Key = [
        0x00, 0x01, 0x02, 0x03,
        0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0a, 0x0b,
        0x0c, 0x0d, 0x0e, 0x0f,
    ];
    let plaintext = aes::ctr::cipher(&ciphertext, &key, &initial_vector, 10);

    assert_eq!(expected, plaintext);
}
