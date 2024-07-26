// wallet.rs
use rand::Rng;
use ripemd160::{Ripemd160, Digest};
use sha2::{Sha256, Digest as Sha256Digest};
use base58;

pub struct Wallet {
    secret_key: Vec<u8>,
    address: String,
}

impl Wallet {
    pub fn new() -> Result<Wallet, MyError> {
        let mut rng = rand::thread_rng();
        let secret_key: Vec<u8> = rng.gen::<[u8; 32]>().to_vec();
        let address = self.generate_address(&secret_key)?;
        Ok(Wallet { secret_key, address })
    }

    fn generate_address(&self, secret_key: &[u8]) -> Result<String, MyError> {
        // 1. Take the SHA-256 hash of the secret key
        let sha256 = Sha256::digest(secret_key);
        // 2. Take the RIPEMD-160 hash of the SHA-256 hash
        let ripemd160 = Ripemd160::digest(&sha256);
        // 3. Add the network byte (0x00 for mainnet, 0x6f for testnet)
        let mut payload = vec![0x00]; // mainnet
        payload.extend_from_slice(&ripemd160);
        // 4. Calculate the checksum (first 4 bytes of the SHA-256 hash of the payload)
        let checksum = Sha256::digest(&payload)[..4].to_vec();
        // 5. Concatenate the payload and checksum
        let mut address_bytes = payload;
        address_bytes.extend_from_slice(&checksum);
        // 6. Encode the address bytes in Base58
        let address = base58::encode(address_bytes);
        Ok(address)
    }

    pub fn get_secret_key(&self) -> Vec<u8> {
        self.secret_key.clone()
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }
}