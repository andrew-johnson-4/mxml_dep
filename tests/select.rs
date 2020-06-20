use mxml_dep::*;

#[test]
fn parse1() {
   let mut xml = parse("<p>a<p>b<p>c</p></p></p>").unwrap();
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
   fme(&mut xml, &m);
   assert_eq!(
      unparse(&xml),
      r#"<p a>a<p b>b<p>c</p></p></p>"#
   );
}

#[test]
fn parse2() {
   let mut xml = parse("<p>a<p a>b<p>c</p></p></p>").unwrap();
   let m = FindMatchEditElement {
      fme: vec![(
         FindElement{find:vec![]},
         MatchElement{when:vec![Match::HasAttributeValue("a".to_string(),"".to_string())]},
         EditElement{edit:vec![]},
      ),(
         FindElement{find:vec![]},
         MatchElement{when:vec![Match::HasTag("p".to_string())]},
         EditElement{edit:vec![Edit::AddAttribute("b".to_string(),"".to_string())]}
      )]
   };
   fme(&mut xml, &m);
   assert_eq!(
      unparse(&xml),
      r#"<p>a<p a>b<p b>c</p></p></p>"#
   );
}
