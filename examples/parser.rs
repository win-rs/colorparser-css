use colorparser_css::parse;

fn main() {
    let test_case = [
        "rgb(137, 180, 250)",
        "#89B4FA",
        "hsl(217, 91.9%, 75.9%)",
        "gradient(rgb(137, 180, 250), #89b4fa, to right)",
    ];

    for s in test_case {
        let a = parse(s).unwrap();
        println!("{a}");

        if let Ok(solid) = a.to_solid() {
            println!("{solid}");
        } else if let Ok(gradient) = a.to_gradient() {
            println!("{gradient}");
        }
    }
}
