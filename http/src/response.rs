use std::collections::HashMap;
use std::io::{Result, Write};

// TODO: do we really need a `Clone`?
#[derive(Debug, PartialEq)]
pub struct Response<'res> {
    version: &'res str,
    status_code: u16,
    status_text: &'res str,
    headers: Option<HashMap<&'res str, &'res str>>,
    body: Option<String>,
}

impl<'res> Default for Response<'res> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: 200,
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'res> Response<'res> {
    #[must_use]
    pub fn new(
        status_code: u16,
        headers: Option<HashMap<&'res str, &'res str>>,
        body: Option<String>,
    ) -> Self {
        let mut response = Self::default();
        if status_code != 200 {
            response.status_code = status_code;
        };
        response.headers = headers.or_else(|| {
            let mut h = HashMap::new();
            h.insert("Content-Type", "text/html");
            Some(h)
        });
        response.status_text = match response.status_code {
            200 => "OK",
            400 => "Bad Request",
            500 => "Internal Server Error",
            _ => "Not Found",
        };
        response.body = body;
        response
    }

    pub fn send_response(self, stream: &mut impl Write) -> Result<()> {
        let res: String = self.into();
        stream.write_all(res.as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> u16 {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut header_string = String::new();
        for (k, v) in map.iter() {
            header_string = format!("{header_string}{k}:{v}\r\n");
        }
        header_string
    }

    fn body(&self) -> &str {
        self.body.as_ref().map_or("", |b| b.as_str())
    }
}

impl<'res> From<Response<'res>> for String {
    fn from(res: Response) -> Self {
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            res.version(),
            res.status_code(),
            res.status_text(),
            res.headers(),
            res.body().len(),
            res.body(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = Response::new(200, None, Some("body".to_owned()));
        let response_expected = Response {
            version: "HTTP/1.1",
            status_code: 200,
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("body".to_owned()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = Response::new(404, None, Some("body".into()));
        let response_expected = Response {
            version: "HTTP/1.1",
            status_code: 404,
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("body".to_owned()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = Response {
            version: "HTTP/1.1",
            status_code: 404,
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("body".to_owned()),
        };
        let response_expected: String = response_expected.into();
        let response_actual =
            "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 4\r\n\r\nbody";
        assert_eq!(response_expected, response_actual);
    }
}
