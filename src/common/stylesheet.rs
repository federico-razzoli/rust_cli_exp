pub mod stylesheet {
    use std::collections::HashMap;

    extern crate console;
    use self::console::Style;

    pub fn new<'a>() -> HashMap<&'a str, Style> {
        HashMap::new()
    }

    pub fn add_style(
            sheet: &mut HashMap<&str, Style>,
            style_name: &'static str,
            style_value: Style
        ) {
        sheet.insert(style_name, style_value);
    }

    pub fn get_style<'a>(
            sheet: HashMap<&str, Style>,
            style_name: &str
        ) -> Style {
        sheet.get(style_name).unwrap().clone()
    }
}
