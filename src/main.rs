// Simulador mock para el dispositivo Ledger
// Desactivar cuando se quiera utilizar el dispositivo físico

use bitcoin::{PrivateKey, PublicKey, Address, Network};
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use rand::Rng;

// Módulo para el simulador mock del dispositivo Ledger
mod ledger_mock {
    use std::collections::HashMap;

    // Estructura para el simulador mock del dispositivo Ledger
    pub struct LedgerMock {
        apdu_responses: HashMap<Vec<u8>, Vec<u8>>,
    }

    impl LedgerMock {
        // Constructor para el simulador mock
        pub fn new() -> Self {
            LedgerMock {
                apdu_responses: HashMap::new(),
            }
        }

        // Método para establecer una respuesta personalizada para un APDU
        pub fn set_apdu_response(&mut self, apdu: Vec<u8>, response: Vec<u8>) {
            self.apdu_responses.insert(apdu, response);
        }

        // Método para transmitir un APDU y obtener la respuesta correspondiente
        pub fn transmit(&mut self, apdu: Vec<u8>) -> Vec<u8> {
            self.apdu_responses.get(&apdu).cloned().unwrap_or_else(|| vec![0x6A, 0x86])
        }
    }
}

// APDUs de prueba
const APDU1: &[u8] = &[0x01, 0x02, 0x03];
const APDU2: &[u8] = &[0x02, 0x03, 0x04];
const APDU3: &[u8] = &[0x03, 0x04, 0x05];

#[derive(Debug)]
enum MyError {
    BitcoinError(()),
}

impl From<bitcoin::secp256k1::Error> for MyError {
    fn from(_err: bitcoin::secp256k1::Error) -> Self {
        MyError::BitcoinError(())
    }
}



fn main() -> Result<(), MyError> {
    let secp = Secp256k1::new();

    // Crear instancia del simulador mock del dispositivo Ledger
    let mut ledger = ledger_mock::LedgerMock::new();

    // Establecer respuestas personalizadas para los APDUs de prueba
    let response = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x30, 0x31, 0x32];
    ledger.set_apdu_response(APDU1.to_vec(), response.clone());
    ledger.set_apdu_response(APDU2.to_vec(), vec![0x90, 0x01]);

    // Simular el envío de comandos y lectura de respuestas
    let apdu = [0x02, 0x01, 0x01, 0x42];
    let __response = ledger.transmit(apdu.to_vec());

    let mut rng = rand::thread_rng();
    let secret_key_bytes: [u8; 32] = rng.gen();
    let secret_key = SecretKey::from_slice(&secret_key_bytes)?;

    // Crear clave privada desde la clave secreta
    let private_key = PrivateKey::new(secret_key, Network::Bitcoin);

    // Crear clave pública desde la clave privada
    let public_key = PublicKey::from_private_key(&secp, &private_key);

    // Crear dirección de Bitcoin desde la clave pública
    let address = Address::p2pkh(&public_key, Network::Bitcoin);

    // Imprimir dirección de Bitcoin
    println!("Dirección de Bitcoin (Ledger): {}", address);

    // Probar el simulador mock con los APDUs de prueba
    let response2 = ledger.transmit(APDU2.to_vec());
    assert_eq!(response2, vec![0x90, 0x01]);

    let response3 = ledger.transmit(APDU3.to_vec());
    assert_eq!(response3, vec![0x6A, 0x86]);

    Ok(())
}

