use hyper::{self, Body, StatusCode};

use super::result::Result;

pub struct Response {
    status: StatusCode,
    body: Option<Body>,
}

impl Response {
    pub fn new(status: StatusCode, body: Option<Body>) -> Self {
        Self {
            status: status,
            body: body,
        }
    }

    pub fn to(rst: Result<Self>) -> hyper::Response<Body> {
        let mut it = hyper::Response::new(Body::empty());
        match rst {
            Ok(r) => {
                *it.status_mut() = r.status;
                if let Some(b) = r.body {
                    *it.body_mut() = b;
                } else {
                    *it.body_mut() = Body::from(format!("{}", r.status));
                }
            }
            Err(e) => {
                *it.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                *it.body_mut() = Body::from(format!("{}", e));
            }
        }
        it
    }
}
