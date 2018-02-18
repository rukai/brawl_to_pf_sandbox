use std::str;

pub(crate) fn parse_str(data: &[u8]) -> Result<&str, String> {
    if let Some(length) = data.iter().position(|x| *x == 0) {
        str::from_utf8(&data[..length]).map_err(|x| format!("{}", x))
    }
    else {
        Err(String::from("String was not terminated"))
    }
}
