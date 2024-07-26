pub mod ledger_mock {
    use std::collections::HashMap;

    pub struct LedgerMock {
        apdu_responses: HashMap<Vec<u8>, Vec<u8>>,
    }

    impl LedgerMock {
        pub fn new() -> Self {
            LedgerMock {
                apdu_responses: HashMap::new(),
            }
        }

        pub fn set_apdu_response(&mut self, apdu: Vec<u8>, response: Vec<u8>) {
            self.apdu_responses.insert(apdu, response);
        }

        pub fn get_apdu_response(&self, apdu: &Vec<u8>) -> Option<&Vec<u8>> {
            self.apdu_responses.get(apdu)
        }

        pub fn transmit(&mut self, apdu: Vec<u8>) -> Vec<u8> {
            self.apdu_responses.get(&apdu).cloned().unwrap_or_else(|| vec![0x6A, 0x86])
        }
    }
}