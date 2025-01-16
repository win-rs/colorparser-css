use colorparser_css::Theme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = r###"
    {
        "blue": "#89b4fa",
        "text": {
            "white": "#cdd6f4",
            "dark": {
                "grey": "#313244"
            }
        },
        "red": "#f38ba8"
    }
    "###;

    let theme = Theme::parse_theme(json_data)?;
    if let Some(value) = theme.get_color("blue") {
        println!("blue = {}", value);
    } else {
        println!("Key not found: blue");
    }

    if let Some(value) = theme.get_color("red") {
        println!("red = {}", value);
    } else {
        println!("Key not found: red");
    }

    if let Some(value) = theme.get_color("text.white") {
        println!("text.white = {}", value);
    } else {
        println!("Key not found: text.white");
    }

    Ok(())
}
