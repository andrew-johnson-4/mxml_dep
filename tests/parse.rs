use mxml_dep::*;

#[test]
fn parse1() {
   let xml = parse("<br/>").unwrap().to_json().unwrap();
   assert_eq!(
      xml,
      r#"{"treeType":"documentFragment","children":[{"name":"br","variant":"void"}]}"#
   );
}

#[test]
fn parse2() {
   let xml = parse("<br/>").unwrap();
   assert_eq!(
      unparse(&xml),
      "<br/>"
   );
}

#[test]
fn parse3() {
   let xml = parse("<div>dave david</div>").unwrap();
   assert_eq!(
      unparse(&xml),
      "<div>dave david</div>"
   );
}

#[test]
fn parse4() {
   let xml = parse(r#"<div id="a"><b class="b">dave</b></div>"#).unwrap();
   assert_eq!(
      unparse(&xml),
      r#"<div id="a"><b class="b">dave</b></div>"#
   );
}
