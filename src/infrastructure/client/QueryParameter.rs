#[derive(Clone, Debug)]
pub struct QueryParameter {
    key: String,
    value: String
}

pub(crate) fn new(key: String, value: String) -> QueryParameter {
    QueryParameter {
        key,
        value
    }
}

pub(crate) fn from(value: &str) -> Option<QueryParameter> {
    let fragments: Vec<&str> = value.split("=").collect();
    if fragments.len() == 2 {
        return Some(new(
            fragments.get(0).unwrap().to_string(), 
            fragments.get(1).unwrap().to_string()
        ));
    }
    return None;
}

impl QueryParameter {
    
    pub fn key(&self) -> String {
        return self.key.clone();
    }

    pub fn value(&self) -> String {
        return self.value.clone();
    }

}