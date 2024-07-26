pub mod apdu {
    pub enum ApduType {
        GetBalance,
    }

    pub const GET_BALANCE_APDU: &[u8] = &[0x02, 0x03, 0x04];

    pub fn generate_apdu(apdu_type: ApduType, data: &[u8]) -> Vec<u8> {
        match apdu_type {
            ApduType::GetBalance => {
                let mut apdu = GET_BALANCE_APDU.to_vec();
                apdu.extend_from_slice(data);
                apdu
            }
        }
    }
}