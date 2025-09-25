use std::fs::File;
use std::io::Read;
use std::{collections::HashMap, fs, hash::Hash};

use super::template;
use axum::Form;
use axum::response::{Html, Result};
use axum::{
    Router,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use itertools::Itertools;
use kuchikiki::iter::{Descendants, Elements, NodeIterator, Siblings};
use kuchikiki::traits::TendrilSink;
use kuchikiki::{ElementData, NodeDataRef, NodeRef, ParseOpts, Selector};
use lazy_static::lazy_static;
use markup5ever::{LocalName, QualName};
use maud::{Markup, PreEscaped, html};
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::json;
use serde_with::NoneAsEmptyString;
use serde_with::serde_as;
use tl::{HTMLTag, Node};

use crate::admin::Company;
use crate::{
    AppState,
    blocks::set_0::{self, angled_element::AngledElement},
};

#[derive(Debug)]
pub enum BlockError {
    ParseFromTagError(String),
}

pub trait Block: std::fmt::Debug {
    fn id(&self) -> &str;
    fn properties(&self) -> Markup;
    fn update(
        &mut self,
        properties: &HashMap<String, String>,
        original: &NodeDataRef<ElementData>,
    ) -> NodeRef;
    fn render_to_noderef(&self) -> NodeRef;
    fn from_element_ref(node: &NodeDataRef<ElementData>) -> Result<Box<dyn Block>, BlockError>
    where
        Self: Sized;
    fn default() -> Box<dyn Block>
    where
        Self: Sized;
}

pub struct ListEntry<'a> {
    name: &'a str,
    description: &'a str,
    image: &'a str,
    from_element_ref: fn(&NodeDataRef<ElementData>) -> Result<Box<dyn Block>, BlockError>,
    default: fn() -> Box<dyn Block>,
}

pub static CONFIG: Lazy<HashMap<&'static str, ListEntry>> = Lazy::new(|| {
    let mut entries = HashMap::new();

    entries.insert("angled-element", ListEntry {
        name: "Angled Block",
        description: "A block whose top and bottom can be angled, and given the signature Groep van Roey arrow.",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::angled_element::AngledElement::from_element_ref,
        default: set_0::angled_element::AngledElement::default
    });

    entries.insert("default-container", ListEntry {
        name: "Default Container",
        description: "A default container (section).",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::default_container::DefaultContainer::from_element_ref,
        default: set_0::default_container::DefaultContainer::default
    });

    entries.insert("theme-text", ListEntry {
        name: "Theme Text",
        description: "A small piece of text, used to indicated the theme of a section.",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::theme_text::ThemeText::from_element_ref,
        default: set_0::theme_text::ThemeText::default
    });

    entries.insert("h2", ListEntry {
        name: "Heading 2",
        description: "A heading element of type h2.",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::heading_2::Heading2::from_element_ref,
        default: set_0::heading_2::Heading2::default
    });

    entries.insert("p", ListEntry {
        name: "Paragraph",
        description: "A paragraph element.",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::paragraph::Paragraph::from_element_ref,
        default: set_0::paragraph::Paragraph::default
    });

    entries.insert("main-content", ListEntry {
        name: "Main Content",
        description: "A large block that can contain a title.",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::main_content::MainContent::from_element_ref,
        default: set_0::main_content::MainContent::default
    });

    entries.insert("carousel", ListEntry {
        name: "Carousel",
        description: "A carousel.",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::carousel::Carousel::from_element_ref,
        default: set_0::carousel::Carousel::default
    });

    entries.insert("animated-button", ListEntry {
        name: "Animated Button",
        description: "A button.",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::animated_button_1::AnimatedButton1::from_element_ref,
        default: set_0::animated_button_1::AnimatedButton1::default
    });

    entries.insert("banner-0", ListEntry {
        name: "Banner",
        description: "A banner",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::banner_0::Banner0::from_element_ref,
        default: set_0::banner_0::Banner0::default
    });

    entries.insert("a-carousel", ListEntry {
        name: "A-Carousel",
        description: "A-Carousel",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::a_carousel::ACarousel::from_element_ref,
        default: set_0::a_carousel::ACarousel::default
    });

    entries.insert("a-carousel-entry", ListEntry {
        name: "A-Carousel entry",
        description: "A-Carousel entry",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::a_carousel_entry::ACarouselEntry::from_element_ref,
        default: set_0::a_carousel_entry::ACarouselEntry::default
    });

    entries.insert("tabs", ListEntry {
        name: "Tabs",
        description: "Tabs",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::tabs::Tabs::from_element_ref,
        default: set_0::tabs::Tabs::default
    });

    entries.insert("tabs-entry", ListEntry {
        name: "Tabs entry",
        description: "Tabs entry",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::tabs_entry::TabsEntry::from_element_ref,
        default: set_0::tabs_entry::TabsEntry::default
    });

    entries.insert("map", ListEntry {
        name: "Map",
        description: "Map",
        image: "https://nintendolesite.com/images/uploads/professeur-layton-et-le-nouveau-monde--vapeur.jpg",
        from_element_ref: set_0::map::Map::from_element_ref,
        default: set_0::map::Map::default
    });

    entries
});

