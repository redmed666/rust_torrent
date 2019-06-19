/**
 * Some hash technologies often used in ExactTopic
 */
#[derive(Debug, PartialEq)]
pub enum HashTech {
    TTH,      // tree (Tiger Tree Hash)
    SHA1,     // sha1
    BitPrint, // bitprint
    ED2K,     // ed2k (eDonkey2000)
    AICH,     // aich (Advanced Intelligent Corruption Handler)
    Kazaa,    // kzhash
    BTIH,     // btih (BitTorrent info hash)
    MD5,      // md5
}

impl std::fmt::Display for HashTech {
    /**
     * Simplest way to implement the Display trait
     * using Debug output
     *
     * @return usable Display thing
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl HashTech {
    /**
     * HashTech generator from String
     *
     * @param name : the name of the hash tech used
     *
     * @return corresponding HashTech
     */
    pub fn from_string(name: &str) -> Result<HashTech, String> {
        match name {
            "tree" => Ok(HashTech::TTH),
            "sha1" => Ok(HashTech::SHA1),
            "bitprint" => Ok(HashTech::BitPrint),
            "ed2k" => Ok(HashTech::ED2K),
            "aich" => Ok(HashTech::AICH),
            "kzhash" => Ok(HashTech::Kazaa),
            "btih" => Ok(HashTech::BTIH),
            "md5" => Ok(HashTech::MD5),
            _ => Err(format!("This hash technology is not managed ({})", name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HashTech;

    #[test]
    fn tree_ok() {
        let result = HashTech::from_string("tree").unwrap();
        assert_eq!(HashTech::TTH, result);
    }

    #[test]
    fn sha1_ok() {
        let result = HashTech::from_string("sha1").unwrap();
        assert_eq!(HashTech::SHA1, result);
    }

    #[test]
    fn bitprint_ok() {
        let result = HashTech::from_string("bitprint").unwrap();
        assert_eq!(HashTech::BitPrint, result);
    }

    #[test]
    fn ed2k_ok() {
        let result = HashTech::from_string("ed2k").unwrap();
        assert_eq!(HashTech::ED2K, result);
    }

    #[test]
    fn aich_ok() {
        let result = HashTech::from_string("aich").unwrap();
        assert_eq!(HashTech::AICH, result);
    }

    #[test]
    fn kzhash_ok() {
        let result = HashTech::from_string("kzhash").unwrap();
        assert_eq!(HashTech::Kazaa, result);
    }

    #[test]
    fn btih_ok() {
        let result = HashTech::from_string("btih").unwrap();
        assert_eq!(HashTech::BTIH, result);
    }

    #[test]
    fn md5_ok() {
        let result = HashTech::from_string("md5").unwrap();
        assert_eq!(HashTech::MD5, result);
    }

    #[test]
    #[should_panic(expected = "This hash technology is not managed")]
    fn tech_ko() {
        let result = HashTech::from_string("bite").unwrap();
    }

}
