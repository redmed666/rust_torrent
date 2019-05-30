mod magnet;
/* real magnet link:
magnet:?
xt=urn:btih:4ddb6ed03f413ef34718111697573c839ed30eb9
&dn=Its+Always+Sunny+in+Philadelphia+Season+1%2C+2%2C+3%2C+4%2C+5+%26amp%3B+6+%2B+
&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969
&tr=udp%3A%2F%2Ftracker.openbittorrent.com%3A80
&tr=udp%3A%2F%2Fopen.demonii.com%3A1337
&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969
&tr=udp%3A%2F%2Fexodus.desync.com%3A6969
*/

fn test_error() -> Result<String, String> {
    Result::Err(String::from("test"))
}
fn main() {
    let test = String::from("test");
    let magnet_link = String::from("magnet:?xt=urn:btih:4ddb6ed03f413ef34718111697573c839ed30eb9&dn=Its+Always+Sunny+in+Philadelphia+Season+1%2C+2%2C+3%2C+4%2C+5+%26amp%3B+6+%2B+&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.openbittorrent.com%3A80&tr=udp%3A%2F%2Fopen.demonii.com%3A1337&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Fexodus.desync.com%3A6969");
    let mag = match magnet::magnet::from_string(magnet_link) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            magnet::magnet::Magnet::new()
        }
    };
    println!("{}", mag.header);
    for tr in mag.trs {
        println!("{}", tr);
    }
    println!("{}", mag.dn);
}
