use rocket::{
    Request, Response,
    fairing::{Fairing, Info, Kind},
    http::Header,
};

#[rocket::options("/delete_blockchain")]
pub fn options_delete_blockchain() -> &'static str {
    ""
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));

           // Required for OPTIONS preflight to succeed
           if request.method().as_str() == "OPTIONS" {
            response.set_status(rocket::http::Status::Ok);
        }
    }
}