pub async fn list() -> Markup {
    html!(
        @for (block_id, entry) in CONFIG.iter() {
            label {
                img loading="lazy" src=(entry.image);
                div.name { (entry.name )}
                div.description { (entry.description )}
                input type="radio" name="block_id" value=(block_id);
            }
        }
    )
}

pub async fn testpage() -> Html<String> {
    let path = "home.html";
    let mut file = match File::open(format!("pages/draft/{path}")) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                fs::copy(format!("pages/live/{path}"), format!("pages/draft/{path}")).unwrap();

                match File::open(format!("pages/draft/{path}")) {
                    Ok(file) => file,
                    Err(_) => todo!(),
                }
            }
            _ => todo!(),
        },
    };

    //let fragment = kuchikiki::parse_html().from_utf8().read_from(&mut file).unwrap().last_child().unwrap().last_child().unwrap();

    //let rendered_elements = get_testpage_list_rendered(fragment.children().elements());

    let mut x = String::new();
    file.read_to_string(&mut x).unwrap();

    Html(x)
}

fn find_element_in_list_2<'a>(
    elements: Elements<Siblings>,
    element_id: &'a str,
) -> Option<NodeDataRef<ElementData>> {
    let mut element = None;

    for e in elements {
        element = match e
            .attributes
            .borrow()
            .get("element-id")
            .map_or(false, |eid| eid == element_id)
        {
            true => Some(e.clone()),
            false => find_element_in_list_2(e.as_node().children().elements(), element_id),
        };

        if element.is_some() {
            break;
        }
    }

    element
}

fn get_testpage_tree_rendered(elements: Elements<Siblings>) -> Vec<Markup> {
    let mut rendered = vec![];

    for element in elements {
        let attributes = element.attributes.borrow();

        if let (Some(block_id), Some(element_id)) =
            (attributes.get("block-id"), attributes.get("element-id"))
        {
            if let Some(entry) = CONFIG.get(block_id) {
                let child_count = element
                    .as_node()
                    .children()
                    .elements() /* .filter(|c| {
                        let child_attributes = c.as_element().unwrap().attributes
                        let (Some(block_id), Some(element_id)) =
                    }) */
                    .count();
                rendered.push(template::tree_item(
                    element_id,
                    entry.name,
                    child_count,
                    false,
                ));
            }
        }
    }

    rendered
}

#[derive(Deserialize)]
pub struct TestPageTreeQuery {
    parent: Option<String>,
}

