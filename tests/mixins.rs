use mxml_dep::*;

#[test]
fn mixin1() {
   let xml = "<p>a<p>b<p>c</p></p></p>";
   let m1 = FindMatchEditElement {
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
   let m2 = FindMatchEditElement {
      fme: vec![(
         FindElement{find:vec![]},
         MatchElement{when:vec![Match::HasAttributeValue("b".to_string(),"".to_string())]},
         EditElement{edit:vec![Edit::AddAttribute("c".to_string(),"".to_string())]}
      )]
   };
   let m = m1.then(m2);
   let xml = m.mixin(xml);
   assert_eq!(
      xml,
      r#"<p a>a<p b c>b<p>c</p></p></p>"#
   );
}

#[test]
fn mixin2() {
   let xml = "<p>a<p>b<p>c</p></p></p>";
   let m1 = FindMatchEditElement {
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
   let m2 = FindMatchEditElement {
      fme: vec![(
         FindElement{find:vec![]},
         MatchElement{when:vec![Match::HasAttributeValue("b".to_string(),"".to_string())]},
         EditElement{edit:vec![Edit::AddAttribute("c".to_string(),"".to_string())]}
      )]
   };
   let m3 = FindMatchEditElement {
      fme: vec![(
         FindElement{find:vec![]},
         MatchElement{when:vec![Match::HasAttributeValue("c".to_string(),"".to_string())]},
         EditElement{edit:vec![Edit::AddAttribute("d".to_string(),"".to_string())]}
      )]
   };
   let m = m1.then(m2).then(m3);
   let xml = m.mixin(xml);
   assert_eq!(
      xml,
      r#"<p a>a<p b c d>b<p>c</p></p></p>"#
   );
}
