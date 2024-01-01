#[derive(Copy, Clone)]
pub struct Key {
    public: &'static str,
    expires: i64
}

fn new(public: &'static str, expires: i64) -> Key {
    Key {
        public,
        expires
    }
}