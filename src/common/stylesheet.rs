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
    Black,
    White,
    Red,
    Green,
    Blue,
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
    /// let mut sheet = stylesheet::new();
    /// sheet::add_style(&mut sheet, "danger", StyleProperties {
    ///     transformation: [StyleTransformation::Bold, StyleTransformation::Blink].to_vec(), color: Some(StyleColor::Red), background: Some(StyleColor::White)
    /// });;
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
                super::StyleColor::Black => style = style.black(),
                super::StyleColor::White => style = style.white(),
                super::StyleColor::Red => style = style.red(),
                super::StyleColor::Green => style = style.green(),
                super::StyleColor::Blue => style = style.blue(),
            }
        }
        // apply specified background color, unless it is None
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

    /// Print a line (string reference), applying to it a single style.
    ///
    /// # Arguments
    ///
    /// * `message` - The string reference to print.
    /// * `sheet` - A reference to a previously created stylesheet.
    /// * `style_name` - The name of the style to use (&str).
    ///
    /// # Example
    ///
    /// ```
    /// let mut sheet = stylesheet::new();
    /// sheet::add_style(&mut sheet, "danger", StyleProperties {
    ///     transformation: [StyleTransformation::Bold, StyleTransformation::Blink].to_vec(), color: Some(StyleColor::Red), background: Some(StyleColor::White)
    /// });;
    /// ```
    pub fn println(
            message: &str,
            sheet: HashMap<&str, Style>,
            style_name: &str
        ) {
        let style = &sheet.get(style_name).unwrap_or_else(|| sheet.get("default").unwrap());
        println!("{}", style.apply_to(message));
    }
}
