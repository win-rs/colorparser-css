use colorparser_css::Color;

fn main() {
    let test_case = [
        "rgb(137, 180, 250)",
        "rgba(137, 180, 250, 1)",
        "hsla(217.16815, 91.869934%, 75.882355%, 1)",
        "blue",
        "mauve",
        "gradient(blue, mauve, to right)",
    ];

    for s in test_case {
        let a = Color::from_html_with_theme(s, "./examples/theme.json").unwrap();

        if let Ok(solid) = a.to_solid() {
            let rgba = solid.to_rgba();

            println!("RGBA: {rgba}");
        } else if let Ok(gradient) = a.to_gradient() {
            println!("{gradient}");
        }
    }
}
