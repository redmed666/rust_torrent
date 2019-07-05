use super::protocol::Protocol;

#[derive(Debug)]
pub struct Tracker {
    protocol: Protocol,
    domain: String,
    port: u16,
}

impl std::fmt::Display for Tracker {
    /**
     * Basic implementation of the Display trait
     *
     * @return usable Display thing
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "protocol: {}, domain: {}, port: {}",
            self.protocol, self.domain, self.port
        )
    }
}

impl Tracker {
    pub fn get_protocol(&self) -> &Protocol {
        &self.protocol
    }

    pub fn get_domain(&self) -> &str {
        &self.domain
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    /**
     * Tracker generator
     *
     * @param address (string) the tracker address
     *                          structure is <protocol>(://)(%3A%2F%2F)<domain>(:)(%3A)<port>
     * @return Tracker that can be deduced from the address given
     */
    pub fn from_string(address: &str) -> Result<Tracker, String> {
        let regex = regex::Regex::new(
            r"(?i)(?P<protocol>[a-z]+)(://|%3A%2F%2F)(?P<domain>.+(\..+)+)(:|%3A)(?P<port>[0-9]+)",
        )
        .unwrap();
        
        if let Some(tracker_capture) = regex.captures(address) {
            let protocol_str = tracker_capture.name("protocol").unwrap().as_str();

            let protocol: Protocol;
            if let Ok(p) = Protocol::from_string(protocol_str) {
                protocol = p;
            } else {
                return Err(format!(
                    "The given protocol in the tracker is not supported ({})",
                    protocol_str
                ));
            }

            Ok(Tracker {
                protocol,
                domain: String::from(tracker_capture.name("domain").unwrap().as_str()),
                port: tracker_capture
                    .name("port")
                    .unwrap()
                    .as_str()
                    .parse::<u16>()
                    .unwrap(),
            })
        } else {
            Err(format!(
                "The given tr is not a valid Tracker !\n({})",
                address
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tracker;

    #[test]
    #[should_panic(expected = "The given tr is not a valid Tracker")]
    fn tracker_syntax_ko() {
        let tracker_str = "coucouloucoucou:paloma//test.failed.com";
        let _result = Tracker::from_string(tracker_str).unwrap();
    }

    #[test]
    #[should_panic(expected = "The given protocol in the tracker is not supported")]
    fn tracker_wrong_protocol() {
        let tracker_str = "coucouloucoucoupaloma://my-domain.com:69";
        let _result = Tracker::from_string(tracker_str).unwrap();
    }

    #[test]
    fn from_string_tracker() {
        let tracker_str = "tcp://my-domain.com:69";
        let result = Tracker::from_string(tracker_str).unwrap();

        assert_eq!(
            result.get_protocol(),
            &super::super::protocol::Protocol::TCP
        );
        assert_eq!(result.get_domain(), "my-domain.com");
        assert_eq!(result.get_port(), 69);
    }
}
