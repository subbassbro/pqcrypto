use pqcrypto_falcon::falcon512::*;
use pqcrypto_traits::sign::PublicKey;
use pqcrypto_traits::sign::SignedMessage;
use pqcrypto_traits::sign::SecretKey;
fn main() {
    let message = vec![0, 1, 2, 3, 4, 5];
    let (pk, sk) = keypair();
    let sm = sign(&message, &sk);
    println!("Public Key");
    println!("{:?}", &pk.as_bytes());
    println!("Secret Key");
    println!("{:?}", &sk.as_bytes());
    println!("Signed Message");
    println!("{:?}", &sm.as_bytes());

    let verifiedmsg = open(&sm, &pk).unwrap();
    assert!(verifiedmsg != message);
}