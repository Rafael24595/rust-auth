use crate::domain::Key::Key;

#[derive(Clone)]
pub struct Service {
    code: &'static str,
    uri: &'static str,
    end_point_status: &'static str,
    end_point_key: &'static str,
    key: Option<Key>
}

pub(crate) fn new(code: String, uri: String, end_point_status: String, end_point_key: String) -> Service {
    Service {
        code: Box::leak(code.into_boxed_str()),
        uri: Box::leak(uri.into_boxed_str()),
        end_point_status: Box::leak(end_point_status.into_boxed_str()),
        end_point_key: Box::leak(end_point_key.into_boxed_str()),
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

    pub fn end_point_status(&self) -> String {
        return self.end_point_status.to_string();
    }

    pub fn end_point_key(&self) -> String {
        return self.end_point_key.to_string();
    }

    pub fn key(&self) -> Option<Key> {
        return self.key.clone();
    }

    pub fn has_key(&self) -> bool {
        return self.key.is_some();
    }

    pub fn update_key(&mut self, key: Key) {
        self.key = Some(key);
    }

}