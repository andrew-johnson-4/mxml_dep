use mxml_dep::*;

#[test]
fn parse1() {
   let xml = XML::parse("<br/>");
   assert_eq!(
      xml,
      XML{ children:vec![
         XMLChild::Tag(XMLTag {
            tag: "br".to_string(),
            attributes: Vec::new(),
            children: Vec::new(),
            self_closing: false
         })
      ]}
   );
}
