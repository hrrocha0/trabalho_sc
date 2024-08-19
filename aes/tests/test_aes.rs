use common::file;

#[test]
fn test_cipher() {
    let plaintext = file::read_file("tests/files/plaintext.txt").content;
    let expected = file::read_file("tests/files/ciphertext.txt").content;
    let key = 0x0f0e0d0c0b0a09080706050403020100;
    let offset = 0x00000000000000000000000000000000;

    let ciphertext = aes::ctr::cipher(&plaintext, key, offset, 10);

    assert_eq!(expected, ciphertext);
}

#[test]
fn test_decipher() {
    let ciphertext = file::read_file("tests/files/ciphertext.txt").content;
    let expected = file::read_file("tests/files/plaintext.txt").content;
    let key = 0x0f0e0d0c0b0a09080706050403020100;
    let offset = 0x00000000000000000000000000000000;

    let plaintext = aes::ctr::cipher(&ciphertext, key, offset, 10);

    assert_eq!(expected, plaintext);
}
