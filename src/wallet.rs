use crate::errors::MyError;

pub struct Wallet {
    secret_key: Vec<u8>,
    address: String,
    // Otros campos que necesites para tu wallet
}

impl Wallet {
    pub fn new() -> Result<Self, MyError> {
        let secret_key = Self::generate_secret_key()?;
        let address = Self::create_address(&secret_key)?;
        Ok(Wallet { secret_key, address })
    }

    pub fn generate_secret_key() -> Result<Vec<u8>, MyError> {
        // Generar una clave secreta aleatoria de 32 bytes
        let mut rng = rand::thread_rng();
        let secret_key: Vec<u8> = (0..32).map(|_| rng.gen::<u8>()).collect();
        Ok(secret_key)
    }

    pub fn create_address(secret_key: &[u8]) -> Result<String, MyError> {
        // Crear una dirección basada en la clave secreta
        let address = base58::encode(secret_key);
        Ok(address)
    }
}