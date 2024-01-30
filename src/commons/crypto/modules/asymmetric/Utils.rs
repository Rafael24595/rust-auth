use std::{fs::File, io::Read};

use reqwest::StatusCode;

use crate::commons::exception::AuthenticationApiException;

use super::{AsymmetricManager, Rsa};

pub(crate) fn find_manager(module: String, format: String, pass_phrase: String) -> Result<impl AsymmetricManager::AsymmetricManager, String> {
    match module.as_str() {
        Rsa::MODULE_CODE => {
            return Ok(Rsa::new(format.clone(), pass_phrase.clone()));
        }
        _ => {
            Err(String::from("Module not dound."))
        }
    }
}

pub(crate) fn read_key(name: String) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
    let file_path = String::from("./assets/keys/") + &name;
    let file = File::open(file_path);

    if file.is_err() {
        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), file.err().unwrap().to_string()));
    }

    let mut key = String::new();
    let result = file.unwrap().read_to_string(&mut key);

    if result.is_err() {
        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), result.err().unwrap().to_string()));
    }

    let key_clean = key
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("\n");

    return Ok(key_clean);
}