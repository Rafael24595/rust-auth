#[derive(Clone, Debug)]
pub struct HeaderParameter {
    key: String,
    values: Vec<String>
}

pub(crate) fn new(key: String, value: String) -> HeaderParameter {
    HeaderParameter {
        key: key,
        values: Vec::from([value])
    }
}

impl HeaderParameter {
    
    pub fn key(&self) -> String {
        return self.key.clone();
    }

    pub fn values(&self) -> Vec<String> {
        return self.values.clone();
    }

    pub fn addValue(&mut self, value: String) {
        self.values.push(value);
    }

    pub fn addValues(&mut self, mut values: Vec<String>) {
        self.values.append(&mut values);
    }

}