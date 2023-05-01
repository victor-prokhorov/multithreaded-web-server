use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl From<String> for Request {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path(String::new());
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = String::new();
        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(':') {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if !line.is_empty() {
                parsed_body += line;
            }
        }
        Self {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            body: parsed_body.to_owned(),
        }
    }
}

fn process_req_line(line: &str) -> (Method, Resource, Version) {
    let mut words = line.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_owned()),
        version.into(),
    )
}

fn process_header_line(line: &str) -> (String, String) {
    let mut header_items = line.split(':');
    let mut key = String::new();
    let mut value = String::new();
    if let Some(k) = header_items.next() {
        key = k.to_owned();
    }
    if let Some(v) = header_items.next() {
        value = v.to_owned();
    }
    (key, value)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Resource {
    Path(String),
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => Self::Uninitialized,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let req = String::from(
            "GET /orders HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n",
        );
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".to_owned(), " localhost".to_owned());
        headers_expected.insert("Accept".to_owned(), " */*".to_owned());
        headers_expected.insert("User-Agent".to_owned(), " curl/7.81.0".to_owned());
        let req: Request = req.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/orders".to_owned()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
