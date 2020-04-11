use csv::ReaderBuilder;
use ftp::FtpStream;
use std::collections::BTreeMap;
use std::slice::Iter;

use super::exchanges::Exchange;

#[derive(Clone)]
pub struct Security<'a> {
   pub symbol: &'a str,
   pub name: &'a str,
   pub exchange: &'a Exchange,
   pub isETF: bool
}

fn load_nasdaq() -> Result<Vec<Security<'a>>, Box<dyn std::error::Error>> {
   let mut ftp_stream = FtpStream::connect("ftp.nasdaqtrader.com:21")?;
   ftp_stream.login("anonymous", "anonymous");
   let _ = ftp_stream.cwd("symboldirectory").unwrap();

   println!("Reading...");
   let ftp_reader = ftp_stream.get("nasdaqlisted.txt").expect("Failed to read file");
   let mut reader = ReaderBuilder::new().delimiter(b'|').from_reader(ftp_reader);
   for result in reader.records() {
      let record = result?;
      if record.get(3).unwrap() != "N" { continue; } // not legit - not interested
      if record.get(4).unwrap() != "N" { continue; } // bad financial status - not interested

      let symbol = record.get(0).unwrap();
      let security = Security {
         symbol: symbol,
         name: record.get(1).unwrap(),
         exchange: Exchange::NASDAQ,
         isETF: if record.get(6).unwrap() == "Y" { true } else { false }
      };

      if !symbol.starts_with("File Creation") { self.loaded.insert(symbol.to_string(), security); }
   }
}

#[derive(Clone)]
pub struct Securities<'a> {
   loaded: BTreeMap<&'a str, Security<'a>>
}
impl<'a> Securities<'a> {
   pub fn import(&mut self, exchange: exchanges::Exchange) -> Result<(), Box<dyn std::error::Error>> {
      // get the symbols on nasdaq
      

      println!("{}", self.loaded.len());

      Ok(())
   }
}

impl<'a, 'b> IntoIterator for &'a Securities<'b> {
   fn into_iter(self) -> Iter<'a, &'b Security> { self.loaded.Values() }
}