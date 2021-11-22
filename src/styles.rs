use crate::{
    AlignContent, AlignItems, AlignSelf, FlexDirection, FlexGap, FlexWrap, JsValueResult,
    JustifyContent, PERCENTAGE_SYMBOL, PIXELS_SYMBOL,
};
use web_sys::HtmlElement;

use core::{
    fmt,
    fmt::{Debug, Display},
    format_args, write,
};
use log::trace;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PuppetStyle {
    padding: PuppetUnit,
    margin: PuppetUnit,
    width: PuppetLength,
    height: PuppetLength,
    /// Based on screen pixels
    min_width: PuppetUnit,
    /// Based on screen pixels
    max_width: PuppetUnit,
    /// Based on screen pixels
    min_height: PuppetUnit,
    /// Based on screen pixels
    max_height: PuppetUnit,
    /// How children will be aligned inside the [Column]
    alignment: PuppetAlignment,
    background_color: PuppetColor,
}

impl Default for PuppetStyle {
    fn default() -> Self {
        Self {
            padding: PuppetUnit::Percentage(2),
            margin: PuppetUnit::Pixels(0),
            width: PuppetLength::Full,
            height: PuppetLength::Normal,
            min_width: PuppetUnit::Pixels(0),
            min_height: PuppetUnit::Pixels(0),
            max_width: PuppetUnit::Pixels(0),
            max_height: PuppetUnit::Pixels(0),
            alignment: PuppetAlignment::default(),
            background_color: PuppetColor::Hex("FFFFFF"),
        }
    }
}

impl PuppetStyle {
    pub fn padding(&mut self, puppet_unit: PuppetUnit) -> &mut Self {
        self.padding = puppet_unit;

        self
    }

    pub fn margin(&mut self, puppet_unit: PuppetUnit) -> &mut Self {
        self.margin = puppet_unit;

        self
    }

    pub fn width(&mut self, puppet_length: PuppetLength) -> &mut Self {
        self.width = puppet_length;

        self
    }

    pub fn height(&mut self, puppet_length: PuppetLength) -> &mut Self {
        self.height = puppet_length;

        self
    }

    pub fn min_width(&mut self, puppet_unit: PuppetUnit) -> &mut Self {
        self.min_width = puppet_unit;

        self
    }

    pub fn min_height(&mut self, puppet_unit: PuppetUnit) -> &mut Self {
        self.min_height = puppet_unit;

        self
    }

    pub fn max_width(&mut self, puppet_unit: PuppetUnit) -> &mut Self {
        self.max_width = puppet_unit;

        self
    }

    pub fn max_height(&mut self, puppet_unit: PuppetUnit) -> &mut Self {
        self.max_height = puppet_unit;

        self
    }

    pub fn align_items(&mut self, alignment: PuppetAlignment) -> &mut Self {
        self.alignment = alignment;

        self
    }

    pub fn background_color(&mut self, background_color: PuppetColor) -> &mut Self {
        self.background_color = background_color;

        self
    }

