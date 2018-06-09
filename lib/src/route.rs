use hyper::Method;

pub struct Route {
    method: Method,
    pattern: String,
}

impl Route {
    pub fn new(m: Method, p: String) -> Self {
        Self {
            method: m,
            pattern: p,
        }
    }
    pub fn is_match<'a>(&self, method: &'a Method, path: &'a str) -> bool {
        if method != &self.method {
            return false;
        }
        true
    }
}