pub async fn testpage_tree(Query(query): Query<TestPageTreeQuery>) -> Result<Markup> {
    let path = "home.html";
    let mut file = match File::open(format!("pages/draft/{path}")) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                fs::copy(format!("pages/live/{path}"), format!("pages/draft/{path}")).unwrap();

                match File::open(format!("pages/draft/{path}")) {
                    Ok(file) => file,
                    Err(_) => todo!(),
                }
            }
            _ => todo!(),
        },
    };

    let fragment = kuchikiki::parse_html()
        .from_utf8()
        .read_from(&mut file)
        .unwrap()
        .last_child()
        .unwrap()
        .last_child()
        .unwrap();

    let elements = match &query.parent {
        Some(parent_id) => {
            match find_element_in_list_2(fragment.children().elements(), parent_id) {
                Some(element) => element.as_node().children().elements(),
                None => return Err(StatusCode::NOT_FOUND.into()),
            }
        }
        None => fragment.children().elements(),
    };

    let rendered_elements = get_testpage_tree_rendered(elements);

    Ok(html!(
        @for element in rendered_elements {
            (element)
        }
    ))
}

#[serde_as]
#[derive(Deserialize)]
pub struct CreateNewElementBody {
    block_id: String,
    element_id: String,
    #[serde_as(as = "NoneAsEmptyString")]
    direction: Option<bool>,
}

pub async fn create_new_element(Form(body): Form<CreateNewElementBody>) -> Result<Markup> {
    let new_block = CONFIG.get(body.block_id.as_str()).unwrap();
    let new_element = (new_block.default)();
    let rendered_new_element = new_element.render_to_noderef();
    let new_li = template::tree_item(new_element.id(), new_block.name, 0, false);

    let mut file = File::options()
        .read(true)
        .write(true)
        .open("pages/draft/home.html")
        .unwrap();

    let html = kuchikiki::parse_html()
        .from_utf8()
        .read_from(&mut file)
        .unwrap();
    let reference_element =
        find_element_in_list_2(html.children().elements(), &body.element_id).unwrap();
    let reference_elmeent_attributes = reference_element.attributes.borrow();
    let reference_element_block_id = reference_elmeent_attributes.get("block-id").unwrap();
    let new_block = CONFIG.get(reference_element_block_id).unwrap();
    let node = reference_element.as_node();

    let direction = match body.direction {
        Some(direction) => {
            if direction {
                node.insert_before(rendered_new_element);
                "beforebegin"
            } else {
                node.insert_after(rendered_new_element);
                "afterend"
            }
        }
        None => {
            node.append(rendered_new_element);
            "none"
        }
    };

    let reference_child_count = node.children().elements().count();

    let li: PreEscaped<String> = template::tree_item(
        &body.element_id,
        new_block.name,
        reference_child_count,
        true,
    );

    html.serialize_to_file("pages/draft/home.html").unwrap();

    Ok(html!(
        (li)
        div hx-swap-oob={(direction)":li[element-id='"(body.element_id)"']"} {
            (new_li)
        }
    ))
}

pub async fn delete_element(Path(element_id): Path<String>) -> Result<()> {
    let mut file = File::options()
        .read(true)
        .write(true)
        .open("pages/draft/home.html")
        .unwrap();

    let html = kuchikiki::parse_html()
        .from_utf8()
        .read_from(&mut file)
        .unwrap();

    let element_ref = find_element_in_list_2(html.children().elements(), &element_id).unwrap();
    let element = element_ref.as_node();

    element.detach();

    html.serialize_to_file("pages/draft/home.html").unwrap();

    Ok(())
}

pub async fn get_properties(Path(element_id): Path<String>) -> Result<Markup> {
    let mut file = File::options()
        .read(true)
        .write(true)
        .open("pages/draft/home.html")
        .unwrap();

    let html = kuchikiki::parse_html()
        .from_utf8()
        .read_from(&mut file)
        .unwrap();

    let element_ref = find_element_in_list_2(html.children().elements(), &element_id).unwrap();
    let attributes = element_ref.attributes.borrow();
    let block_id = attributes.get("block-id").unwrap();
    let new_block = CONFIG.get(block_id).unwrap();

    let generated_block = (new_block.from_element_ref)(&element_ref).unwrap();
    let properties = generated_block.properties();

    Ok(properties)
}

