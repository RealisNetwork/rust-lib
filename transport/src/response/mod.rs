use crate::response::stan::Response;

pub mod stan;

#[derive(Debug)]
pub enum VResponse {
    Response(Response),
}

impl From<Response> for VResponse {
    fn from(response: Response) -> Self {
        VResponse::Response(response)
    }
}


