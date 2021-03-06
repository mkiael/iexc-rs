use std::error;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Response {
    pub protocol_version: String,
    pub status_code: u16,
    pub status_message: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Response {
    pub fn parse<R: BufRead>(reader: &mut R) -> Result<Response> {
        let mut status_line = String::new();
        reader.read_line(&mut status_line)?;

        let (protocol_version, status_code, status_message) = parse_status_line(&status_line)?;

        let mut headers = Vec::new();
        loop {
            let mut header = String::new();
            let bytes_read = reader.read_line(&mut header)?;
            if bytes_read == 2 {
                break;
            }
            headers.push(parse_header(&header)?);
        }

        let content_length = find_content_length(&headers);
        let mut body = String::new();
        reader.take(content_length).read_to_string(&mut body)?;

        Ok(Response {
            protocol_version,
            status_code,
            status_message,
            headers,
            body,
        })
    }
}

fn parse_status_line(status_line: &str) -> Result<(String, u16, String)> {
    if let Some((version, rest1)) = status_line.split_once(" ") {
        if let Some((status_code, rest2)) = rest1.split_once(" ") {
            Ok((
                version.trim().to_string(),
                status_code.trim().parse::<u16>().unwrap(),
                rest2.trim().to_string(),
            ))
        } else {
            Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unable to parse status code",
            )))
        }
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unable to parse protocol version",
        )))
    }
}

fn parse_header(header_line: &str) -> Result<(String, String)> {
    match header_line.split_once(":") {
        Some((key, value)) => Ok((
            key.to_ascii_lowercase(),
            value.trim().to_ascii_lowercase(),
        )),
        _ => Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unable to parse header line",
        ))),
    }
}

fn find_content_length(headers: &[(String, String)]) -> u64 {
    match headers.iter().find(|t| t.0.eq("content-length")) {
        Some(t) => t.1.parse::<u64>().unwrap(),
        _ => 0,
    }
}

trait Stream: io::Read + io::Write {}
impl<T: io::Read + io::Write> Stream for T {}

pub struct Client {
    domain: String,
    port: i16,
    insecure: bool,
}

impl Client {
    pub fn new(domain: String) -> Self {
        Self {
            domain,
            port: 443,
            insecure: false,
        }
    }

    pub fn _new_insecure(domain: String) -> Self {
        Self {
            domain,
            port: 80,
            insecure: true,
        }
    }

    pub fn get(&self, path: &str) -> Result<Response> {
        let mut stream = self.create_stream()?;

        write!(
            stream,
            "GET {} HTTP/1.1\r\n\
            Host: {}\r\n\
            Accept: */*\r\n\
            \r\n",
            path, self.domain
        )?;

        Response::parse(&mut io::BufReader::new(stream))
    }

    fn create_stream(&self) -> Result<Box<dyn Stream>> {
        if self.insecure {
            match TcpStream::connect(format!("{}:{}", self.domain, self.port)) {
                Ok(stream) => Ok(Box::new(stream)),
                Err(e) => Err(Box::new(e)),
            }
        } else {
            let mut config = rustls::ClientConfig::new();
            config
                .root_store
                .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
            let dns_name = webpki::DNSNameRef::try_from_ascii_str(self.domain.as_str())?;
            let session = rustls::ClientSession::new(&Arc::new(config), dns_name);
            let socket = TcpStream::connect(format!("{}:{}", self.domain, self.port))?;
            Ok(Box::new(rustls::StreamOwned::new(session, socket)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http::Response;

    #[test]
    fn test_parse_valid_response() {
        let mut response = b"HTTP/1.1 200 OK\r\n\
                             Date: Sun, 10 Oct 2010 23:26:07 GMT\r\n\
                             Server: Apache/2.2.8 (Ubuntu) mod_ssl/2.2.8 OpenSSL/0.9.8g\r\n\
                             Last-Modified: Sun, 26 Sep 2010 22:04:35 GMT\r\n\
                             ETag: \"45b6-834-49130cc1182c0\"\r\n\
                             Accept-Ranges: bytes\r\n\
                             Content-Length: 12\r\n\
                             Connection: close\r\n\
                             Content-Type: text/html\r\n\
                             \r\n\
                             Hello world!" as &[u8];

        let resp = Response::parse(&mut response).unwrap();

        assert_eq!(resp.protocol_version, "HTTP/1.1");
        assert_eq!(resp.status_code, 200);
        assert_eq!(resp.status_message, "OK");
        assert_eq!(resp.body, "Hello world!");
    }

    #[test]
    fn test_parse_long_status_line() {
        let mut response = b"HTTP/1.1 301 Moved to some nice place\n\r\
                             Date: Sun, 10 Oct 2010 23:26:07 GMT\r\n\
                             Server: Apache/2.2.8 (Ubuntu) mod_ssl/2.2.8 OpenSSL/0.9.8g\r\n\
                             \r\n" as &[u8];

        let resp = Response::parse(&mut response).unwrap();

        assert_eq!(resp.protocol_version, "HTTP/1.1");
        assert_eq!(resp.status_code, 301);
        assert_eq!(resp.status_message, "Moved to some nice place");
    }

    #[test]
    fn test_parse_invalid_status() {
        let mut response = b"OK\n\r\
                             Date: Sun, 10 Oct 2010 23:26:07 GMT\r\n\
                             Server: Apache/2.2.8 (Ubuntu) mod_ssl/2.2.8 OpenSSL/0.9.8g\r\n\
                             \r\n" as &[u8];

        match Response::parse(&mut response) {
            Ok(_) => assert!(false),
            Err(_) => (),
        }
    }
}
