use mxml_dep::*;

#[test]
fn parse1() {
   let xml = parse("<br/>").unwrap().to_json().unwrap();
   assert_eq!(
      xml,
      r#"{"treeType":"documentFragment","children":[{"name":"br","variant":"void"}]}"#
   );
}
