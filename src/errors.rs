pub enum MyError {
    BitcoinError,
    LedgerError,
    RngError,
    // Agregar más errores según sea necesario
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::BitcoinError => write!(f, "Bitcoin error"),
            MyError::LedgerError => write!(f, "Ledger error"),
            MyError::RngError => write!(f, "RNG error"),
            // Agregar más casos según sea necesario
        }
    }
}

impl std::error::Error for MyError {}