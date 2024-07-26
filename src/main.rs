// main.rs
use crate::wallet::Wallet;
use crate::ledger::Ledger;
use crate::apdu::ApduType;

fn main() -> Result<(), MyError> {
    let wallet = Wallet::new()?;
    let secret_key = wallet.generate_secret_key()?;
    let address = wallet.create_address()?;

    let ledger = Ledger::new()?;
    let apdu = ApduType::GetBalance;
    let data = &[0x01, 0x01, 0x42];
    let response = ledger.transmit(apdu, data)?;

    println!("Response: {:?}", response);
    Ok(())
}

