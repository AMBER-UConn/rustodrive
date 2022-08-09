use crate::{canframe::{CANRequest, CANResponse, ODriveCANFrame}, axis::AxisID};

#[derive(Clone, PartialEq, Debug)]
pub struct ErrorResponse {
    pub request: CANRequest,
    pub err: ODriveError
}

#[derive(Clone, Debug, PartialEq)]
pub enum ODriveError {
    FailedToSend,
    NoResponse,
    ConvertedBadData
}

pub type ODriveResponse = Result<ResponseType, ErrorResponse>;

#[derive(Debug)]
pub struct Success<T: TryFrom<ODriveCANFrame>> {
    pub axis: AxisID,
    pub sent_request: CANRequest,
    pub data: T
}


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