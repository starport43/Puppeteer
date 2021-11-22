use core::{fmt::Debug, format_args};
use log::{trace, Level};
use std::panic;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element, HtmlElement, Window};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod styles;
pub use styles::*;
mod global;
pub use global::*;
mod flex;
pub use flex::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    match console_log::init_with_level(Level::Trace) {
        Ok(_) => (),
        Err(e) => trace!("{:?}", e),
    }

    let mut column_style = PuppetStyle::default();
    column_style.width(PuppetLength::Half);
    column_style.height(PuppetLength::ViewPortHeight);
    column_style.min_width(PuppetUnit::Pixels(50));
    column_style.min_height(PuppetUnit::Pixels(50));
    column_style.background_color(PuppetColor::Rgb(0, 0, 0));

    let mut column = PuppetColumn::new();
    column.column_id("foo");
    column.style = column_style.clone();
    let document = get_document();

    column.to_html(&document, &html_app_node())?;

    Ok(())
}

pub(crate) fn get_window() -> Window {
    match web_sys::window() {
        Some(window) => window,
        None => {
            trace!("Could not get the current Window");
            panic!();
        }
    }
}

pub(crate) fn get_document() -> Document {
    let window = get_window();

    match window.document() {
        Some(document) => document,
        None => {
            trace!("Could not get the current Html Document");
            panic!();
        }
    }
}

pub(crate) fn html_app_node() -> Element {
    let app_id = get_document().get_element_by_id("app");

    match app_id {
        Some(app_id) => app_id,
        None => {
            trace!(
                r#"Could not get the current HTML node with an ID of `app`. Try creating a 
                    `<section id = "app"></section>`
                "#
            );
            panic!();
        }
    }
}

/// A opinionated GUI builder that makes it difficult to mess up UI/UX experiences.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PuppetColumn {
    column_id: Option<&'static str>,
    style: PuppetStyle,
    rows: Vec<PuppetRow>,
}

impl PuppetColumn {
    pub fn new() -> Self {
        Self {
            column_id: Option::default(),
            style: PuppetStyle::default(),
            rows: Vec::default(),
        }
    }
    pub fn column_id(&mut self, id: &'static str) -> &mut Self {
        if !id.is_empty() {
            self.column_id = Some(id);

            self
        } else {
            self.column_id = None;

            self
        }
    }

    pub fn set_style(&mut self, style: &PuppetStyle) -> &mut Self {
        self.style = *style;

        self
    }

    pub fn to_html(&self, document: &Document, app_node: &Element) -> JsValueResult<()> {
        //-> HtmlDivElement {
        let div = document.create_element("div")?;

        let div = div.dyn_ref::<HtmlElement>().ok_or(JsValue::from_str(
            "Could not convert `Element` to `HtmlElement`",
        ));
        let div = div?;

        match self.column_id {
            Some(column_id) => div.set_id(column_id),
            None => (),
        }

        self.style.to_html(&div)?;

        self.rows.iter().for_each(|puppet_row| {
            div.append_with_str_1(&puppet_row.to_html());
        });

        app_node.append_with_node_1(div)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PuppetRow {
    column_id: Option<&'static str>,
    style: PuppetStyle,
    components: Vec<Component>,
}


impl PuppetRow {
    pub fn new() -> Self {
        Self {
            column_id: Option::default(),
            style: PuppetStyle::default(),
            components: Vec::default(),
        }
    }
    pub fn column_id(&mut self, id: &'static str) -> &mut Self {
        if !id.is_empty() {
            self.column_id = Some(id);

            self
        } else {
            self.column_id = None;

            self
        }
    }

    pub fn set_style(&mut self, style: &PuppetStyle) -> &mut Self {
        self.style = *style;

        self
    }

    pub fn to_html(&self, document: &Document) -> JsValueResult<()> {
        //-> HtmlDivElement {
        let div = document.create_element("div")?;

        let div = div.dyn_ref::<HtmlElement>().ok_or(JsValue::from_str(
            "Could not convert `Element` to `HtmlElement`",
        ));
        let div = div?;

        match self.column_id {
            Some(column_id) => div.set_id(column_id),
            None => (),
        }

        self.style.to_html(&div)?;

        self.components.iter().for_each(|puppet_row| {
            div.append_with_str_1(&puppet_row.to_html());
        });

        app_node.append_with_node_1(div)?;

        Ok(())
    }