    pub fn to_html(&self, target: &HtmlElement) -> JsValueResult<()> {
        // FIXME find an efficient way to bundle all css Properties at once
        target
            .style()
            .set_property("margin", &self.margin.to_html_units())?;
        target
            .style()
            .set_property("padding", &self.padding.to_html_units())?;
        target
            .style()
            .set_property("width", self.width.to_html_units())?;
        target
            .style()
            .set_property("height", self.height.to_html_units())?;
        target
            .style()
            .set_property("min-width", &self.min_width.to_html_units())?;
        target
            .style()
            .set_property("min-height", &self.min_height.to_html_units())?;
        target
            .style()
            .set_property("max-width", &self.max_width.to_html_units())?;
        target
            .style()
            .set_property("max-height", &self.max_height.to_html_units())?;
        target
            .style()
            .set_property("background-color", &self.background_color.to_html_color())?;

        Ok(())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Transparency {
    // Zero on alpha channel
    Full,
    Ten,
    Twenty,
    Thirty,
    Forty,
    Fifty,
    Sixty,
    Seventy,
    Eighty,
    Ninety,
    // Color is opaque on alpha channel
    Opaque,
}

impl Transparency {
    pub fn to_html_alpha(&self) -> &'static str {
        match self {
            Self::Full => "0",
            Self::Ten => "0.1",
            Self::Twenty => "0.2",
            Self::Thirty => "0.3",
            Self::Forty => "0.4",
            Self::Fifty => "0.5",
            Self::Sixty => "0.6",
            Self::Seventy => "0.7",
            Self::Eighty => "0.8",
            Self::Ninety => "0.9",
            Self::Opaque => "1",
        }
    }
}

impl Default for Transparency {
    fn default() -> Self {
        Self::Opaque
    }
}

impl Debug for Transparency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Self::Full => "Transparency::Full",
                Self::Ten => "Transparency::Ten",
                Self::Twenty => "Transparency::Twenty",
                Self::Thirty => "Transparency::Thirty",
                Self::Forty => "Transparency::Forty",
                Self::Fifty => "Transparency::Fifty",
                Self::Sixty => "Transparency::Sixty",
                Self::Seventy => "Transparency::Seventy",
                Self::Eighty => "Transparency::Eighty",
                Self::Ninety => "Transparency::Ninety",
                Self::Opaque => "Transparency::Opaque",
            }
        )
    }
}

impl Display for Transparency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Self::Full => "Full",
                Self::Ten => "Ten",
                Self::Twenty => "Twenty",
                Self::Thirty => "Thirty",
                Self::Forty => "Forty",
                Self::Fifty => "Fifty",
                Self::Sixty => "Sixty",
                Self::Seventy => "Seventy",
                Self::Eighty => "Eighty",
                Self::Ninety => "Ninety",
                Self::Opaque => "Opaque",
            }
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PuppetColor {
    Hex(&'static str),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, Transparency),
}

impl PuppetColor {
    pub fn hex_color_constraints(hex_color: &str) -> bool {
        if hex_color.len() != 6_usize {
            trace!(
                "INVALID HEX COLOR LENGTH. HEX COLOR {} MUST BE OF SIX CHARACTERS",
                hex_color
            );
            return false;
        }

        let mut check = true;

        hex_color.chars().for_each(|hex_char| {
            if hex_char > 'f' {
                // FIXME show the formatted error on the char attribute
                trace!("INVALID HEX COLOR: {}", hex_color);

                check = false;
            }
        });

        check
    }

    pub fn to_html_color(&self) -> String {
        match self {
            PuppetColor::Hex(hex_color) => {
                if PuppetColor::hex_color_constraints(&hex_color.to_lowercase()) {
                    let mut formatted_hex_color = String::default();
                    formatted_hex_color.push_str("#");
                    formatted_hex_color.push_str(&hex_color);

                    formatted_hex_color
                } else {
                    "INVALID_COLOR".into()
                }
            }
            PuppetColor::Rgb(r, g, b) => {
                format!("{}", format_args!("rgb({},{},{})", r, g, b))
            }
            PuppetColor::Rgba(r, g, b, a) => {
                format!(
                    "{}",
                    format_args!("rgb({},{},{},{})", r, g, b, a.to_html_alpha())
                )
            }
        }
    }
}

impl Default for PuppetColor {
    fn default() -> Self {
        Self::Hex("FFFFFF")
    }
}

impl Debug for PuppetColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PuppetColor::Hex(hex_color) => {
                if PuppetColor::hex_color_constraints(&hex_color.to_lowercase()) {
                    write!(f, "PuppetColor::Hex(#{:?})", hex_color)
                } else {
                    write!(f, "{:?}", "PuppetColor::Hex(INVALID_COLOR)")
                }
            }
            PuppetColor::Rgb(r, g, b) => {
                write!(f, "PuppetColor::Rgb(rgb({},{},{}))", r, g, b)
            }
            PuppetColor::Rgba(r, g, b, a) => {
                write!(f, "PuppetColor::Rgba(rgba({},{},{},{}))", r, g, b, a)
            }
        }
    }
}

