use bitcoin::{PrivateKey, PublicKey, Address, Network};
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use rand::Rng;



mod ledger_mock;
mod apdu;

use crate::apdu::apdu::ApduType;
//use crate::apdu::apdu::{SELECT_APPLET_APDU, GET_BALANCE_APDU, GET_TRANSACTION_HISTORY_APDU};
use apdu::apdu::generate_apdu;
use ledger_mock::ledger_mock::LedgerMock;


#[derive(Debug)]
enum MyError {
    BitcoinError(()),
}

impl From<bitcoin::secp256k1::Error> for MyError {
    fn from(_err: bitcoin::secp256k1::Error) -> Self {
        MyError::BitcoinError(())
    }
}

const RESPONSE_1: [u8; 32] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14,
    0x15, 0x16, 0x17, 0x18, 0x19, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    0x29, 0x30, 0x31, 0x32,
];

//const APDU_1: &[u8] = crate::apdu::apdu::SELECT_APPLET_APDU;
const APDU_2: &[u8] = crate::apdu::apdu::GET_BALANCE_APDU;
// const APDU_3: &[u8] = crate::apdu::apdu::GET_TRANSACTION_HISTORY_APDU;

fn main() -> Result<(), MyError> {
    println!("{:?}", APDU_2);
    let secp = Secp256k1::new();

    let mut ledger = LedgerMock::new();

    let response = RESPONSE_1.to_vec();
    ledger.set_apdu_response(APDU_2.to_vec(), response.clone());
    ledger.set_apdu_response(APDU_2.to_vec(), vec![0x90, 0x01]);

    let apdu = generate_apdu(ApduType::GetBalance, &[0x01, 0x01, 0x42]);
    let _response = ledger.transmit(apdu);

    let mut rng = rand::thread_rng();
    let secret_key_bytes: [u8; 32] = rng.gen();
    let secret_key = SecretKey::from_slice(&secret_key_bytes)?;

    let private_key = PrivateKey::new(secret_key, Network::Bitcoin);
    let public_key = PublicKey::from_private_key(&secp, &private_key);
    let address = Address::p2pkh(&public_key, Network::Bitcoin);

    println!("Dirección de Bitcoin (Ledger): {}", address);

    let response2 = ledger.transmit(APDU_2.to_vec());
    assert_eq!(response2, vec![0x90, 0x01]);

    let _response3 = ledger;

    return Ok(())
}

