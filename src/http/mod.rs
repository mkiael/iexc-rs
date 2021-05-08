use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::net::TcpStream;

pub struct Response {
    pub protocol_version: String,
    pub status_code: u16,
    pub status_message: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Response {
    pub fn parse<R: BufRead>(reader: &mut R) -> Result<Response, Error> {
        let mut status_line = String::new();
        reader.read_line(&mut status_line)?;
        let status_fields: Vec<&str> = status_line.trim_end().split_whitespace().collect();
        if status_fields.len() < 3 {
            return Err(Error::new(ErrorKind::InvalidData, ""));
        }
        let mut headers = Vec::new();
        loop {
            let mut header = String::new();
            let bytes_read = reader.read_line(&mut header)?;
            if bytes_read == 2 {
                break;
            }
            let header_fields: Vec<&str> = header.split(":").map(|s| s.trim()).collect();
            headers.push((header_fields[0].to_string(), header_fields[1].to_string()));
        }
        let content_length = Response::find_content_length(&headers);
        let mut body = String::new();
        reader.take(content_length).read_to_string(&mut body)?;

        Ok(Response {
            protocol_version: status_fields[0].to_string(),
            status_code: status_fields[1].parse::<u16>().unwrap(),
            status_message: status_fields[2].to_string(),
            headers: headers,
            body: body,
        })
    }

    fn find_content_length(headers: &Vec<(String, String)>) -> u64 {
        match headers
            .iter()
            .find(|t| t.0.eq_ignore_ascii_case("content-length"))
        {
            Some(t) => t.1.parse::<u64>().unwrap(),
            _ => 0,
        }
    }
}

pub struct Client {
    domain: String,
    port: i16,
}

impl Client {
    pub fn new(domain: String) -> Self {
        Self { domain, port: 80 }
    }

    pub fn get(&self) -> Result<Response, Error> {
        let mut socket = TcpStream::connect(format!("{}:{}", self.domain, self.port))?;

        write!(
            socket,
            "GET / HTTP/1.1\r\n\
            Host: {}\r\n\
            Accept: */*\r\n\
            \r\n",
            self.domain
        )?;

        Response::parse(&mut BufReader::new(socket))
    }
}

#[cfg(test)]
mod tests {
    use crate::http::Response;

    #[test]
    fn test_parse_response() {
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
}