impl Display for PuppetColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PuppetColor::Hex(hex_color) => {
                if PuppetColor::hex_color_constraints(&hex_color.to_lowercase()) {
                    write!(f, "#{:?}", hex_color)
                } else {
                    write!(f, "{:?}", "INVALID_COLOR")
                }
            }
            PuppetColor::Rgb(r, g, b) => {
                write!(f, "rgb({},{},{})", r, g, b)
            }
            PuppetColor::Rgba(r, g, b, a) => {
                write!(f, "rgb({},{},{},{:?})", r, g, b, a)
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PuppetUnit {
    Pixels(u16),
    Percentage(u8),
}

impl PuppetUnit {
    pub fn to_html_units(&self) -> String {
        match self {
            PuppetUnit::Pixels(units) => {
                if *units == 0_u16 {
                    "auto".into()
                } else {
                    // FIXME move format_args to own function for reusability
                    format!("{}", format_args!("{:?}{}", units, PIXELS_SYMBOL))
                }
            }
            PuppetUnit::Percentage(units) => {
                if *units == 0_u8 {
                    "auto".into()
                } else {
                    // FIXME move format_args to own function for reusability
                    format!("{}", format_args!("{:?}{}", units, PERCENTAGE_SYMBOL))
                }
            }
        }
    }
}

impl Display for PuppetUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pixels(units) => Ok(write!(f, "{}{}", units, PIXELS_SYMBOL)?),
            Self::Percentage(units) => Ok(write!(f, "{}{}", units, PERCENTAGE_SYMBOL)?),
        }
    }
}

impl Debug for PuppetUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pixels(units) => Ok(write!(f, "PuppetUnit::Pixels({})", units)?),
            Self::Percentage(units) => Ok(write!(f, "PuppetUnit::Percentage({})", units)?),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PuppetLength {
    ViewPortWidth,
    ViewPortHeight,
    Full,
    ThreeQuarter,
    Half,
    Quarter,
    Eighth,
    /// Fill the width depending on the width of the elements
    Normal,
}

impl PuppetLength {
    pub fn to_html_units(&self) -> &'static str {
        match self {
            Self::ViewPortWidth => "100vw",
            Self::ViewPortHeight => "100vh",
            Self::Full => "100%",
            Self::ThreeQuarter => "75%",
            Self::Half => "50%",
            Self::Quarter => "25%",
            Self::Eighth => "12.5%",
            Self::Normal => "auto",
        }
    }
}

impl Debug for PuppetLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Self::ViewPortWidth => "PuppetLength::ViewPortWidth",
                Self::ViewPortHeight => "PuppetLength::ViewPortHeight",
                Self::Full => "PuppetLength::Full",
                Self::ThreeQuarter => "PuppetLength::ThreeQuarter",
                Self::Half => "PuppetLength::Half",
                Self::Quarter => "PuppetLength::Quarter",
                Self::Eighth => "PuppetLength::Eighth",
                Self::Normal => "PuppetLength::Normal",
            }
        )
    }
}
impl Display for PuppetLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ViewPortWidth => "ViewPortWidth",
                Self::ViewPortHeight => "ViewPortHeight",
                Self::Full => "Full",
                Self::ThreeQuarter => "ThreeQuarter",
                Self::Half => "Half",
                Self::Quarter => "Quarter",
                Self::Eighth => "Eighth",
                Self::Normal => "Normal",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PuppetAlignment {
    Flex {
        flex_direction: FlexDirection,
        flex_wrap: FlexWrap,
        justify_content: JustifyContent,
        align_self: AlignSelf,
        align_items: AlignItems,
        align_content: AlignContent,
        gap: FlexGap,
    },
    Grid,
}

impl Default for PuppetAlignment {
    fn default() -> Self {
        PuppetAlignment::Flex {
            flex_direction: FlexDirection::default(),
            flex_wrap: FlexWrap::default(),
            justify_content: JustifyContent::default(),
            align_self: AlignSelf::default(),
            align_items: AlignItems::default(),
            align_content: AlignContent::default(),
            gap: FlexGap::default(),
        }
    }
}
