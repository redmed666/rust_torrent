extern crate regex;
extern crate url;

mod exact_topic;
mod hash_tech;

use exact_topic::ExactTopic;

/**
 * @overview contains information that could be stored in a magnet link
 *           experimental fields are not managed
 *
 * @see https://en.wikipedia.org/wiki/Magnet_URI_scheme
 */
pub struct Magnet {
    link: String,
    header: String,       // should be magnet:?
    xts: Vec<ExactTopic>, // eXact Topic: URN containing hash
    dn: String,           // Display Name: filename shown to user
    acs: String,          // Acceptable Source: web link to the file online
    kt: Vec<String>,      // Keyword Topic: key words for search
    trs: Vec<String>,     // address TRacker: tracker URL for BitTorrent downloads
    xs: String,           // eXact Source: P2P link identified by a content-hash
    xl: u128,             // eXact Length: filesize
    mt: String, // Manifest Topic:  link to the metafile that contains a list of magneto (see MAGMA)
}

impl Magnet {
    pub fn new() -> Magnet {
        let res = Magnet {
            link: String::new(),
            header: String::new(),
            xts: Vec::new(),
            dn: String::new(),
            xl: 0,
            acs: String::new(),
            xs: String::new(),
            kt: Vec::new(),
            mt: String::new(),
            trs: Vec::new(),
        };
        res
    }

    fn find_header(&self) -> Option<usize> {
        self.link.find("magnet:?")
    }

    fn get_xt(&self) -> Option<Vec<String>> {
        let mut results = Vec::new();
        let re = regex::Regex::new(r"xt=(.+?)(&|$)").unwrap();
        let xts_found = re.find_iter(&(self.link));

        for xt_ in xts_found {
            let xt_ = xt_.as_str();
            results.push(String::from(xt_));
        }
        Some(results)
    }

    pub fn get_tr(&self) -> Option<Vec<String>> {
        let mut results = Vec::new();
        let re = regex::Regex::new(r"tr=(.+?)(&|$)").unwrap();
        let trs_found = re.find_iter(&(self.link));

        for tr_ in trs_found {
            let tr_ = tr_.as_str();
            let tr_ = url::percent_encoding::percent_decode(tr_.as_bytes())
                .decode_utf8()
                .unwrap();
            results.push(tr_.into_owned());
        }
        Some(results)
    }

    pub fn get_dn(&self) -> Option<String> {
        let mut result: String;
        let re = regex::Regex::new(r"dn=(.+?)(&|$)").unwrap();
        let dns_found = re.find_iter(&(self.link)).last().unwrap().as_str();
        let dns_found = url::percent_encoding::percent_decode(dns_found.as_bytes())
            .decode_utf8()
            .unwrap();
        result = dns_found.into_owned();
        Some(result)
    }

    pub fn get_header(&self) -> String {
        if let Some(_i) = self.link.find("magnet:?") {
            String::from("magnet:?")
        } else {
            String::from("")
        }
    }
}

fn parse_xt(xts_out: &mut Vec<ExactTopic>, xts: Vec<String>) {
    for xt_ in xts {
        let xt_tmp = ExactTopic::from_string(&xt_).unwrap();
        xts_out.push(xt_tmp);
    }
}

fn parse_tr(trs_out: &mut Vec<String>, trs: Vec<String>) {
    for tr in trs {
        let tr = tr.replace("&", "");
        let tr = tr.replace("tr=", "");
        trs_out.push(tr);
    }
}

fn parse_dn(dn_out: &mut String, dn: &mut String) {
    *dn_out = dn.replace("dn=", "").replace("&", "");
}

pub fn from_string(magnet_link: String) -> Result<Magnet, String> {
    let mut result = Magnet::new();
    result.link = magnet_link;
    let header_pos = match result.find_header() {
        Some(v) => v,
        None => 1,
    };

    if header_pos == 0 {
        result.header = String::from("magnet:?");
        println!("[*] Magnet header found");
        let xts_found = result.get_xt().unwrap();
        parse_xt(&mut (result.xts), xts_found);
        let trs_found = result.get_tr().unwrap();
        parse_tr(&mut (result.trs), trs_found);
        let mut dn_found = result.get_dn().unwrap();
        parse_dn(&mut (result.dn), &mut dn_found);
    } else {
        return Err(String::from("Magnet header not found"));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple_test() {
        assert_eq!(1, 2 - 1);
    }

    // test with string ok
    // test with ok xt
    // test with ok 2 xt
    // test with xt that have a unexisting hash tech
    // test with xt that only have 1 ":" in it

}
