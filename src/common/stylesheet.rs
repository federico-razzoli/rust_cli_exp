use std::collections::HashMap;

extern crate console;
use self::console::Style;


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


/// Stylesheet struct creates and handles a stylesheet.
/// A stylesheet is a library of named styles that can be applied to texts.
/// A Stylesheet style will play in your code the same role that a named
/// CSS style plays in an HTML document.
pub struct Stylesheet {
    styles: HashMap<&'static str, Style>,
    is_frozen: bool,
}

impl Stylesheet {
    const DEFAULT_STYLE: &'static str = "_default";


    /// Return a new stylesheet. It only contains DEFAULT_STYLE,
    /// that can be used explicitally and is used implicitally when
    /// we try to use a non-existing style.
    pub fn new() -> Stylesheet {
        let mut hash: HashMap<&str, Style> = HashMap::new();
        hash.insert(Stylesheet::DEFAULT_STYLE, Style::new());

        Stylesheet {
            styles: hash,
            is_frozen: false,
        }
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.styles.len()
    }

    fn contains(&self, style_name: &str) -> bool {
        self.styles.contains_key(style_name)
    }

    /// Add a style to an existing stylesheet.
    ///
    /// # Arguments
    ///
    /// * `style_name` - The name of the style to be added to the stylesheet (&str).
    /// * `style_definition` - The definition of the style to be added to the stylesheet (StyleProperties).
    ///
    /// # Example
    ///
    /// ```
    /// use common::stylesheet::*;
    /// use common::stylesheet::StyleColor::*;
    /// use common::stylesheet::StyleTransformation::*;
    /// let mut sheet = Stylesheet::new();
    /// sheet.add_style("danger", StyleProperties {
    ///     transformation: [Bold, Blink,].to_vec(), color: Some(Red), background: Some(White)
    /// });
    /// ```
    pub fn add_style(
            &mut self,
            style_name: &'static str,
            style_definition: StyleProperties
        ) {
        if self.is_frozen {
            panic!("FATAL: Trying to add a style to a frozen Stylesheet");
        }

        // style is a handler from console::Style.
        // Based on the contents of style_definition call style functions
        // to create a proper style.
        let mut style: Style = Style::new();
        // apply all specified transformations, if any
        for s in &style_definition.transformation {
            match s {
                StyleTransformation::Blink => style = style.blink(),
                StyleTransformation::Bold => style = style.bold(),
                StyleTransformation::Italic => style = style.italic(),
                StyleTransformation::Underlined => style = style.underlined(),
            }
        }
        // apply specified text color, unless it is None
        if style_definition.color.is_some() {
            let color = &style_definition.color.unwrap();
            match color {
                StyleColor::DefaultColor => (),
                StyleColor::Black => style = style.black(),
                StyleColor::White => style = style.white(),
                StyleColor::Red => style = style.red(),
                StyleColor::Green => style = style.green(),
                StyleColor::Blue => style = style.blue(),
                StyleColor::Cyan => style = style.cyan(),
                StyleColor::Magenta => style = style.magenta(),
                StyleColor::Yellow => style = style.yellow(),
            }
        }
        // apply specified background color, unless it is None
        if style_definition.background.is_some() {
            let color = &style_definition.background.unwrap();
            match color {
                StyleColor::DefaultColor => (),
                StyleColor::Black => style = style.on_black(),
                StyleColor::White => style = style.on_white(),
                StyleColor::Red => style = style.on_red(),
                StyleColor::Green => style = style.on_green(),
                StyleColor::Blue => style = style.on_blue(),
                StyleColor::Cyan => style = style.on_cyan(),
                StyleColor::Magenta => style = style.on_magenta(),
                StyleColor::Yellow => style = style.on_yellow(),
            }
        }
        //println!("{:?}", style);

        self.styles.insert(style_name, style);
    }

    /// Freeze the Stylesheet. It will not be possible to modify it again.
    /// However, trying to freeze it will not cause any complain.
    ///
    /// # Example
    ///
    /// ```
    /// use common::stylesheet::*;
    /// use common::stylesheet::StyleColor::*;
    /// use common::stylesheet::StyleTransformation::*;
    /// let mut sheet = Stylesheet::new();
    /// sheet.add_style("danger", StyleProperties {
    ///     transformation: [Bold, Blink,].to_vec(), color: Some(Red), background: Some(White)
    /// });
    /// sheet.freeze();
    /// ```
    pub fn freeze(&mut self) {
        self.is_frozen = true;
    }

