use std::fmt;

use crate::canframe::{CANRequest, CANResponse};

#[derive(Clone, PartialEq, Debug)]
pub struct ErrorResponse {
    pub request: CANRequest,
    pub err: ODriveError
}

#[derive(Clone, Debug, PartialEq)]
pub enum ODriveError {
    FailedToSend,
    ImproperCast(String),
    NoResponse,
}

pub type ODriveResponse = Result<ResponseType, ErrorResponse>;

#[derive(Clone, Debug, PartialEq)]
pub enum ResponseType {
    Body {request: CANRequest, response: CANResponse},
    Bodyless{ req: CANRequest},
}

impl ResponseType {
    pub fn body(self) -> (CANRequest, CANResponse) {
        match self {
            ResponseType::Body{ request: req, response: resp} => (req, resp),
            ResponseType::Bodyless { req: _} => {
                panic!("Write requests do not return a response body")
            }
        }
    }

    pub fn request(self) -> CANRequest {
        match self {
            ResponseType::Body { request: req, response: _resp} => req,
            ResponseType::Bodyless { req} => req,
        }
    }
}

pub trait ResultAll<T, E> {
    fn unwrap_all(self) -> Vec<T>;
    fn expect_all(self, msg: String) -> Vec<T>;
}

impl<T, E> ResultAll<T, E> for Vec<Result<T, E>> where E: fmt::Debug{
    /// This method calls `.unwrap()` on all responses.
    /// This will panic if a single response is an error
    fn unwrap_all(self) -> Vec<T> {
        let mut frames = Vec::new();

        for response in self.into_iter() {
            frames.push(response.unwrap());
        }
        frames
    }


    fn expect_all(self, msg: String) -> Vec<T> {
        let mut frames = Vec::new();

        for response in self.into_iter() {
            frames.push(response.expect(&msg));
        }
        frames
    }
}
