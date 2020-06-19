
use html_parser::{Dom, Node, Result, Element, ElementVariant};

#[derive(Clone)]
pub enum Find {
   ChildElement
}
#[derive(Clone)]
pub enum Match {
   HasTag(String),
   HasId(String),
   HasClass(String),
   HasAttribute(String),
   HasAttributeValue(String,String)
}
#[derive(Clone)]
pub enum Edit {
   AddId(String),
   AddClass(String),
   AddAttribute(String,String),
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

pub fn fme(_d: &mut Dom, _fme: &FindMatchEditElement) {
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

   for (k,v) in e.attributes.iter() {
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
