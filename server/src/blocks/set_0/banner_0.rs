use std::{collections::HashMap, mem};

use byteorder::{LittleEndian, WriteBytesExt};
use kuchikiki::{Attribute, ElementData, ExpandedName, NodeDataRef, NodeRef};
use markup5ever::{LocalName, QualName};
use maud::html;
use regex::Regex;
use tl::Node;
use uuid::Uuid;

use crate::blocks::{
    config::{Block, BlockError},
    properties,
};

const BACKGROUND_COLOURS: [&str; 2] = ["off-white", "alabaster"];

#[derive(Debug)]
pub struct Banner0 {
    pub id: String,
    pub image: Option<String>,
    pub fixed: bool
}

impl Block for Banner0 {
    fn id(&self) -> &'_ str {
        &self.id
    }

    fn from_element_ref(element: &NodeDataRef<ElementData>) -> Result<Box<dyn Block>, BlockError> {
        let attributes = element.attributes.borrow();

        let image = attributes.get("style").and_then(|style| {
            let regex = Regex::new(r#"background-image:\s*url\(([^)]+)\);"#).unwrap();

            regex.captures(style).unwrap().get(1).map(|i| i.as_str().to_string())
        });

        let fixed = attributes.get("class").unwrap_or_default().contains("fixed");

        let element_id = attributes.get("element-id").unwrap();

        Ok(Box::new(Self {
            id: element_id.to_string(),
            image,
            fixed
        }))
    }

    fn default() -> Box<dyn Block> {
        let id = Uuid::new_v4().to_string();

        Box::new(Self { id, image: Some("https://mediabox.groepvanroey.be/m/4fa75f31a73b8f62/original/home-wat_we_doen-banner.webp".to_string()), fixed: true })
    }

    fn render_to_noderef(&self) -> kuchikiki::NodeRef {
        let mut class = String::from("banner-0 content-0");

        if self.fixed {
            class.push_str(" fixed");
        }

        let mut attributes = vec![
            (
                ExpandedName::new(ns!(), LocalName::from("block-id")),
                Attribute {
                    prefix: None,
                    value: "banner-0".to_string(),
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

        if let Some(image) = &self.image {
            attributes.push((
                ExpandedName::new(ns!(), LocalName::from("style")),
                Attribute {
                    prefix: None,
                    value: format!("background-image:url({image});"),
                },
            ));
        }

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
                    "Image"
                    (properties::inner_text("image", &self.image, false))
                }
                label {
                    "Fixed Image"
                    (properties::boolean("fixed", &Some(self.fixed)))
                }
            }
        )
    }

    fn update(
        &mut self,
        properties: &HashMap<String, String>,
        original: &NodeDataRef<ElementData>,
    ) -> NodeRef {
        self.image = properties.get("image").map(|v| v.parse().unwrap());
        self.fixed = properties.get("fixed").map(|v| v == "true").unwrap_or_default();

        let generated = self.render_to_noderef();

        for child in original.as_node().children() {
            generated.append(child);
        }

        generated
    }
}
