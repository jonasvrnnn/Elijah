use std::{collections::HashMap, mem};

use byteorder::{LittleEndian, WriteBytesExt};
use kuchikiki::{Attribute, ElementData, ExpandedName, NodeDataRef, NodeRef};
use markup5ever::{LocalName, QualName};
use maud::html;
use tl::Node;
use uuid::Uuid;

use crate::blocks::{
    config::{Block, BlockError},
    properties,
};

const BACKGROUND_COLOURS: [&str; 2] = ["off-white", "alabaster"];

#[derive(Debug)]
pub struct DefaultContainer {
    pub id: String,
    pub colour: Option<String>,
}

impl Block for DefaultContainer {
    fn id(&self) -> &'_ str {
        &self.id
    }

    fn from_element_ref(element: &NodeDataRef<ElementData>) -> Result<Box<dyn Block>, BlockError> {
        let attributes = element.attributes.borrow();

        let element_id = attributes.get("element-id").unwrap();

        let classlist = attributes.get("class").unwrap_or_default();

        let colour = BACKGROUND_COLOURS
            .iter()
            .find(|v| classlist.contains(**v))
            .map(|v| v.to_string());

        Ok(Box::new(Self {
            id: element_id.to_string(),
            colour,
        }))
    }

    fn default() -> Box<dyn Block> {
        let id = Uuid::new_v4().to_string();

        Box::new(Self { id, colour: None })
    }

    fn render_to_noderef(&self) -> kuchikiki::NodeRef {
        let mut class = String::from("container-0 content-0");

        if let Some(colour) = &self.colour {
            class.push_str(&format!(" {colour}"));
        }

        let attributes = vec![
            (
                ExpandedName::new(ns!(), LocalName::from("block-id")),
                Attribute {
                    prefix: None,
                    value: "default-container".to_string(),
                },
            ),
            (
                ExpandedName::new(ns!(), LocalName::from("element-id")),
                Attribute {
                    prefix: None,
                    value: self.id.clone(),
                },
            ),
            (
                ExpandedName::new(ns!(), LocalName::from("class")),
                Attribute {
                    prefix: None,
                    value: class,
                },
            ),
        ];

        let element = NodeRef::new_element(
            QualName::new(None, ns!(html), LocalName::from("section")),
            attributes,
        );

        element
    }

    fn properties(&self) -> maud::Markup {
        html!(
            form hx-put={"/api/blocks/"(self.id)"/properties"} hx-trigger="change" hx-on::after-request="refresh_preview()" hx-swap="none" {
                label {
                    "Background"
                    (properties::select("colour", &self.colour, &BACKGROUND_COLOURS.to_vec()))
                }
            }
        )
    }

    fn update(
        &mut self,
        properties: &HashMap<String, String>,
        original: &NodeDataRef<ElementData>,
    ) -> NodeRef {
        self.colour = properties.get("colour").map(|v| v.parse().unwrap());

        let generated = self.render_to_noderef();

        for child in original.as_node().children() {
            generated.append(child);
        }

        generated
    }
}
