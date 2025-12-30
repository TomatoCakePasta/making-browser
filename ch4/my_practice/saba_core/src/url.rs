use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

// define Url struct
#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    // constructor
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    // getter methods
    // in rust grammar, no need "return" keyword
    // just the last expression is returned
    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> String {
        self.port.clone()
    }   

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn searchpart(&self) -> String {
        self.searchpart.clone()
    }

    // url format
    // http://<host>:<port>/<path>?<searchpart>
    fn is_http(&self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        false
    }

    fn extract_host(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        // in rust grammar, no need "return" keyword
        // just the last expression is returned

        // url_parts is following 2 patterns
        // ["<host>:<port>", "<path>?<searchpart>"]
        // or 
        // ["<host>"]
        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }

    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        // in rust grammar, no need "return" keyword
        // just the last expression is returned

        // url_parts is following 2 patterns
        // ["<host>:<port>", "<path>?<searchpart>"]
        // or 
        // ["<host>"]
        if let Some(index) = url_parts[0].find(':') {
            // "<host>:<port>"
            // extract over ":", so it returns "<port>"
            url_parts[0][index + 1..].to_string()
        } else {
            // default port
            "80".to_string()
        }
    }

    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        // url_parts is following 2 patterns
        // ["<host>:<port>", "<path>?<searchpart>"]
        // or 
        // ["<host>"]
        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        // ["<path>", "<searchpart>"]
        // or 
        // ["<path>"]
        path_and_searchpart[0].to_string()
    }

    fn extract_searchpart(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        // url_parts is following 2 patterns
        // ["<host>:<port>", "<path>?<searchpart>"]
        // or 
        // ["<host>"]
        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_add_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        // ["<path>", "<searchpart>"]
        // or 
        // ["<path>"]
        if path_add_searchpart.len() < 2 {
            "".to_string()
        } else {
            path_add_searchpart[1].to_string()
        }
    }

    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported.".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        // Ok is enum variant of Result type
        Ok(self.clone())
    }
}


// following is unit test code
// generrally, unit test code is written in same file as implementation code
// cfg is configuration attribute
// attribute is metadata applied to modules, crates, or items
#[cfg(test)]
// generally, test code is written in tests module
mod tests {
    use super::*;

    // unit test function needs attribute "test"
    #[test]
    fn test_url_host() {
        let url = "http://example.com".to_string();
        // Ok is enum variant of Result type
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        
        // This test verifies that parsing "http://example.com" succeeds
        // and returns Ok(Url) with the expected host, port, path, and searchpart.

        // A function with "!"" at the end is macro
        // A function runs on values at runtime, while a macro runs on code at compile time and generates code.
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port() {
        let url = "http://example.com:8888".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port_path() {
        let url = "http://example.com:8888/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_path() {
        let url = "http://example.com/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port_path_searchquery() {
        let url = "http://example.com:8888/index.html?a=123&b=456".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    // Failure case test
    #[test]
    fn test_no_scheme() {
        let url = "example.com".to_string();
        let expected = Err("Only HTTP scheme is supported.".to_string());
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_unsupported_scheme() {
        let url = "https://example.com".to_string();
        let expected = Err("Only HTTP scheme is supported.".to_string());
        assert_eq!(expected, Url::new(url).parse());
    }
}
