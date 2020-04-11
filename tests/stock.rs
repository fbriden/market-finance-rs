use market_finance::stock::{ Exchange, Securities };

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let symbols = Securities;
   symbols.load()?;

   Ok(())
}