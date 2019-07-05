use super::hash_tech::HashTech;

/**
 * Tuple created by the technology used for the hash and the content hash itself
 */
#[derive(Debug)]
pub struct ExactTopic {
    urn: HashTech,
    hash: String,
}

impl std::fmt::Display for ExactTopic {
    /**
     * Basic implementation of the Display trait
     *
     * @return usable Display thing
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "urn: {} - hash: {}", self.urn, self.hash)
    }
}

impl ExactTopic {
    /**
     * ExactTopic generator that is created from a string
     * The string must be like urn:<hash-tech>:<hash>
     */
    pub fn from_string(xt: &str) -> Result<ExactTopic, String> {
        // separate the string in 3
        let xt_splitted = xt.split(":").collect::<Vec<&str>>();

        // sanity check : length = 3
        if xt_splitted.len() != 3 {
            return Err(String::from(
                "The given string must have three parts separated by ':'",
            ));
        }

        // sanity check : first element = "urn"
        if xt_splitted[0] != "urn" {
            return Err(String::from("The given string must begin by 'urn'"));
        }

        // if everything went fine, return newly created ExactTopic
        Ok(ExactTopic {
            urn: HashTech::from_string(xt_splitted[1]).unwrap(),
            hash: String::from(xt_splitted[2]),
        })
    }

    /**
     * Getter (urn)
     *
     * @return a reference to the hashTech (urn) of the current ExactTopic
     */
    pub fn get_urn(&self) -> &HashTech {
        &self.urn
    }

    /**
     * Getter (hash)
     *
     * @return a reference to the string (hash) of the current ExactTopic
     */
    pub fn get_hash(&self) -> &str {
        &self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::ExactTopic;

    #[test]
    #[should_panic(expected = "The given string must have three parts separated by")]
    fn generate_xt_too_many_colon() {
        let urn = "urn:sha1:lkqsjdfmlkj:bite";
        let _xt = ExactTopic::from_string(urn).unwrap();
    }

    #[test]
    #[should_panic(expected = "The given string must have three parts separated by")]
    fn generate_xt_too_few_colon() {
        let urn = "urn:sha1";
        let _xt = ExactTopic::from_string(urn).unwrap();
    }

    #[test]
    #[should_panic(expected = "The given string must begin by")]
    fn generate_xt_no_urn_at_start() {
        let urn = "bite:sha1:mlqskjfdlkqsj";
        let _xt = ExactTopic::from_string(urn).unwrap();
    }

    #[test]
    #[should_panic(expected = "This hash technology is not managed")]
    fn generate_xt_not_a_valid_hash_tech() {
        let urn = "urn:bite:qslmkfjmsqlkjfd";
        let _xt = ExactTopic::from_string(urn).unwrap();
    }

    #[test]
    fn generate_xt_ok() {
        let urn = "urn:sha1:mqlskjfmlkqfsjd";
        let xt = ExactTopic::from_string(urn).unwrap();
        assert_eq!(xt.get_urn(), &super::HashTech::SHA1);
        assert_eq!(xt.get_hash(), "mqlskjfmlkqfsjd");
    }
}
