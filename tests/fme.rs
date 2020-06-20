use mxml_dep::*;

#[test]
fn fme1() {
   let mut xml = parse("<div>a</div><p>b</p>").unwrap();
   let m = FindMatchEditElement {
      fme: vec![(
         FindElement{find:vec![Find::ChildElement]},
         MatchElement{when:vec![Match::HasTag("div".to_string())]},
         EditElement{edit:vec![Edit::AddClass("d".to_string())]}
      )]
   };
   fme(&mut xml, &m);
   assert_eq!(
      unparse(&xml),
      r#"<div class="d">a</div><p>b</p>"#
   );
}
