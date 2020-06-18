
pub enum XMLChild {
   Tag(XMLTag),
   String(String)
}

pub struct XMLTag {
   tag: String,
   attributes: Vec<(String,String)>,
   children: Vec<XMLChild>,
   self_closing: bool
}
