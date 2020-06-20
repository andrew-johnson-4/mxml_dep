
use html_parser::{Dom, Node, Result, Element, ElementVariant};

#[derive(Clone)]
pub enum Find {
   ChildElement
}
impl Find {
   fn matches(&self, _e: &Element) -> bool {
      match self {
         Find::ChildElement => { true }
      }
   }
}

#[derive(Clone)]
pub enum Match {
   HasTag(String),
   HasId(String),
   HasClass(String),
   HasAttribute(String),
   HasAttributeValue(String,String)
}
impl Match {
   fn matches(&self, e: &Element) -> bool {
      match self {
         Match::HasTag(t) => { &e.name == t },
         Match::HasId(id) => { e.id == Some(id.to_string()) },
         Match::HasClass(c) => { e.classes.iter().any(|cc| c==cc) },
         Match::HasAttribute(k) => { e.attributes.contains_key(k) },
         Match::HasAttributeValue(k,v) => { e.attributes.get(k) == Some(&Some(v.to_string())) },
      }
   }
}

#[derive(Clone)]
pub enum Edit {
   AddId(String),
   AddClass(String),
   AddAttribute(String,String),
}
impl Edit {
   fn apply(&self, e: &mut Element) {
      match self {
         Edit::AddId(id) => { e.id = Some(id.to_string()); },
         Edit::AddClass(c) => { e.classes.push(c.to_string()); },
         Edit::AddAttribute(k,v) => {
            if v.len()==0 { e.attributes.insert(k.to_string(),None); }
            else { e.attributes.insert(k.to_string(),Some(v.to_string())); }
         },
      }
   }
}

#[derive(Clone)]
pub struct FindElement {
   pub find: Vec<Find>
}
#[derive(Clone)]
pub struct MatchElement {
   pub when: Vec<Match>
}
#[derive(Clone)]
pub struct EditElement {
   pub edit: Vec<Edit>
}

#[derive(Clone)]
pub struct FindMatchEditElement {
   pub fme: Vec<(FindElement,MatchElement,EditElement)>
}

pub fn fme(d: &mut Dom, fme: &FindMatchEditElement) {
   for mut n in d.children.iter_mut() {
      fme_node(&mut n, fme);
   }
}
pub fn fme_node(n: &mut Node, fme: &FindMatchEditElement) {
   match n {
      Node::Element(ref mut e) => {
         fme_element(e, fme)
      },
      _ => {}
   }
}
pub fn fme_element(el: &mut Element, fme: &FindMatchEditElement) {
   for (f,m,e) in fme.fme.iter() {
      if f.find.iter().all(|f| f.matches(el)) &&
         m.when.iter().all(|m| m.matches(el)) {
         for ed in e.edit.iter() {
            ed.apply(el)
         }
      }
   }
}

pub fn parse(es: &str) -> Result<Dom> {
   Dom::parse(es)
}

pub fn unparse(d: &Dom) -> String {
   let mut es = String::new();
   for c in d.children.iter() {
      es.push_str(&unparse_node(c));
   }
   es
}

pub fn unparse_node(n: &Node) -> String {
   match n {
      Node::Text(s) => { s.clone() },
      Node::Element(e) => {
         unparse_element(e)
      },
      Node::Comment(_) => { "".to_string() }
   }
}

pub fn unparse_element(e: &Element) -> String {
   let mut es = String::new();

   es.push_str("<");
   es.push_str(&e.name);

   if let Some(ref id) = e.id {
      es.push_str(&format!(r#" id="{}""#, id));
   }

   if e.classes.len()>0 {
      es.push_str(&format!(r#" class="{}""#, e.classes.join(" ")));
   }

   let mut kvs: Vec<_> = e.attributes.iter().collect();
   kvs.sort_by(|x,y| x.0.cmp(&y.0));
   for (k,v) in kvs.iter() {
      if let Some(v) = v {
         es.push_str(&format!(r#" {}="{}""#, k, v.replace("\"","\\\"")));
      } else {
         es.push_str(&format!(" {}", k));
      }
   }

   match e.variant {
      ElementVariant::Normal => {
         es.push_str(">");
         for c in e.children.iter() {
            es.push_str(&unparse_node(c));
         }
      },
      ElementVariant::Void => {}
   }

   match e.variant {
      ElementVariant::Normal => {
         es.push_str(&format!("</{}>", e.name));
      },
      ElementVariant::Void => {
         es.push_str("/>");
      }
   }

   es
}
