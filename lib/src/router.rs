use std::fmt;
use std::time::SystemTime;

use hyper::{self, Body, Method, Request, StatusCode};
use regex::{Captures, Regex};

use super::{context::Context, response::Response, result::Result, session::Session};

pub type Handler = Fn(&Context, &Session) -> Result<Response> + Sync + Send;

pub struct Route {
    method: Method,
    pattern: Regex,
    handler: Box<Handler>,
}

impl Route {
    pub fn new(m: Method, p: &str, h: Box<Handler>) -> Result<Self> {
        Ok(Self {
            method: m,
            pattern: Regex::new(p)?,
            handler: h,
        })
    }

    pub fn parse<'a>(&self, method: &'a Method, path: &'a str) -> Option<Captures<'a>> {
        if method == &self.method {
            if let Some(cap) = self.pattern.captures(path) {
                return Some(cap);
            }
        }
        None
    }

    pub fn walk<F>(&self, f: F) -> bool
    where
        F: Fn(&Method, &Regex) -> bool,
    {
        f(&self.method, &self.pattern)
    }
}

pub struct Router {
    routes: Vec<Route>,
    context: Context,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            context: Context::new(),
        }
    }
    pub fn get(&mut self, p: &str, h: Box<Handler>) -> Result<()> {
        self.add(Method::GET, p, h)
    }
    pub fn add(&mut self, m: Method, p: &str, h: Box<Handler>) -> Result<()> {
        self.routes.push(Route::new(m, p, h)?);
        Ok(())
    }

    pub fn handle(&self, req: Request<Body>) -> hyper::Response<Body> {
        let begin = SystemTime::now();
        let res = Response::to(self.walk(req));
        if let Ok(dur) = begin.elapsed() {
            info!("{} {:?}", res.status(), dur);
        };
        res
    }

    fn walk(&self, req: Request<Body>) -> Result<Response> {
        let method = req.method();
        let uri = req.uri();
        info!("{:?} {} {}", req.version(), method, uri);
        for it in self.routes.iter() {
            if let Some(_) = it.parse(method, uri.path()) {
                return (it.handler)(&self.context, &Session::new());
            }
        }

        Ok(Response::new(StatusCode::NOT_FOUND, None))
    }
}

impl fmt::Debug for Router {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("{:8} {}", "METHOD", "PATH");
        for it in self.routes.iter() {
            write!(f, "{:8} {}", it.method, it.pattern)?;
        }
        Ok(())
    }
}
