use yew::prelude::*;
use web_sys::window;

#[derive(Properties, PartialEq)]
pub struct MetaProps {
    pub title: String,
    pub description: String,
    #[prop_or_default]
    pub keywords: Option<String>,
    #[prop_or_default]
    pub canonical_url: Option<String>,
    #[prop_or_default]
    pub og_image: Option<String>,
}

#[function_component(Meta)]
pub fn meta(props: &MetaProps) -> Html {
    let window = window().unwrap();
    let document = window.document().unwrap();
    
    // Mise à jour du titre
    document.set_title(&format!("{} | Lightdash - Gestion de Nœud Lightning", props.title));
    
    // Mise à jour des métadonnées
    let head = document.head().unwrap();
    
    // Description
    let meta_desc = document.create_element("meta").unwrap();
    meta_desc.set_attribute("name", "description").unwrap();
    meta_desc.set_attribute("content", &props.description).unwrap();
    head.append_child(&meta_desc).unwrap();
    
    // Keywords
    if let Some(keywords) = &props.keywords {
        let meta_keywords = document.create_element("meta").unwrap();
        meta_keywords.set_attribute("name", "keywords").unwrap();
        meta_keywords.set_attribute("content", keywords).unwrap();
        head.append_child(&meta_keywords).unwrap();
    }
    
    // Open Graph
    let meta_og_title = document.create_element("meta").unwrap();
    meta_og_title.set_attribute("property", "og:title").unwrap();
    meta_og_title.set_attribute("content", &props.title).unwrap();
    head.append_child(&meta_og_title).unwrap();
    
    let meta_og_desc = document.create_element("meta").unwrap();
    meta_og_desc.set_attribute("property", "og:description").unwrap();
    meta_og_desc.set_attribute("content", &props.description).unwrap();
    head.append_child(&meta_og_desc).unwrap();
    
    let meta_og_type = document.create_element("meta").unwrap();
    meta_og_type.set_attribute("property", "og:type").unwrap();
    meta_og_type.set_attribute("content", "website").unwrap();
    head.append_child(&meta_og_type).unwrap();
    
    if let Some(og_image) = &props.og_image {
        let meta_og_image = document.create_element("meta").unwrap();
        meta_og_image.set_attribute("property", "og:image").unwrap();
        meta_og_image.set_attribute("content", og_image).unwrap();
        head.append_child(&meta_og_image).unwrap();
    }
    
    // Twitter Card
    let meta_twitter_card = document.create_element("meta").unwrap();
    meta_twitter_card.set_attribute("name", "twitter:card").unwrap();
    meta_twitter_card.set_attribute("content", "summary_large_image").unwrap();
    head.append_child(&meta_twitter_card).unwrap();
    
    // Canonical URL
    if let Some(canonical) = &props.canonical_url {
        let link_canonical = document.create_element("link").unwrap();
        link_canonical.set_attribute("rel", "canonical").unwrap();
        link_canonical.set_attribute("href", canonical).unwrap();
        head.append_child(&link_canonical).unwrap();
    }
    
    html! {}
} 