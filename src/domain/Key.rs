use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Copy, Clone, Debug)]
pub struct Key {
    public: &'static str,
    expires: u128
}

pub(crate) fn new(public: String, expires: u128) -> Key {
    Key {
        public: Box::leak(public.into_boxed_str()),
        expires: expires
    }
}

impl Key {
    
    pub fn key(&self) -> String {
        return self.public.to_string();
    }

    pub fn is_expired(&self) -> bool {
        let current_time = SystemTime::now();
        let duration_since_epoch = current_time.duration_since(UNIX_EPOCH).expect("Critical error.");
        let milliseconds = duration_since_epoch.as_millis();

        return self.expires < milliseconds;
    }
    
}