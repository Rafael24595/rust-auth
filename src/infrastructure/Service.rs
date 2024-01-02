use crate::domain::Auth;

pub(crate) async fn status(service: String) -> Result<(), (u16, String)> {
    let o_service = Auth::find_service(service.as_str());
    if o_service.is_some() {
        let service_data = o_service.unwrap();
        let url = service_data.uri() + &service_data.end_point_status();
        let response = reqwest::get(url).await;

        if response.is_err() {
            return Err((reqwest::StatusCode::BAD_REQUEST.as_u16(), response.err().unwrap().to_string()));
        }

        let response_data = response.unwrap();
        let status = response_data.status();
        let text = response_data.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err((status.as_u16(), text));
        } 

        return Ok(());
    }

    return Err((reqwest::StatusCode::BAD_REQUEST.as_u16(), "Service not defined".to_string()));
}