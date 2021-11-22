pub struct Component {
    component_type: ComponentType,
    style: PuppetStyle,
    // Fot images, text is used for information
    text: &'static str,
}

pub enum ComponentType {
    Button,
    Paragraph,
    Heading,
    Image,
    Link,
}
