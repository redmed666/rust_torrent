extern crate regex;
extern crate url;

mod exact_topic;
mod hash_tech;
mod protocol;
mod tracker;

use exact_topic::ExactTopic;
use regex::Regex;
use tracker::Tracker;

/**
 * @overview contains information that could be stored in a magnet link
 *           experimental fields are not managed
 *
 * @see https://en.wikipedia.org/wiki/Magnet_URI_scheme
 */
pub struct Magnet {
    link: String,
    xts: Vec<ExactTopic>, // eXact Topic: URN containing hash
    dn: String,           // Display Name: filename shown to user
    xl: u128,             // eXact Length: filesize
    acs: String,          // Acceptable Source: web link to the file online
    xs: String,           // eXact Source: P2P link identified by a content-hash
    kts: Vec<String>,     // Keyword Topic: key words for search
    trs: Vec<Tracker>,    // address TRacker: tracker URL for BitTorrent downloads
    mt: String, // Manifest Topic:  link to the metafile that contains a list of magneto (see MAGMA)
}

impl Magnet {
    /**
     * Constructor
     *
     * @return new empty Magnet
     */
    pub fn new() -> Magnet {
        Magnet {
            link: String::new(),
            xts: Vec::new(),
            dn: String::new(),
            xl: 0,
            acs: String::new(),
            xs: String::new(),
            kts: Vec::new(),
            trs: Vec::new(),
            mt: String::new(),
        }
    }

    pub fn get_xt(&self) -> &Vec<ExactTopic> {
        &self.xts
    }

    pub fn get_tr(&self) -> &Vec<Tracker> {
        &self.trs
    }

    pub fn get_dn(&self) -> &str {
        &self.dn
    }

    pub fn get_xl(&self) -> u128 {
        self.xl
    }

    pub fn get_kt(&self) -> &Vec<String> {
        &self.kts
    }

    pub fn get_header(&self) -> String {
        if let Some(_i) = self.link.find("magnet:?") {
            String::from("magnet:?")
        } else {
            String::new()
        }
    }

    pub fn from_string(magnet_link: String) -> Result<Magnet, String> {
        // XT
        let mut xts: Vec<ExactTopic> = Vec::new();
        let re_xt = Regex::new(r"(?P<xt_struct>xt=(?P<xt>urn:[a-z0-9]+:[a-z0-9]+)(&|$))").unwrap();

        // foreach capture, get the group named "xt" and create an ExactTopic from it
        for xt_capture in re_xt.captures_iter(&magnet_link) {
            let xt_string = String::from(xt_capture.name("xt").unwrap().as_str());

            // manage the case for which the string given is not valid
            if let Ok(xt_tmp) = ExactTopic::from_string(&xt_string) {
                xts.push(xt_tmp);
            } else {
                println!(
                    "Your magnet is shit, you may want to check it around\n\"{}\"",
                    String::from(xt_capture.name("xt_struct").unwrap().as_str())
                );
                continue;
            }
        }

        // TR
        let mut trs: Vec<Tracker> = Vec::new();
        let re_tr = Regex::new(r"tr=(?P<tr>.+?)(&|$)").unwrap();

        for tr_capture in re_tr.captures_iter(&magnet_link) {
            if let Ok(tracker) = Tracker::from_string(tr_capture.name("tr").unwrap().as_str()) {
                trs.push(tracker);
            } else {
                println!("Error trying to add a tracker");
            }
        }

        // DN
        let dn: String;
        let re_dn = Regex::new(r"dn=(?P<dn>.+?)(&|$)").unwrap();
        // if the regex do match, we expect the group named "dn" to exist -> unwrap is ok
        if let Some(dn_capture) = re_dn.captures(&magnet_link) {
            dn = String::from(dn_capture.name("dn").unwrap().as_str());
        } else {
            dn = String::new();
        }

        // XL
        let xl: u128;
        let re_xl = Regex::new(r"xl=(?P<xl>.+?)(&|$)").unwrap();
        if let Some(xl_capture) = re_xl.captures(&magnet_link) {
            xl = xl_capture
                .name("xl")
                .unwrap()
                .as_str()
                .parse::<u128>()
                .unwrap();
        } else {
            xl = 0;
        }

        // KT
        let mut kts: Vec<String> = Vec::new();
        let re_kt = Regex::new(r"kt=(?P<kt>.+?)(&|$)").unwrap();
        for kt_capture in re_kt.captures_iter(&magnet_link) {
            let kt_str = String::from(kt_capture.name("kt").unwrap().as_str());
            kts.push(kt_str);
        }

        Ok(Magnet {
            link: magnet_link,
            xts,
            trs,
            dn,
            xl,
            acs: String::new(),
            xs: String::new(),
            kts,
            mt: String::new(),
        })
    }
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
