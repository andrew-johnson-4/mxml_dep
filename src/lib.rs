#[derive(Debug,PartialEq,Eq)]
pub struct XML {
   pub children: Vec<XMLChild>
}

#[derive(Debug,PartialEq,Eq)]
pub enum XMLChild {
   Tag(XMLTag),
   String(String)
}

#[derive(Debug,PartialEq,Eq)]
pub struct XMLTag {
   pub tag: String,
   pub attributes: Vec<(String,String)>,
   pub children: Vec<XMLChild>,
   pub self_closing: bool
}

impl XML {
   pub fn parse(_es: &str) -> XML {
     XML {
       children: Vec::new()
     }
   }
}
