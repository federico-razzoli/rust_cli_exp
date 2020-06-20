/// Transformations that can be applied to texts.
#[derive(Debug, Clone)]
pub enum StyleTransformation {
    Blink,
    Bold,
    Italic,
    Underlined,
}

/// Colors that can be used for texts and/or their backgrounds.
#[derive(Debug, Clone)]
pub enum StyleColor {
    DefaultColor,
    Black,
    White,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
}

/// All properties that form a style.
#[derive(Debug, Clone)]
pub struct StyleProperties {
    /// A vector of all the transformations to apply to texts.
    /// Can be empty. Remember that it's a vector, not an array.
    ///
    /// # Examples
    /// [].to_vec()
    /// [StyleTransformation.Bold, StyleTransformation.Underlined].to_vec()
    pub transformation: Vec<StyleTransformation>,

    /// The color to apply to the text. The type is Option<StyleColor>.
    /// None means default.
    pub color: Option<StyleColor>,

    /// The color to apply to the text background. The type is Option<StyleColor>.
    /// None means default.
    pub background: Option<StyleColor>,
}


/// stylesheet module creates and handles a stylesheet.
/// A stylesheet is a library of named styles that can be applied to texts.
pub mod stylesheet {
    use std::collections::HashMap;

    extern crate console;
    use self::console::Style;

    /// Return a new stylesheet. It only contains the "default" style,
    /// that can be used explicitally and is used implicitally when
    /// we try to use a non-existing style.
    pub fn new<'a>() -> HashMap<&'a str, Style> {
        let mut hash: HashMap<&str, Style> = HashMap::new();
        hash.insert("default", Style::new());
        hash
    }

    /// Add a style to an existing stylesheet.
    ///
    /// # Arguments
    ///
    /// * `sheet` - A mutable reference to a previously created stylesheet.
    /// * `style_name` - The name of the style to be added to the stylesheet (&str).
    /// * `style_definition` - The definition of the style to be added to the stylesheet (StyleProperties).
    ///
    /// # Example
    ///
    /// ```
    /// use common::stylesheet::*;
    /// use common::stylesheet::StyleColor::*;
    /// use common::stylesheet::StyleTransformation::*;
    /// let mut sheet = stylesheet::new();
    /// stylesheet::add_style(&mut sheet, "danger", StyleProperties {
    ///     transformation: [Bold, Blink,].to_vec(), color: Some(Red), background: Some(White)
    /// });
    /// ```
    pub fn add_style(
            sheet: &mut HashMap<&str, Style>,
            style_name: &'static str,
            style_definition: super::StyleProperties
        ) {
        // style is a handler from console::Style.
        // Based on the contents of style_definition call style functions
        // to create a proper style.
        let mut style: Style = Style::new();
        // apply all specified transformations, if any
        for s in &style_definition.transformation {
            match s {
                super::StyleTransformation::Blink => style = style.blink(),
                super::StyleTransformation::Bold => style = style.bold(),
                super::StyleTransformation::Italic => style = style.italic(),
                super::StyleTransformation::Underlined => style = style.underlined(),
            }
        }
        // apply specified text color, unless it is None
        if style_definition.color.is_some() {
            let color = &style_definition.color.unwrap();
            match color {
                super::StyleColor::DefaultColor => (),
                super::StyleColor::Black => style = style.black(),
                super::StyleColor::White => style = style.white(),
                super::StyleColor::Red => style = style.red(),
                super::StyleColor::Green => style = style.green(),
                super::StyleColor::Blue => style = style.blue(),
                super::StyleColor::Cyan => style = style.cyan(),
                super::StyleColor::Magenta => style = style.magenta(),
                super::StyleColor::Yellow => style = style.yellow(),
            }
        }
        // apply specified background color, unless it is None
        if style_definition.background.is_some() {
            let color = &style_definition.background.unwrap();
            match color {
                super::StyleColor::DefaultColor => (),
                super::StyleColor::Black => style = style.on_black(),
                super::StyleColor::White => style = style.on_white(),
                super::StyleColor::Red => style = style.on_red(),
                super::StyleColor::Green => style = style.on_green(),
                super::StyleColor::Blue => style = style.on_blue(),
                super::StyleColor::Cyan => style = style.on_cyan(),
                super::StyleColor::Magenta => style = style.on_magenta(),
                super::StyleColor::Yellow => style = style.on_yellow(),
            }
        }
        //println!("{:?}", style);

        sheet.insert(style_name, style);
    }

    /// Print a line (string reference), applying to it a single style.
    ///
    /// # Arguments
    ///
    /// * `message` - The string struct or string reference to print.
    /// * `sheet` - A reference to a previously created stylesheet.
    /// * `style_name` - The name of the style to use (&str).
    ///
    /// # Example
    ///
    /// ```
    /// use common::stylesheet::*;
    /// use common::stylesheet::StyleColor::*;
    /// use common::stylesheet::StyleTransformation::*;
    /// let mut sheet = stylesheet::new();
    /// stylesheet::add_style(&mut sheet, "danger", StyleProperties {
    ///     transformation: [Bold, Blink,].to_vec(), color: Some(Red), background: Some(White)
    /// });
    /// stylesheet::println("Some text", &sheet, "danger");
    /// ```
    pub fn println<S>(
            message: S,
            sheet: &HashMap<&str, Style>,
            style_name: &str
        ) where S: AsRef<str> {
        let style = &sheet.get(style_name).unwrap_or_else(|| sheet.get("default").unwrap());
        println!("{}", style.apply_to(message.as_ref()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::StyleColor::*;
    use super::StyleTransformation::*;
    use std::collections::HashMap;

    extern crate console;
    use self::console::Style;

    #[test]
    fn create_stylesheet() {
        let sheet: HashMap<&str, Style> = stylesheet::new();
        // must be a HashMap of Style's and only contain a "default" key
        assert_eq!(sheet.len(), 1);
        assert!(sheet.get("default").is_some());
    }

    #[test]
    fn add_empty_style() {
        let mut sheet = stylesheet::new();
        // must be able to retrieve an empty style and retrieve it
        // there are more ways to achieve this

        let style_name = "empty_guy";
        stylesheet::add_style(&mut sheet, style_name,
            StyleProperties { transformation: [].to_vec(), color: None, background: None, }
        );
        assert!(sheet.get(style_name).is_some());

        let style_name = "empty_lady";
        stylesheet::add_style(&mut sheet, style_name,
            StyleProperties { transformation: [].to_vec(), color: Some(DefaultColor), background: Some(DefaultColor), }
        );
        assert!(sheet.get(style_name).is_some());
    }

    #[test]
    fn add_style() {
        let mut sheet = stylesheet::new();
        // must be able to retrieve a regular style and retrieve it
        let style_name = "empty_guy";
        stylesheet::add_style(&mut sheet, style_name,
            StyleProperties {
                transformation: [ Bold, Underlined, ].to_vec(),
                color: Some(Blue),
                background: Some(White),
            }
        );
        assert!(sheet.get(style_name).is_some());
    }

    #[test]
    fn println() {
        let sheet = stylesheet::new();
        // must be able to println a &str and a String without panicking
        stylesheet::println("Just a test String reference", &sheet, "default");
        stylesheet::println("A cool String struct", &sheet, "default");
        // did not panick
        assert!(true);
    }
}
