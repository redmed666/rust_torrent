#[derive(Debug, PartialEq)]
pub enum Protocol {
    HTTP,
    UDP,
    FTP,
    TCP,
}

impl std::fmt::Display for Protocol {
    /**
     * Basic implementation of the Display trait
     *
     * @return usable Display thing
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Protocol {
    /**
     * Protocol generator, create the corresponding Protocol from the given string
     *
     * @param name (string) the protocol's name (lower case is mandatory)
     *
     * @return corresponding Protocol
     */
    pub fn from_string(name: &str) -> Result<Protocol, String> {
        match name {
            "http" => Ok(Protocol::HTTP),
            "udp" => Ok(Protocol::UDP),
            "ftp" => Ok(Protocol::FTP),
            "tcp" => Ok(Protocol::TCP),
            _ => Err(format!("This protocol is not managed ({})", name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Protocol;

    #[test]
    fn http_ok() {
        let result = Protocol::from_string("http").unwrap();
        assert_eq!(Protocol::HTTP, result);
    }

    #[test]
    fn udp_ok() {
        let result = Protocol::from_string("udp").unwrap();
        assert_eq!(Protocol::UDP, result);
    }

    #[test]
    fn ftp_ok() {
        let result = Protocol::from_string("ftp").unwrap();
        assert_eq!(Protocol::FTP, result);
    }

    #[test]
    fn tcp_ok() {
        let result = Protocol::from_string("tcp").unwrap();
        assert_eq!(Protocol::TCP, result);
    }

    #[test]
    #[should_panic(expected = "This protocol is not managed")]
    fn protocol_ko() {
        let _result = Protocol::from_string("bite").unwrap();
    }
}