pub async fn update_properties(
    Path(element_id): Path<String>,
    Form(properties): Form<HashMap<String, String>>,
) -> Result<()> {
    let mut file = File::options()
        .read(true)
        .write(true)
        .open("pages/draft/home.html")
        .unwrap();

    let html = kuchikiki::parse_html()
        .from_utf8()
        .read_from(&mut file)
        .unwrap();

    let element_ref = find_element_in_list_2(html.children().elements(), &element_id).unwrap();
    let node = element_ref.as_node();
    let attributes = element_ref.attributes.borrow();
    let block_id = attributes.get("block-id").unwrap();
    let new_block = CONFIG.get(block_id).unwrap();

    let mut generated_block = (new_block.from_element_ref)(&element_ref).unwrap();
    let generated_noderef = generated_block.update(&properties, &element_ref);

    node.insert_after(generated_noderef);
    node.detach();

    html.serialize_to_file("pages/draft/home.html").unwrap();

    Ok(())
}

#[derive(Deserialize)]
pub struct MoveItemBody {
    next: Option<String>,
}

pub async fn move_item(
    Path(element_id): Path<String>,
    Form(body): Form<MoveItemBody>,
) -> Result<()> {
    let mut file = File::options()
        .read(true)
        .write(true)
        .open("pages/draft/home.html")
        .unwrap();

    let file_contents = fs::read_to_string("pages/draft/home.html").unwrap();

    println!("{file_contents}");

    let html = kuchikiki::parse_html()
        .from_utf8()
        .read_from(&mut file_contents.as_bytes())
        .unwrap();

    let moved_element = html
        .select(&format!("[element-id='{element_id}']"))
        .unwrap()
        .next()
        .unwrap();
    let moved_node = moved_element.as_node();
    let cloned_moved_node = moved_node.clone();
    let parent = moved_node.parent().unwrap();

    if let Some(next_id) = body.next {
        let next_element = parent
            .select(&format!("[element-id='{next_id}']"))
            .unwrap()
            .next()
            .unwrap();
        let next_node = next_element.as_node();

        next_node.insert_before(cloned_moved_node);
    } else {
        parent.append(cloned_moved_node);
    }

    html.serialize_to_file("pages/draft/home.html").unwrap();

    Ok(())
}

const RESERVED_FILE_NAMES: [&str; 1] = [
    "home"
];

struct PageList(Vec<(String, Option<PageList>)>);

pub fn get_pages(path: &str) -> PageList {
    let mut list = vec![];
    
    for page in fs::read_dir(format!("pages/live/{path}")).unwrap() {
        let page = page.unwrap();

        let name = page.file_name().into_string().unwrap().replace(".html", "");
        let is_directory = page.file_type().unwrap().is_dir();

        if !RESERVED_FILE_NAMES.contains(&name.as_str()) {
            let mut page_list = None;

            if is_directory {
                page_list = Some(get_pages(&format!("{path}/{name}")));
            }

            list.push((name, page_list));
        }
    }

    PageList(list)
}

pub fn render_page_list(company: &str, PageList(page_list): &PageList) -> Markup {
    html!(
        @for entry in page_list {
            li class={
                @if entry.1.is_some() {
                    "folder"
                } @else {
                    "page"
                }
            } {
                @if let Some(sub_list) = &entry.1 {
                    details {
                        summary {
                            span { (entry.0) }
                            button.add {}
                            form.new hx-post="/api/blocks/pages" hx-push-url="false" {
                                input name="name";
                                input type="submit" value="";
                            }
                        }
                        
                        ul {
                            (render_page_list(company, sub_list))
                        }
                    }
                } @else {
                    (entry.0)
                }
            }
        }
    )
}

pub async fn pages(Company(company): Company<String>,) -> Result<Markup> {
    let list = get_pages(&company);

    Ok(render_page_list(&company, &list))
}

#[derive(Deserialize)]
pub struct CreatePageBody {
    name: String
}

pub async fn create_page(Company(company): Company<String>, Form(body): Form<CreatePageBody>) -> Result<Markup> {
    println!("{company:?}: {}", body.name);

    Ok(html!())
}