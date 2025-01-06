use colorparser_css::{Color, ColorspaceImpl};

fn main() {
    let test_case = [
        "rgb(137, 180, 250)",
        "#89B4FA",
        "rgba(137, 180, 250, 1)",
        "hsla(217.16815, 91.869934%, 75.882355%, 1)",
        "gradient(rgb(137, 180, 250), rgb(203, 166, 247))",
    ];

    let other_test = "darken(rgb(137, 180, 250), 5%)";

    println!(
        "{}",
        Color::from_html(other_test)
            .unwrap()
            .to_solid()
            .unwrap()
            .to_rgba()
    );

    for s in test_case {
        let a = Color::from_html(s).unwrap();

        if let Ok(solid) = a.to_solid() {
            let rgba = solid.to_rgba();
            let darken_rgba = rgba.darken(5.0);
            let darken_solid = solid.darken(5.0).to_rgba();

            println!("RGBA: {rgba}");
            println!("Darken RGBA: {darken_rgba}");
            println!("Darken Solid RGBA: {darken_solid}");
        } else if let Ok(gradient) = a.to_gradient() {
            println!("{gradient}");
        }
    }
}
