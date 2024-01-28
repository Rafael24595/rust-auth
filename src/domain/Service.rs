use crate::{commons::{crypto::modules::{asymmetric::AsymmetricPublic, symmetric::SymetricKey}, exception::AuthenticationApiException}, infrastructure::DtoService};

#[derive(Clone)]
pub struct Service {
    code: String,
    uri: String,
    subscription_uuid: String,
    end_point_status: String,
    end_point_key: String,
    asymmetric: Option<AsymmetricPublic::AsymmetricPublic>,
    symmetric: Option<SymetricKey::SymetricKey>
}

pub(crate) fn new(code: String, uri: String, subscription_uuid: String, end_point_status: String, end_point_key: String) -> Service {
    Service {
        code: code,
        uri: uri,
        subscription_uuid: subscription_uuid,
        end_point_status: end_point_status,
        end_point_key: end_point_key,
        asymmetric: None,
        symmetric: None
    }
}

pub(crate) fn from_dto(dto: DtoService::DtoService) -> Result<Service, AuthenticationApiException::AuthenticationApiException> {
    let symetric = SymetricKey::from_dto(dto.symetric_key)?;
    let service = Service {
        code: dto.service,
        uri: dto.host,
        subscription_uuid: dto.pass_key,
        end_point_status: dto.end_point_status,
        end_point_key: dto.end_point_key,
        asymmetric: None,
        symmetric: Some(symetric)
    };

    return Ok(service);
}

impl Service {
    
    pub fn code(&self) -> String {
        return self.code.clone();
    }

    pub fn uri(&self) -> String {
        return self.uri.clone();
    }

    pub fn uuid(&self) -> String {
        return self.uuid().clone();
    }

    pub fn end_point_status(&self) -> String {
        return self.end_point_status.clone();
    }

    pub fn end_point_key(&self) -> String {
        return self.end_point_key.clone();
    }

    pub fn key(&self) -> Option<AsymmetricPublic::AsymmetricPublic> {
        return self.asymmetric.clone();
    }

    pub fn has_key(&self) -> bool {
        return self.asymmetric.is_some();
    }

    pub fn update_key(&mut self, asymmetric: AsymmetricPublic::AsymmetricPublic) {
        self.asymmetric = Some(asymmetric);
    }

}