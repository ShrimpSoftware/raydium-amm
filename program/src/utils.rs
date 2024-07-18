use std::error::Error;

pub const ACCOUNT_HEAD_PADDING: &[u8; 5] = b"serum";
pub const ACCOUNT_TAIL_PADDING: &[u8; 7] = b"padding";

pub fn strip_padding(data: &mut [u8]) -> Result<&mut [u8], Box<dyn Error>> {
    let data_len = data.len();
    let head = &data[0..ACCOUNT_HEAD_PADDING.len()];
    let tail = &data[data_len - ACCOUNT_TAIL_PADDING.len()..];

    if head == ACCOUNT_HEAD_PADDING && tail == ACCOUNT_TAIL_PADDING {
        let stripped_data =
            &mut data[ACCOUNT_HEAD_PADDING.len()..data_len - ACCOUNT_TAIL_PADDING.len()];
        Ok(stripped_data)
    } else {
        Ok(data)
    }
}
