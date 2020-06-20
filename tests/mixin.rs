use mxml_dep::*;

#[test]
fn mixin1() {
   let xml = "<p>a<p>b<p>c</p></p></p>";
   let m = FindMatchEditElement {
      fme: vec![(
         FindElement{find:vec![]},
         MatchElement{when:vec![Match::HasTag("p".to_string())]},
         EditElement{edit:vec![Edit::AddAttribute("a".to_string(),"".to_string())]}
      ),(
         FindElement{find:vec![]},
         MatchElement{when:vec![Match::HasTag("p".to_string())]},
         EditElement{edit:vec![Edit::AddAttribute("b".to_string(),"".to_string())]}
      )]
   };
   let xml = m.mixin(xml);
   assert_eq!(
      xml,
      r#"<p a>a<p b>b<p>c</p></p></p>"#
   );
}
