extern crate alloc;
use alloc::string::String;
use saba_core::error:Error;
use saba_core::http::HttpResponse;
use alloc::format;
use alloc::string::ToString;
use noli::net::lookup_host;
use noli::net::SocketAddr;
use noli::net::TcpStream;

pub struct HttpClient {}

impl HttpClient {
    // constructor
    // This structure doesn't have fields
    pub fn new() -> Self {
        Self {}
    }

    // sending GET request
    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        // lookup host returns list of IP addresses as vector 
        let ips = match lookup_host(&host) {
            // if lookup is successful, following code is executed
            Ok(ips) => ips,
            // if lookup fails, following error handling is performed
            Err(e) => {
                return Err(Error::Network(format!(
                    "Failed to find IP addresses: {:#?}",
                    e
                )))
            }
        };

        // not found any IP addresses
        if ips.len() < 1 {
            return Err(Error::Network("Failed to find IP addresses".to_string()));
        }

        // set IP address and port number to SocketAddr
        let socket_addr: SocketAddr = (ips[0], port).into();

        // stream is established here
        // TcpStream::connect tries to connect to the specified SocketAddr
        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to connect to TCP stream".to_string(),
                ))
            }
        };

        // HTTP consists of three parts: start line, headers, and body
        // start line is request line for HTTP request or status line for HTTP response

        // constructing HTTP GET request line
        // request line format: method SP request-target SP HTTP-version CRLF
        // "GET /path HTTP/1.1"
        let mut request = String::from("GET /");
        request.push_str(&path);
        request.push_str(" HTTP/1.1\n");

        // adding headers
        // header format: field-name ":" OWS field-value OWS
        // OWS is optional whitespace
        request.push_str("Host: ");
        request.push_str(&host);
        request.push('\n');
        request.push_str("Accept: text/html\n");
        request.push_str("Connection: close\n");
        request.push('\n');
    }
}