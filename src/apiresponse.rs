#[derive(serde::Serialize, serde::Deserialize, Debug)]

pub enum ResponseStatus {
    Success,
    Error,
    NotFound,
    BadRequest,
    Unauthorized,
    Forbidden,
    Conflict,
    InternalServerError,
    NotImplemented,
    ServiceUnavailable,
    GatewayTimeout,
    Unknown,
    Created,
}

impl ResponseStatus {
    pub fn code(&self) -> u16 {
        match self {
            ResponseStatus::Success => 200,
            ResponseStatus::Error => 500,
            ResponseStatus::NotFound => 404,
            ResponseStatus::BadRequest => 400,
            ResponseStatus::Unauthorized => 401,
            ResponseStatus::Forbidden => 403,
            ResponseStatus::Conflict => 409,
            ResponseStatus::InternalServerError => 500,
            ResponseStatus::NotImplemented => 501,
            ResponseStatus::ServiceUnavailable => 503,
            ResponseStatus::GatewayTimeout => 504,
            ResponseStatus::Unknown => 520,
            ResponseStatus::Created => 201,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub  struct ApiResponse<T> {
    pub status: ResponseStatus,
    pub message: String,
    pub data: Option<T>,
}


impl<T> ApiResponse<T> {
    pub fn new(status: ResponseStatus, message: &str, data: Option<T>) -> Self {
        Self {
            status,
            message : message.to_string(),
            data,
        }
    }

    pub fn success(data: T) -> Self {
        Self {
            status: ResponseStatus::Success,
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            status: ResponseStatus::Error,
            message,
            data: None,
        }
    }

    pub fn not_found(message: String) -> Self {
        Self {
            status: ResponseStatus::NotFound,
            message,
            data: None,
        }
    }

    pub fn bad_request(message: String) -> Self {
        Self {
            status: ResponseStatus::BadRequest,
            message,
            data: None,
        }
    }

    pub fn unauthorized(message: String) -> Self {
        Self {
            status: ResponseStatus::Unauthorized,
            message,
            data: None,
        }
    }

    pub fn forbidden(message: String) -> Self {
        Self {
            status: ResponseStatus::Forbidden,
            message,
            data: None,
        }
    }

    pub fn conflict(message: String) -> Self {
        Self {
            status: ResponseStatus::Conflict,
            message,
            data: None,
        }
    }

    pub fn internal_server_error(message: String) -> Self {
        Self {
            status: ResponseStatus::InternalServerError,
            message,
            data: None,
        }
    }

    pub fn not_implemented(message: String) -> Self {
        Self {
            status: ResponseStatus::NotImplemented,
            message,
            data: None,
        }
    }

    pub fn service_unavailable(message: String) -> Self {
        Self {
            status: ResponseStatus::ServiceUnavailable,
            message,
            data: None,
        }
    }

    pub fn gateway_timeout(message: String) -> Self {
        Self {
            status: ResponseStatus::GatewayTimeout,
            message,
            data: None,
        }
    }

    pub fn unknown(message: String) -> Self {
        Self {
            status: ResponseStatus::Unknown,
            message,
            data: None,
        }
    }
}