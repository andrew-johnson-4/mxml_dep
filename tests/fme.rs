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

#[test]
fn fme2() {
   let mut xml = parse(r#"<div>a</div><p id="b">b</p>"#).unwrap();
   let m = FindMatchEditElement {
      fme: vec![(
         FindElement{find:vec![Find::ChildElement]},
         MatchElement{when:vec![Match::HasId("b".to_string())]},
         EditElement{edit:vec![Edit::AddClass("d".to_string())]}
      )]
   };
   fme(&mut xml, &m);
   assert_eq!(
      unparse(&xml),
      r#"<div>a</div><p id="b" class="d">b</p>"#
   );
}

#[test]
fn fme3() {
   let mut xml = parse(r#"<div class="b">a</div><p>b</p>"#).unwrap();
   let m = FindMatchEditElement {
      fme: vec![(
         FindElement{find:vec![Find::ChildElement]},
         MatchElement{when:vec![Match::HasClass("b".to_string())]},
         EditElement{edit:vec![Edit::AddClass("d".to_string())]}
      )]
   };
   fme(&mut xml, &m);
   assert_eq!(
      unparse(&xml),
      r#"<div class="b d">a</div><p>b</p>"#
   );
}

#[test]
fn fme4() {
   let mut xml = parse(r#"<div b>a</div><p>b</p>"#).unwrap();
   let m = FindMatchEditElement {
      fme: vec![(
         FindElement{find:vec![Find::ChildElement]},
         MatchElement{when:vec![Match::HasAttributeValue("b".to_string(),"".to_string())]},
         EditElement{edit:vec![Edit::AddClass("d".to_string())]}
      )]
   };
   fme(&mut xml, &m);
   assert_eq!(
      unparse(&xml),
      r#"<div class="d" b>a</div><p>b</p>"#
   );
}

#[test]
fn fme5() {
   let mut xml = parse(r#"<div b="c">a</div><p>b</p>"#).unwrap();
   let m = FindMatchEditElement {
      fme: vec![(
         FindElement{find:vec![Find::ChildElement]},
         MatchElement{when:vec![Match::HasAttributeValue("b".to_string(),"c".to_string())]},
         EditElement{edit:vec![Edit::AddClass("d".to_string())]}
      )]
   };
   fme(&mut xml, &m);
   assert_eq!(
      unparse(&xml),
      r#"<div class="d" b="c">a</div><p>b</p>"#
   );
}

#[test]
fn fme6() {
   let mut xml = parse(r#"<div><p>b</p></div>"#).unwrap();
   let m = FindMatchEditElement {
      fme: vec![(
         FindElement{find:vec![Find::ChildElement]},
         MatchElement{when:vec![]},
         EditElement{edit:vec![]}
      ),(
         FindElement{find:vec![Find::ChildElement]},
         MatchElement{when:vec![]},
         EditElement{edit:vec![Edit::AddClass("d".to_string())]}
      )]
   };
   fme(&mut xml, &m);
   assert_eq!(
      unparse(&xml),
      r#"<div><p class="d">b</p></div>"#
   );
}
