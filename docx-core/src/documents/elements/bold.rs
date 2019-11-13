use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Bold {}

impl Bold {
    pub fn new() -> Bold {
        Bold {}
    }
}

impl BuildXML for Bold {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.b().build()
    }
}
