use crate::{alloc::string::ToString, error::Error};
use alloc::{format, string::String, vec::Vec};

#[derive(Debug, Clone)]
pub struct HttpResponse {
    version: String,
    status_code: u32,
    reason: String,
    headers: Vec<Header>,
    body: String,
}

#[derive(Debug, Clone)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

impl HttpResponse {
    pub fn new(raw_response: String) -> Result<Self, Error> {
        let preproceed_response = raw_response.trim_start().replace("\n\r", "\n");

        let (status_line, remaining) = match preproceed_response.split_once("\n") {
            Some((s, r)) => (s, r),
            None => {
                return Err(Error::Network(format!(
                    "invalid http response: {}",
                    preproceed_response,
                )))
            }
        };

        let (headers, body) = match remaining.split_once("\n\n") {
            Some((h, b)) => {
                let mut headers = Vec::new();
                for header in h.split("\n") {
                    let splitted_header: Vec<&str> = header.splitn(2, ":").collect();
                    headers.push(Header::new(
                        String::from(splitted_header[0].trim()),
                        String::from(splitted_header[1].trim()),
                    ));
                }
                (headers, b)
            }
            None => (Vec::new(), remaining),
        };

        let statuses: Vec<&str> = status_line.split(' ').collect();

        Ok(Self {
            version: statuses[0].to_string(),
            status_code: statuses[1].parse().unwrap_or(404),
            reason: statuses[2].to_string(),
            headers,
            body: body.to_string(),
        })
    }

    pub fn version(&self) -> String {
        self.version.clone()
    }

    pub fn status_code(&self) -> u32 {
        self.status_code
    }

    pub fn reason(&self) -> String {
        self.reason.clone()
    }

    pub fn headers(&self) -> Vec<Header> {
        self.headers.clone()
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn header_value(&self, name: &str) -> Result<String, String> {
        for h in &self.headers {
            if h.name == name {
                return Ok(h.value.clone());
            }
        }

        Err(format!("failed to find {} in headers", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_status_line_only() {
        let raw = "HTTP/1.1 200 OK\n\n".to_string();
        let res = HttpResponse::new(raw).expect("failed to parse http response");
        assert_eq!(res.version(), "HTTP/1.1");
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn test_one_header() {
        let raw = "HTTP/1.1 200 OK\nDate:xx xx xx\n\n".to_string();
        let res = HttpResponse::new(raw).expect("failed to parse http response");
        assert_eq!(res.version(), "HTTP/1.1");
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.reason(), "OK");

        assert_eq!(res.header_value("Date"), Ok("xx xx xx".to_string()));
    }

    #[test]
    fn test_two_headers_with_white_space() {
        let raw = "HTTP/1.1 200 OK\nDate:xx xx xx\nContent-Length: 42\n\n".to_string();
        let res = HttpResponse::new(raw).expect("failed to parse http response");
        assert_eq!(res.version(), "HTTP/1.1");
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.reason(), "OK");

        assert_eq!(res.header_value("Date"), Ok("xx xx xx".to_string()));
        assert_eq!(res.header_value("Content-Length"), Ok("42".to_string()));
    }

    #[test]
    fn test_body() {
        let raw = "HTTP/1.1 200 OK\nDate: xx xx xx\n\nbody message".to_string();
        let res = HttpResponse::new(raw).expect("failed to parse http response");
        assert_eq!(res.version(), "HTTP/1.1");
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.reason(), "OK");

        assert_eq!(res.header_value("Date"), Ok("xx xx xx".to_string()));
        assert_eq!(res.body(), "body message".to_string());
    }

    #[test]
    fn test_invalid() {
        let raw = "HTTP/1.1 200 OK".to_string();
        // 改行文字がない文字列は不正
        assert!(HttpResponse::new(raw).is_err());
    }
}
