#[derive(Debug, Clone)]
pub enum StyleTransformation {
    Blink,
    Bold,
    Italic,
    Underlined,
}

#[derive(Debug, Clone)]
pub enum StyleColor {
    Black,
    White,
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
pub struct StyleProperties {
    pub transformation: Vec<StyleTransformation>,
    pub color: Option<StyleColor>,
    pub background: Option<StyleColor>,
}


pub mod stylesheet {
    use std::collections::HashMap;

    extern crate console;
    use self::console::Style;

    pub fn new<'a>() -> HashMap<&'a str, Style> {
        let mut hash: HashMap<&str, Style> = HashMap::new();
        hash.insert("default", Style::new());
        hash
    }

    pub fn add_style(
            sheet: &mut HashMap<&str, Style>,
            style_name: &'static str,
            style_definition: super::StyleProperties
        ) {
        let mut style: Style = Style::new();
        // apply specified transformation
        for s in &style_definition.transformation {
            match s {
                super::StyleTransformation::Blink => style = style.blink(),
                super::StyleTransformation::Bold => style = style.bold(),
                super::StyleTransformation::Italic => style = style.italic(),
                super::StyleTransformation::Underlined => style = style.underlined(),
            }
        }
        if style_definition.color.is_some() {
            let color = &style_definition.color.unwrap();
            match color {
                super::StyleColor::Black => style = style.black(),
                super::StyleColor::White => style = style.white(),
                super::StyleColor::Red => style = style.red(),
                super::StyleColor::Green => style = style.green(),
                super::StyleColor::Blue => style = style.blue(),
            }
        }
        if style_definition.background.is_some() {
            let color = &style_definition.background.unwrap();
            match color {
                super::StyleColor::Black => style = style.on_black(),
                super::StyleColor::White => style = style.on_white(),
                super::StyleColor::Red => style = style.on_red(),
                super::StyleColor::Green => style = style.on_green(),
                super::StyleColor::Blue => style = style.on_blue(),
            }
        }
        //println!("{:?}", style);

        sheet.insert(style_name, style);
    }

    pub fn println(
            message: &str,
            sheet: HashMap<&str, Style>,
            style_name: &str
        ) {
        let style = &sheet.get(style_name).unwrap_or_else(|| sheet.get("default").unwrap());
        println!("{}", style.apply_to(message));
    }
}
