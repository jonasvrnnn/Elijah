use std::{collections::HashMap, mem};

use byteorder::{LittleEndian, WriteBytesExt};
use kuchikiki::{Attribute, Attributes, ElementData, ExpandedName, NodeDataRef, NodeRef};
use markup5ever::{LocalName, QualName};
use maud::{Markup, html};
use regex::Regex;
use tl::Node;
use uuid::Uuid;

use crate::blocks::config::{Block, BlockError};

use super::super::properties;

#[derive(Debug)]
pub struct Map {
    pub id: String,
    pub pb: String,
}

const GVR_PB: &str = "!1m18!1m12!1m3!1d2492.816199321357!2d4.746279676946868!3d51.33290042343565!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47c6a96e717a03c9%3A0x46004d158a49ceb8!2sGroep%20Van%20Roey!5e0!3m2!1snl!2sbe!4v1694701918719!5m2!1snl!2sbe";

impl Block for Map {
    fn id(&self) -> &'_ str {
        &self.id
    }

    fn from_element_ref(element: &NodeDataRef<ElementData>) -> Result<Box<dyn Block>, BlockError> {
        let attributes = element.attributes.borrow();

        let element_id = attributes.get("element-id").unwrap();

        let pb = attributes.get("style").and_then(|pb| {
            let regex = Regex::new(r#"src="https://www.google.com/maps/embed?pb=([^)]+)""#).unwrap();

            regex.captures(pb).unwrap().get(1).map(|i| i.as_str().to_string())
        }).unwrap_or(String::from(GVR_PB));

        Ok(Box::new(Self {
            id: element_id.to_string(),
            pb,
        }))
    }

    fn default() -> Box<dyn Block> {
        let id = Uuid::new_v4().to_string();

        Box::new(Self {
            id,
            pb: GVR_PB.to_string()
        })
    }

    fn render_to_noderef(&self) -> kuchikiki::NodeRef {
        let element = NodeRef::new_element(
            QualName::new(None, ns!(html), LocalName::from("iframe")),
            vec![
                (
                    ExpandedName::new(ns!(), LocalName::from("block-id")),
                    Attribute {
                        prefix: None,
                        value: "map".to_string(),
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
                    ExpandedName::new(ns!(), LocalName::from("src")),
                    Attribute {
                        prefix: None,
                        value: format!("https://www.google.com/maps/embed?pb={}", self.pb),
                    },
                ),
                (
                    ExpandedName::new(ns!(), LocalName::from("frameborder")),
                    Attribute {
                        prefix: None,
                        value: 0.to_string(),
                    },
                ),
                (
                    ExpandedName::new(ns!(), LocalName::from("allowfullscreen")),
                    Attribute {
                        prefix: None,
                        value: "true".to_string(),
                    },
                ),
                (
                    ExpandedName::new(ns!(), LocalName::from("loading")),
                    Attribute {
                        prefix: None,
                        value: "lazy".to_string(),
                    },
                ),
            ],
        );

        element
    }

    fn properties(&self) -> Markup {
        html!(
            form hx-put={"/api/blocks/"(self.id)"/properties"} hx-trigger="change" hx-on::after-request="refresh_preview()" hx-swap="none" {
                label {
                    "PB-waarde"
                    (properties::inner_text("pb", &Some(self.pb.clone()), false))
                }
            }
        )
    }

    fn update(
        &mut self,
        properties: &HashMap<std::string::String, std::string::String>,
        original: &NodeDataRef<ElementData>,
    ) -> NodeRef {
        self.pb = properties.get("pb").map_or(GVR_PB, |v| v).to_string();

        self.render_to_noderef()
    }
}
