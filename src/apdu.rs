pub mod apdu {
    pub const APDU_1: &[u8] = &[0x01, 0x02, 0x03];
    pub const APDU_2: &[u8] = &[0x02, 0x03, 0x04];
    pub const APDU_3: &[u8] = &[0x03, 0x04, 0x05];

    pub fn generate_apdu(apdu_type: u8, data: &[u8]) -> Vec<u8> {
        let mut apdu = vec![apdu_type];
        apdu.extend_from_slice(data);
        apdu
    }
}