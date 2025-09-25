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
pub struct TabsEntry {
    pub id: String,
    pub tab_title: Option<String>,
    pub title: Option<String>,
    pub image: Option<String>,
}

impl Block for TabsEntry {
    fn id(&self) -> &'_ str {
        &self.id
    }

    fn from_element_ref(element: &NodeDataRef<ElementData>) -> Result<Box<dyn Block>, BlockError> {
        let attributes = element.attributes.borrow();

        let element_id = attributes.get("element-id").unwrap();
        let title = attributes.get("title").map(|v| v.to_string());
        let tab_title = attributes.get("tab-title").map(|v| v.to_string());
        let image = attributes.get("image").map(|v| v.to_string());

        Ok(Box::new(Self {
            id: element_id.to_string(),
            title,
            tab_title,
            image,
        }))
    }

    fn default() -> Box<dyn Block> {
        let id = Uuid::new_v4().to_string();

        Box::new(Self {
            id,
            title: Some("Title".to_string()),
            tab_title: Some("tab-title".to_string()),
            image: None,
        })
    }

    fn render_to_noderef(&self) -> kuchikiki::NodeRef {
        let mut attributes = vec![
            (
                ExpandedName::new(ns!(), LocalName::from("block-id")),
                Attribute {
                    prefix: None,
                    value: "tabs-entry".to_string(),
                },
            ),
            (
                ExpandedName::new(ns!(), LocalName::from("element-id")),
                Attribute {
                    prefix: None,
                    value: self.id.clone(),
                },
            ),
        ];

        if let Some(title) = &self.title {
            attributes.push((
                ExpandedName::new(ns!(), LocalName::from("title")),
                Attribute {
                    prefix: None,
                    value: title.to_string()
                },
            ));
        }

        if let Some(tab_title) = &self.tab_title {
            attributes.push((
                ExpandedName::new(ns!(), LocalName::from("tab-title")),
                Attribute {
                    prefix: None,
                    value: tab_title.to_string()
                },
            ));
        }

        if let Some(image) = &self.image {
            attributes.push((
                ExpandedName::new(ns!(), LocalName::from("image")),
                Attribute {
                    prefix: None,
                    value: image.to_string()
                },
            ));
        }

        let element = NodeRef::new_element(
            QualName::new(None, ns!(html), LocalName::from("div")),
            attributes,
        );

        element
    }

    fn properties(&self) -> Markup {
        html!(
            form hx-put={"/api/blocks/"(self.id)"/properties"} hx-trigger="change" hx-on::after-request="refresh_preview()" hx-swap="none" {
                label {
                    "Image"
                    (properties::inner_text("image", &self.image, false))
                }
                label {
                    "Title"
                    (properties::inner_text("title", &self.title, true))
                }
                label {
                    "Tab title"
                    (properties::inner_text("tab-title", &self.tab_title, true))
                }
            }
        )
    }

    fn update(
        &mut self,
        properties: &HashMap<std::string::String, std::string::String>,
        original: &NodeDataRef<ElementData>,
    ) -> NodeRef {
        self.title = properties.get("title").cloned();
        self.tab_title = properties.get("tab-title").cloned();
        self.image = properties.get("image").map(|v| v.parse().unwrap());

        let generated = self.render_to_noderef();

        /* for child in original.as_node().children() {
            generated.append(child);
        } */

        generated
    }
}
