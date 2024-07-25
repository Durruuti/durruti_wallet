use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::Network;
use bitcoin::PrivateKey;
use bitcoin::PublicKey;
use bitcoin::Address;

fn main() {
    let secp = Secp256k1::new();
    let priv_key = ExtendedPrivKey::new_master(Network::Bitcoin, &[1; 32]).expect("Failed to create private key");
    let private_key = PrivateKey {
        compressed: true,
        inner: priv_key.private_key,
        network: Network::Bitcoin,
    };
    let pub_key = PublicKey::from_private_key(&secp, &private_key);
    let address = Address::p2pkh(&pub_key, Network::Bitcoin);

    println!("Dirección de Bitcoin: {}", address);
}