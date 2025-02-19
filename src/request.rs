use anyhow::Result;
use curl::easy::{Easy, List};

pub fn request(cf_who: &str) -> Result<String> {
    let mut easy = Easy::new();
    let url = "https://ntp.cloudflarenet.workers.dev/synctime2";
    let mut list = List::new();
    list.append(format!("cf-who: {}", cf_who).as_str())?;
    easy.url(url)?;
    easy.http_headers(list)?;
    let mut dst = Vec::new();
    let mut transfer = easy.transfer();

    transfer.write_function(|data| {
        // stdout().write_all(data).unwrap();
        dst.extend_from_slice(data);
        Ok(data.len())
    })?;

    transfer.perform().unwrap();
    drop(transfer);

    let txt = String::from_utf8_lossy(&dst).to_string();

    Ok(txt)
}
