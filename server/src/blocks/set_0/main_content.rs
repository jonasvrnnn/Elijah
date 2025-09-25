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
pub struct MainContent {
    pub id: String,
    pub title: Option<String>,
    pub video: Option<String>,
}

impl Block for MainContent {
    fn id(&self) -> &'_ str {
        &self.id
    }

    fn from_element_ref(element: &NodeDataRef<ElementData>) -> Result<Box<dyn Block>, BlockError> {
        let attributes = element.attributes.borrow();

        let element_id = attributes.get("element-id").unwrap();

        let title = element
            .as_node()
            .select("h1")
            .unwrap()
            .next()
            .and_then(|v| v.as_node().first_child())
            .map(|v| v.as_text().unwrap().take());

        let video = element
            .as_node()
            .select("video")
            .unwrap()
            .next()
            .and_then(|v| v.attributes.borrow().get("src").map(|v| v.to_string()));

        Ok(Box::new(Self {
            id: element_id.to_string(),
            title,
            video,
        }))
    }

    fn default() -> Box<dyn Block> {
        let id = Uuid::new_v4().to_string();

        Box::new(Self {
            id,
            title: Some("Lorem Ipsum\ndolor".to_string()),
            video: Some(
                "https://mediabox.groepvanroey.be/m/62e6b2ac1979b802/original/home-banner-0.webm"
                    .to_string(),
            ),
        })
    }

    fn render_to_noderef(&self) -> kuchikiki::NodeRef {
        let attributes = vec![
            (
                ExpandedName::new(ns!(), LocalName::from("block-id")),
                Attribute {
                    prefix: None,
                    value: "main-content".to_string(),
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
                    value: String::from("main-content"),
                },
            ),
        ];

        let element = NodeRef::new_element(
            QualName::new(None, ns!(html), LocalName::from("section")),
            attributes,
        );

        if let Some(title) = &self.title {
            let title_element = NodeRef::new_element(
                QualName::new(None, ns!(html), LocalName::from("h1")),
                vec![],
            );

            let text = NodeRef::new_text(title);

            title_element.append(text);

            element.append(title_element);
        }

        if let Some(video) = &self.video {
            let video_element = NodeRef::new_element(
                QualName::new(None, ns!(html), LocalName::from("video")),
                vec![
                    (
                        ExpandedName::new(ns!(), LocalName::from("src")),
                        Attribute {
                            prefix: None,
                            value: video.to_string(),
                        },
                    ),
                    (
                        ExpandedName::new(ns!(), LocalName::from("loop")),
                        Attribute {
                            prefix: None,
                            value: "true".to_string(),
                        },
                    ),
                    (
                        ExpandedName::new(ns!(), LocalName::from("muted")),
                        Attribute {
                            prefix: None,
                            value: "true".to_string(),
                        },
                    ),
                    (
                        ExpandedName::new(ns!(), LocalName::from("autoplay")),
                        Attribute {
                            prefix: None,
                            value: "".to_string(),
                        },
                    ),
                    (
                        ExpandedName::new(ns!(), LocalName::from("playsinline")),
                        Attribute {
                            prefix: None,
                            value: "".to_string(),
                        },
                    )
                ],
            );

            element.append(video_element);
        }

        element
    }

    fn properties(&self) -> Markup {
        html!(
            form hx-put={"/api/blocks/"(self.id)"/properties"} hx-trigger="change" hx-on::after-request="refresh_preview()" hx-swap="none" {
                label {
                    "Title"
                    (properties::inner_text("title", &self.title, true))
                }
                label {
                    "Video"
                    (properties::inner_text("video", &self.video, false))
                }
            }
        )
    }

    fn update(&mut self, properties: &HashMap<std::string::String, std::string::String>, original: &NodeDataRef<ElementData>) -> NodeRef {
        self.video = properties.get("video").take().cloned();
        self.title = properties.get("title").take().cloned();

        self.render_to_noderef()
    }
}
