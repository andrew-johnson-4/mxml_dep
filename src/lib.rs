
use html_parser::{Dom, Result};

pub fn parse(es: &str) -> Result<Dom> {
   Dom::parse(es)
}
