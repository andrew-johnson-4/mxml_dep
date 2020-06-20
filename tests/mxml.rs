use mxml_dep::*;

#[test]
fn mxml1() {
   let mut xml = parse(r#"<button type="button" class="btn btn-secondary">tooltip will be mixed in</button>"#).unwrap();
   let m = FindMatchEditElement {
      fme: vec![(
         FindElement{find:vec![Find::ChildElement]},
         MatchElement{when:vec![]},
         EditElement{edit:vec![
            Edit::AddAttribute("data-toggle".to_string(),"tooltip".to_string()),
            Edit::AddAttribute("data-placement".to_string(),"top".to_string()),
            Edit::AddAttribute("message".to_string(),"hey!".to_string()),
         ]}
      )]
   };
   fme(&mut xml, &m);
   assert_eq!(
      unparse(&xml),
      r#"<button class="btn btn-secondary" data-placement="top" data-toggle="tooltip" message="hey!" type="button">tooltip will be mixed in</button>"#
   );
}
