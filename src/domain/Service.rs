use crate::domain::Key::Key;

#[derive(Copy, Clone)]
pub struct Service {
    code: &'static str,
    uri: &'static str,
    key: Option<Key>
}

pub(crate) fn new(code: String, uri: String) -> Service {
    let static_code: &'static str = Box::leak(code.into_boxed_str());
    let static_uri: &'static str = Box::leak(uri.into_boxed_str());
    Service {
        code: static_code,
        uri: static_uri,
        key: None
    }
}

impl Service {
    
    pub fn code(&self) -> String {
        return self.code.to_string();
    }

    pub fn uri(&self) -> String {
        return self.uri.to_string();
    }

}