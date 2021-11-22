use crate::PuppetUnit;
use core::{
    fmt,
    fmt::{Debug, Display},
    format_args,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        FlexDirection::Row
    }
}

impl Debug for FlexDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                FlexDirection::Row => "FlexDirection::Row",
                FlexDirection::RowReverse => "FlexDirection::RowReverse",
                FlexDirection::Column => "FlexDirection::Column",
                FlexDirection::ColumnReverse => "FlexDirection::ColumnReverse",
            }
        )
    }
}

impl Display for FlexDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                FlexDirection::Row => "Row",
                FlexDirection::RowReverse => "RowReverse",
                FlexDirection::Column => "Column",
                FlexDirection::ColumnReverse => "ColumnReverse",
            }
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FlexWrap {
    Wrap,
    NoWrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> Self {
        FlexWrap::Wrap
    }
}

impl Debug for FlexWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                FlexWrap::Wrap => "FlexWrap::Wrap",
                FlexWrap::NoWrap => "FlexWrap::NoWrap",
                FlexWrap::WrapReverse => "FlexWrap::WrapReverse",
            }
        )
    }
}

impl Display for FlexWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                FlexWrap::Wrap => "Wrap",
                FlexWrap::NoWrap => "NoWrap",
                FlexWrap::WrapReverse => "WrapReverse",
            }
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> Self {
        JustifyContent::Center
    }
}

impl Debug for JustifyContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                JustifyContent::FlexStart => "JustifyContent::FlexStart",
                JustifyContent::FlexEnd => "JustifyContent::FlexEnd",
                JustifyContent::Center => "JustifyContent::Center",
                JustifyContent::SpaceBetween => "JustifyContent::SpaceBetween",
                JustifyContent::SpaceAround => "JustifyContent::SpaceAround",
                JustifyContent::SpaceEvenly => "JustifyContent::SpaceEvenly",
            }
        )
    }
}

impl Display for JustifyContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                JustifyContent::FlexStart => "FlexStart",
                JustifyContent::FlexEnd => "FlexEnd",
                JustifyContent::Center => "Center",
                JustifyContent::SpaceBetween => "SpaceBetween",
                JustifyContent::SpaceAround => "SpaceAround",
                JustifyContent::SpaceEvenly => "SpaceEvenly",
            }
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Default for AlignSelf {
    fn default() -> Self {
        AlignSelf::Center
    }
}

impl Debug for AlignSelf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                AlignSelf::Auto => "AlignSelf::Auto",
                AlignSelf::FlexStart => "AlignSelf::FlexStart",
                AlignSelf::FlexEnd => "AlignSelf::FlexEnd",
                AlignSelf::Center => "AlignSelf::Center",
                AlignSelf::Baseline => "AlignSelf::Baseline",
                AlignSelf::Stretch => "AlignSelf::Stretch",
            }
        )
    }
}

impl Display for AlignSelf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                AlignSelf::Auto => "Auto",
                AlignSelf::FlexStart => "FlexStart",
                AlignSelf::FlexEnd => "FlexEnd",
                AlignSelf::Center => "Center",
                AlignSelf::Baseline => "Baseline",
                AlignSelf::Stretch => "Stretch",
            }
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Default for AlignItems {
    fn default() -> Self {
        AlignItems::Center
    }
}

impl Debug for AlignItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                AlignItems::FlexStart => "AlignItems::FlexStart",
                AlignItems::FlexEnd => "AlignItems::FlexEnd",
                AlignItems::Center => "AlignItems::Center",
                AlignItems::Baseline => "AlignItems::Baseline",
                AlignItems::Stretch => "AlignItems::Stretch",
            }
        )
    }
}

impl Display for AlignItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                AlignItems::FlexStart => "FlexStart",
                AlignItems::FlexEnd => "FlexEnd",
                AlignItems::Center => "Center",
                AlignItems::Baseline => "Baseline",
                AlignItems::Stretch => "Stretch",
            }
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

impl Default for AlignContent {
    fn default() -> Self {
        AlignContent::Center
    }
}

impl Debug for AlignContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                AlignContent::FlexStart => "AlignContent::FlexStart",
                AlignContent::FlexEnd => "AlignContent::FlexEnd",
                AlignContent::Center => "AlignContent::Center",
                AlignContent::Stretch => "AlignContent::Stretch",
                AlignContent::SpaceBetween => "AlignContent::SpaceBetween",
                AlignContent::SpaceAround => "AlignContent::SpaceAround",
            }
        )
    }
}

impl Display for AlignContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                AlignContent::FlexStart => "FlexStart",
                AlignContent::FlexEnd => "FlexEnd",
                AlignContent::Center => "Center",
                AlignContent::Stretch => "Stretch",
                AlignContent::SpaceBetween => "SpaceBetween",
                AlignContent::SpaceAround => "SpaceAround",
            }
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FlexGap {
    Column(PuppetUnit),
    Row(PuppetUnit),
}

impl Default for FlexGap {
    fn default() -> Self {
        FlexGap::Row(PuppetUnit::Pixels(2_u16))
    }
}

impl Debug for FlexGap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexGap::Column(gap) => write!(f, "{:?}", format_args!("FlexGap::Column({:?})", gap)),
            FlexGap::Row(gap) => write!(f, "{:?}", format_args!("FlexGap::Row({:?})", gap)),
        }
    }
}

impl Display for FlexGap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexGap::Column(gap) => write!(f, "{:?}", format_args!("Column({:?})", gap)),
            FlexGap::Row(gap) => write!(f, "{:?}", format_args!("Row({:?})", gap)),
        }
    }
}
