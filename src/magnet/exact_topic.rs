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
     *
     */
    pub fn from_string(xt: &str) -> Result<ExactTopic, String> {
        let xt_splitted = xt.split(":").collect::<Vec<&str>>();
        Ok(ExactTopic {
            urn: HashTech::from_string(xt_splitted[1]).unwrap(),
            hash: String::from(xt_splitted[2]),
        })
    }
}
