use std::{collections::HashMap, mem};

use byteorder::{LittleEndian, WriteBytesExt};
use kuchikiki::{Attribute, Attributes, ElementData, ExpandedName, NodeDataRef, NodeRef};
use markup5ever::{LocalName, QualName};
use maud::{Markup, html};
use tl::Node;
use uuid::Uuid;

use crate::blocks::config::{Block, BlockError};

use super::super::properties;

#[derive(Debug)]
pub struct Header {
    pub id: String,
    pub content: Vec<Box<dyn Block>>
}

impl Block for Header {
    fn id(&self) -> &'_ str {
        &self.id
    }

    fn render(&self) -> maud::Markup {
        let node = self.render_to_noderef();
    }

    fn from_element_ref(element: &NodeDataRef<ElementData>) -> Result<Box<dyn Block>, BlockError> {
        let attributes = element.attributes.borrow();

        let element_id = attributes.get("element-id").unwrap();

        Ok(Box::new(Self {
            id: element_id.to_string(),
            content: element.text_contents(),
        }))
    }

    fn default() -> Box<dyn Block> {
        let id = Uuid::new_v4().to_string();

        Box::new(Self {
            id,
            content: "Lorem Ipsum Dolor".to_string(),
        })
    }

    fn render_to_noderef(&self) -> kuchikiki::NodeRef {
        let element = NodeRef::new_element(
            QualName::new(None, ns!(html), LocalName::from("div")),
            vec![
                (
                    ExpandedName::new(ns!(), LocalName::from("block-id")),
                    Attribute {
                        prefix: None,
                        value: "theme-text".to_string(),
                    },
                ),
                (
                    ExpandedName::new(ns!(), LocalName::from("element-id")),
                    Attribute {
                        prefix: None,
                        value: self.id.clone(),
                    },
                ),
            ],
        );

        element.append(NodeRef::new_text(&self.content));

        element
    }

    fn properties(&self) -> Markup {
        html!(
            form hx-put={"/api/blocks/"(self.id)"/properties"} hx-trigger="change" hx-on::after-request="refresh_preview()" hx-swap="none" {
                label {
                    "Content"
                    (properties::inner_text("content", &self.content, false))
                }
            }
        )
    }
    
    fn update(&mut self, properties: &HashMap<std::string::String, std::string::String>) {
        self.content = properties.get("content").unwrap().to_string();
    }
}
