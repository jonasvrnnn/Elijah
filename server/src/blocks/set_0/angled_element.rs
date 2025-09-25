use std::{collections::HashMap, mem};

use super::super::properties;
use byteorder::{LittleEndian, WriteBytesExt};
use kuchikiki::Attribute;
use kuchikiki::ElementData;
use kuchikiki::ExpandedName;
use kuchikiki::NodeDataRef;
use kuchikiki::NodeRef;
use markup5ever::{LocalName, QualName};
use maud::Markup;
use maud::html;
use tl::Node;
use uuid::Uuid;

use crate::blocks::config::{Block, BlockError};

#[derive(Debug)]
pub struct AngledElement {
    pub id: String,
    pub content: Option<Box<dyn Block>>,
    pub angle_top: Option<f32>,
    pub angle_bottom: Option<f32>,
    pub triangle_top: Option<bool>,
    pub triangle_bottom: Option<bool>,
    pub triangle_top_position: Option<f32>,
    pub triangle_bottom_position: Option<f32>,
    pub triangle_top_size: Option<f32>,
    pub triangle_bottom_size: Option<f32>,
}

impl Block for AngledElement {
    fn id(&self) -> &'_ str {
        &self.id
    }

    fn from_element_ref(element: &NodeDataRef<ElementData>) -> Result<Box<dyn Block>, BlockError> {
        let attributes = element.attributes.borrow();

        let element_id = attributes.get("element-id").unwrap();
        let angle_top: Option<f32> = attributes.get("angle-top").map(|v| v.parse().unwrap());
        let angle_bottom: Option<f32> = attributes.get("angle-bottom").map(|v| v.parse().unwrap());
        let triangle_top = attributes
            .get("triangle-top")
            .map(|v| v.is_empty() || v != "false");
        let triangle_bottom = attributes
            .get("triangle-bottom")
            .map(|v| v.is_empty() || v != "false");
        let triangle_top_position = attributes
            .get("triangle-top-position")
            .map(|v| v.parse().unwrap());
        let triangle_bottom_position = attributes
            .get("triangle-bottom-position")
            .map(|v| v.parse().unwrap());
        let triangle_top_size = attributes
            .get("triangle-top-size")
            .map(|v| v.parse().unwrap());
        let triangle_bottom_size = attributes
            .get("triangle-bottom-size")
            .map(|v| v.parse().unwrap());

        Ok(Box::new(Self {
            id: element_id.to_string(),
            content: None,
            angle_top,
            angle_bottom,
            triangle_top,
            triangle_bottom,
            triangle_top_position,
            triangle_bottom_position,
            triangle_top_size,
            triangle_bottom_size,
        }))
    }

    fn default() -> Box<dyn Block> {
        let id = Uuid::new_v4().to_string();

        Box::new(Self {
            id,
            content: None,
            angle_top: None,
            angle_bottom: Some(2.0),
            triangle_top: None,
            triangle_bottom: Some(true),
            triangle_top_position: None,
            triangle_bottom_position: Some(0.125),
            triangle_top_size: None,
            triangle_bottom_size: Some(60.0),
        })
    }

    fn render_to_noderef(&self) -> NodeRef {
        let mut attributes = vec![
            (
                ExpandedName::new(ns!(), LocalName::from("block-id")),
                Attribute {
                    prefix: None,
                    value: "angled-element".to_string(),
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

        if let Some(angle_top) = self.angle_top {
            attributes.push((
                ExpandedName::new(ns!(), LocalName::from("angle-top")),
                Attribute {
                    prefix: None,
                    value: angle_top.to_string(),
                },
            ));
        }

        if let Some(angle_bottom) = self.angle_bottom {
            attributes.push((
                ExpandedName::new(ns!(), LocalName::from("angle-bottom")),
                Attribute {
                    prefix: None,
                    value: angle_bottom.to_string(),
                },
            ));
        }

        if let Some(triangle_top) = self.triangle_top {
            attributes.push((
                ExpandedName::new(ns!(), LocalName::from("triangle-top")),
                Attribute {
                    prefix: None,
                    value: triangle_top.to_string(),
                },
            ));
        }

        if let Some(triangle_bottom) = self.triangle_bottom {
            attributes.push((
                ExpandedName::new(ns!(), LocalName::from("triangle-bottom")),
                Attribute {
                    prefix: None,
                    value: triangle_bottom.to_string(),
                },
            ));
        }

        if let Some(triangle_top_position) = self.triangle_top_position {
            attributes.push((
                ExpandedName::new(ns!(), LocalName::from("triangle-top-position")),
                Attribute {
                    prefix: None,
                    value: triangle_top_position.to_string(),
                },
            ));
        }

        if let Some(triangle_bottom_position) = self.triangle_bottom_position {
            attributes.push((
                ExpandedName::new(ns!(), LocalName::from("triangle-bottom-position")),
                Attribute {
                    prefix: None,
                    value: triangle_bottom_position.to_string(),
                },
            ));
        }

        if let Some(triangle_top_size) = self.triangle_top_size {
            attributes.push((
                ExpandedName::new(ns!(), LocalName::from("triangle-top-size")),
                Attribute {
                    prefix: None,
                    value: triangle_top_size.to_string(),
                },
            ));
        }

        if let Some(triangle_bottom_size) = self.triangle_bottom_size {
            attributes.push((
                ExpandedName::new(ns!(), LocalName::from("triangle-bottom-size")),
                Attribute {
                    prefix: None,
                    value: triangle_bottom_size.to_string(),
                },
            ));
        }

        NodeRef::new_element(
            QualName::new(None, ns!(html), LocalName::from("angled-element")),
            attributes,
        )
    }

    fn properties(&self) -> Markup {
        html!(
            form hx-put={"/api/blocks/"(self.id)"/properties"} hx-trigger="change" hx-on::after-request="refresh_preview()" hx-swap="none" {
                label {
                    "Top Angle"
                    (properties::number("angle_top", &self.angle_top, Some(-90.0), Some(90.0), Some(0.1)))
                }
                label {
                    "Bottom Angle"
                    (properties::number("angle_bottom", &self.angle_bottom, Some(-90.0), Some(90.0), Some(0.1)))
                }
                label {
                    "Triangle Top"
                    (properties::boolean("triangle_top", &self.triangle_top))
                }
                label {
                    "Triangle Top Size"
                    (properties::number("triangle_top_size", &self.triangle_top_size, Some(0.0), None, Some(0.1)))
                }
                label {
                    "Triangle Top Position"
                    (properties::number("triangle_top_position", &self.triangle_top_position, Some(0.0), Some(1.0), Some(0.001)))
                }
                label {
                    "Triangle Bottom"
                    (properties::boolean("triangle_bottom", &self.triangle_bottom))
                }
                label {
                    "Triangle Bottom Size"
                    (properties::number("triangle_bottom_size", &self.triangle_bottom_size, Some(0.0), None, Some(0.1)))
                }
                label {
                    "Triangle Bottom Position"
                    (properties::number("triangle_bottom_position", &self.triangle_bottom_position, Some(0.0), Some(1.0), Some(0.001)))
                }
            }
        )
    }

    fn update(&mut self, properties: &HashMap<std::string::String, std::string::String>, original: &NodeDataRef<ElementData>) -> NodeRef {
        self.angle_top = properties.get("angle_top").filter(|v| !v.is_empty()).map(|v| v.parse().unwrap());
        self.angle_bottom = properties.get("angle_bottom").filter(|v| !v.is_empty()).map(|v| v.parse().unwrap());
        self.triangle_top = properties.get("triangle_top").map(|v| v == "true");
        self.triangle_bottom = properties.get("triangle_bottom").map(|v| v == "true");
        self.triangle_top_size = properties
            .get("triangle_top_size")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().unwrap());
        self.triangle_bottom_size = properties
            .get("triangle_bottom_size")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().unwrap());
        self.triangle_top_position = properties
            .get("triangle_top_position")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().unwrap());
        self.triangle_bottom_position = properties
            .get("triangle_bottom_position")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().unwrap());

        let generated = self.render_to_noderef();

        for child in original.as_node().children() {
            generated.append(child);
        }

        generated
    }
}
