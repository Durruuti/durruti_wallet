mod wallet;
mod ledger_mock;
mod apdu;
mod errors;

use wallet::wallet::Wallet;
use ledger_mock::ledger_mock::LedgerMock;
use apdu::apdu::ApduType;
use errors::errors::MyError;

fn main() -> Result<(), MyError> {
    let mut ledger = LedgerMock::new();
    ledger.set_apdu_response(GET_BALANCE_APDU.to_vec(), vec![0x01, 0x01, 0x42]);

    let wallet = Wallet::new()?;
    let secret_key = wallet.generate_secret_key()?;
    let address = wallet.create_address()?;

    let apdu = ApduType::GetBalance;
    let data = &[0x01, 0x01, 0x42];
    let response = ledger.transmit(Vec<u8>)?;

    println!("Response: {:?}", response);
    Ok(())
}