    /// Print a line (string reference), applying to it a single style.
    ///
    /// # Arguments
    ///
    /// * `style_name` - The name of the style to use (&str).
    /// * `message` - The string struct or string reference to print.
    ///
    /// # Example
    ///
    /// ```
    /// use common::stylesheet::*;
    /// use common::stylesheet::StyleColor::*;
    /// use common::stylesheet::StyleTransformation::*;
    /// let mut sheet = Stylesheet::new();
    /// sheet.add_style("danger", StyleProperties {
    ///     transformation: [Bold, Blink,].to_vec(), color: Some(Red), background: Some(White)
    /// });
    /// sheet.println("danger", "Some text");
    /// ```
    pub fn println<S>(
            &self,
            mut style_name: &str,
            message: S
        ) where S: AsRef<str> {
        // if the requested style doesn't exist we fall back to default
        if !self.contains(style_name) {
            style_name = Stylesheet::DEFAULT_STYLE;
        }
        let style = self.styles.get(style_name).unwrap();
        println!("{}", style.apply_to(message.as_ref()));
    }

    /// Similar to println(), but print() doesn't append a newline character.
    ///
    /// # Arguments
    ///
    /// * `style_name` - The name of the style to use (&str).
    /// * `message` - The string struct or string reference to print.
    ///
    /// # Example
    ///
    /// ```
    /// use common::stylesheet::*;
    /// use common::stylesheet::StyleColor::*;
    /// use common::stylesheet::StyleTransformation::*;
    /// let mut sheet = Stylesheet::new();
    /// sheet.add_style("danger", StyleProperties {
    ///     transformation: [Bold, Blink,].to_vec(), color: Some(Red), background: Some(White)
    /// });
    /// sheet.print("danger", "Some text");
    /// ```
    pub fn print<S>(
            &self,
            mut style_name: &str,
            message: S
        ) where S: AsRef<str> {
        // if the requested style doesn't exist we fall back to default
        if !self.contains(style_name) {
            style_name = Stylesheet::DEFAULT_STYLE;
        }
        let style = self.styles.get(style_name).unwrap();
        println!("{}", style.apply_to(message.as_ref()));
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::StyleColor::*;
    use super::StyleTransformation::*;


    #[test]
    fn create_stylesheet() {
        let sheet: Stylesheet = Stylesheet::new();
        // must be a HashMap of Style's and only contain DEFAULT_STYLE key
        assert!(sheet.contains(Stylesheet::DEFAULT_STYLE));
        assert_eq!(sheet.len(), 1);
    }

    #[test]
    fn add_empty_style() {
        let mut sheet = Stylesheet::new();
        // must be able to retrieve an empty style and retrieve it
        // there are more ways to achieve this

        let style_name = "empty_guy";
        sheet.add_style(style_name,
            StyleProperties { transformation: [].to_vec(), color: None, background: None, }
        );
        assert!(sheet.contains(style_name));

        let style_name = "empty_lady";
        sheet.add_style(style_name,
            StyleProperties { transformation: [].to_vec(), color: Some(DefaultColor), background: Some(DefaultColor), }
        );
        assert!(sheet.contains(style_name));
    }

    #[test]
    fn add_style() {
        let mut sheet = Stylesheet::new();
        // must be able to retrieve a regular style and retrieve it
        let style_name = "new_guy";
        sheet.add_style(
            style_name,
            StyleProperties {
                transformation: [ Bold, Underlined, ].to_vec(),
                color: Some(Blue),
                background: Some(White),
            }
        );
        assert!(sheet.contains(style_name));
    }

    #[test]
    fn freeze() {
        let mut sheet = Stylesheet::new();
        // should be able to freeze (become immutable)
        sheet.freeze();
        // did not panick
        assert!(true);
    }

    #[test]
    fn freeze_twice() {
        let mut sheet = Stylesheet::new();
        // should be able to freeze (become immutable)
        sheet.freeze();
        sheet.freeze();
        // did not panick
        assert!(true);
    }

    #[test]
    #[should_panic(expected = "FATAL:")]
    fn freeze_and_try_to_modify() {
        let mut sheet = Stylesheet::new();
        sheet.freeze();
        sheet.add_style(
            "test",
            StyleProperties { transformation: [].to_vec(), color: None, background: None, }
        );
    }

    #[test]
    fn println() {
        let sheet = Stylesheet::new();
        // must be able to println a &str and a String without panicking
        sheet.println(Stylesheet::DEFAULT_STYLE, "Just a test String reference");
        sheet.println(Stylesheet::DEFAULT_STYLE, "A cool String struct");
        // did not panick
        assert!(true);
    }

    #[test]
    fn print() {
        let sheet = Stylesheet::new();
        // must be able to print a &str and a String without panicking
        sheet.print(Stylesheet::DEFAULT_STYLE, "A ");
        sheet.print(Stylesheet::DEFAULT_STYLE, "B");
        // did not panick
        assert!(true);
    }
}
