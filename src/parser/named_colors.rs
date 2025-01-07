// https://www.w3.org/TR/css-color-4/#named-colors

use std::sync::LazyLock;

use regex::Regex;
use rustc_hash::FxHashMap;

const HEX_PATTERN: &str = r"#[0-9A-F]{3,8}";
const RGBA_PATTERN: &str =
    r"rgba?\([0-9]{1,3},\s*[0-9]{1,3},\s*[0-9]{1,3}(?:,\s*[0-9]*(?:\.[0-9]+)?)?\)";
const ACCENT_TRANSPARENT_PATTERN: &str = r"(accent|transparent|accent_inactive)";
const DARKEN_LIGHTEN_PATTERN: &str = r"(?:darken|lighten)\(\s*(?:#[0-9A-F]{3,8}|rgba?\([0-9]{1,3},\s*[0-9]{1,3},\s*[0-9]{1,3}(?:,\s*[0-9]*(?:\.[0-9]+)?)?\)|(?:aliceblue|antiquewhite|aqua|aquamarine|azure|beige|bisque|black|blanchedalmond|blue|blueviolet|brown|burlywood|cadetblue|chartreuse|chocolate|coral|cornflowerblue|cornsilk|crimson|cyan|darkblue|darkcyan|darkgoldenrod|darkgray|darkgreen|darkgrey|darkkhaki|darkmagenta|darkolivegreen|darkorange|darkorchid|darkred|darksalmon|darkseagreen|darkslateblue|darkslategray|darkslategrey|darkturquoise|darkviolet|deeppink|deepskyblue|dimgray|dimgrey|dodgerblue|firebrick|floralwhite|forestgreen|fuchsia|gainsboro|ghostwhite|gold|goldenrod|gray|green|greenyellow|grey|honeydew|hotpink|indianred|indigo|ivory|khaki|lavender|lavenderblush|lawngreen|lemonchiffon|lightblue|lightcoral|lightcyan|lightgoldenrodyellow|lightgray|lightgreen|lightgrey|lightpink|lightsalmon|lightseagreen|lightskyblue|lightslategray|lightslategrey|lightsteelblue|lightyellow|lime|limegreen|linen|magenta|maroon|mediumaquamarine|mediumblue|mediumorchid|mediumpurple|mediumseagreen|mediumslateblue|mediumspringgreen|mediumturquoise|mediumvioletred|midnightblue|mintcream|mistyrose|moccasin|navajowhite|navy|oldlace|olive|olivedrab|orange|orangered|orchid|palegoldenrod|palegreen|paleturquoise|palevioletred|papayawhip|peachpuff|peru|pink|plum|powderblue|purple|rebeccapurple|red|rosybrown|royalblue|saddlebrown|salmon|sandybrown|seagreen|seashell|sienna|silver|skyblue|slateblue|slategray|slategrey|snow|springgreen|steelblue|tan|teal|thistle|tomato|turquoise|violet|wheat|white|whitesmoke|yellow|yellowgreen))\s*,\s*\d+(?:\.\d+)?%\s*\)";
const NAMED_COLOR_PATTERN: &str = r"\b(?:aliceblue|antiquewhite|aqua|aquamarine|azure|beige|bisque|black|blanchedalmond|blue|blueviolet|brown|burlywood|cadetblue|chartreuse|chocolate|coral|cornflowerblue|cornsilk|crimson|cyan|darkblue|darkcyan|darkgoldenrod|darkgray|darkgreen|darkgrey|darkkhaki|darkmagenta|darkolivegreen|darkorange|darkorchid|darkred|darksalmon|darkseagreen|darkslateblue|darkslategray|darkslategrey|darkturquoise|darkviolet|deeppink|deepskyblue|dimgray|dimgrey|dodgerblue|firebrick|floralwhite|forestgreen|fuchsia|gainsboro|ghostwhite|gold|goldenrod|gray|green|greenyellow|grey|honeydew|hotpink|indianred|indigo|ivory|khaki|lavender|lavenderblush|lawngreen|lemonchiffon|lightblue|lightcoral|lightcyan|lightgoldenrodyellow|lightgray|lightgreen|lightgrey|lightpink|lightsalmon|lightseagreen|lightskyblue|lightslategray|lightslategrey|lightsteelblue|lightyellow|lime|limegreen|linen|magenta|maroon|mediumaquamarine|mediumblue|mediumorchid|mediumpurple|mediumseagreen|mediumslateblue|mediumspringgreen|mediumturquoise|mediumvioletred|midnightblue|mintcream|mistyrose|moccasin|navajowhite|navy|oldlace|olive|olivedrab|orange|orangered|orchid|palegoldenrod|palegreen|paleturquoise|palevioletred|papayawhip|peachpuff|peru|pink|plum|powderblue|purple|rebeccapurple|red|rosybrown|royalblue|saddlebrown|salmon|sandybrown|seagreen|seashell|sienna|silver|skyblue|slateblue|slategray|slategrey|snow|springgreen|steelblue|tan|teal|thistle|tomato|turquoise|violet|wheat|white|whitesmoke|yellow|yellowgreen)\b";
const DARKEN_LIGHTEN_FETCH_PATTERN: &str = r"(?i)(darken|lighten)\(\s*(#[0-9A-Fa-f]{3,8}|rgba?\(\s*\d{1,3},\s*\d{1,3},\s*\d{1,3}(?:,\s*(?:1|0(?:\.\d+)?))?\s*\)|(?:aliceblue|antiquewhite|aqua|aquamarine|azure|beige|bisque|black|blanchedalmond|blue|blueviolet|brown|burlywood|cadetblue|chartreuse|chocolate|coral|cornflowerblue|cornsilk|crimson|cyan|darkblue|darkcyan|darkgoldenrod|darkgray|darkgreen|darkgrey|darkkhaki|darkmagenta|darkolivegreen|darkorange|darkorchid|darkred|darksalmon|darkseagreen|darkslateblue|darkslategray|darkslategrey|darkturquoise|darkviolet|deeppink|deepskyblue|dimgray|dimgrey|dodgerblue|firebrick|floralwhite|forestgreen|fuchsia|gainsboro|ghostwhite|gold|goldenrod|gray|green|greenyellow|grey|honeydew|hotpink|indianred|indigo|ivory|khaki|lavender|lavenderblush|lawngreen|lemonchiffon|lightblue|lightcoral|lightcyan|lightgoldenrodyellow|lightgray|lightgreen|lightgrey|lightpink|lightsalmon|lightseagreen|lightskyblue|lightslategray|lightslategrey|lightsteelblue|lightyellow|lime|limegreen|linen|magenta|maroon|mediumaquamarine|mediumblue|mediumorchid|mediumpurple|mediumseagreen|mediumslateblue|mediumspringgreen|mediumturquoise|mediumvioletred|midnightblue|mintcream|mistyrose|moccasin|navajowhite|navy|oldlace|olive|olivedrab|orange|orangered|orchid|palegoldenrod|palegreen|paleturquoise|palevioletred|papayawhip|peachpuff|peru|pink|plum|powderblue|purple|rebeccapurple|red|rosybrown|royalblue|saddlebrown|salmon|sandybrown|seagreen|seashell|sienna|silver|skyblue|slateblue|slategray|slategrey|snow|springgreen|steelblue|tan|teal|thistle|tomato|turquoise|violet|wheat|white|whitesmoke|yellow|yellowgreen))\s*,\s*(\d+(?:\.\d+)?)%\s*\)";

pub static COLOR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        format!(
            r"(?i){}|{}|{}|{}|{}",
            HEX_PATTERN,
            RGBA_PATTERN,
            ACCENT_TRANSPARENT_PATTERN,
            DARKEN_LIGHTEN_PATTERN,
            NAMED_COLOR_PATTERN,
        )
        .as_str(),
    )
    .unwrap()
});

pub static DARKEN_LIGHTEN_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(DARKEN_LIGHTEN_FETCH_PATTERN).unwrap());

#[cfg(feature = "hash-colors")]
pub static NAMED_COLORS_MAP: LazyLock<FxHashMap<&'static str, [u8; 3]>> = LazyLock::new(|| {
    let mut map = FxHashMap::default();
    map.insert("aliceblue", [240, 248, 255]);
    map.insert("antiquewhite", [250, 235, 215]);
    map.insert("aqua", [0, 255, 255]);
    map.insert("aquamarine", [127, 255, 212]);
    map.insert("azure", [240, 255, 255]);
    map.insert("beige", [245, 245, 220]);
    map.insert("bisque", [255, 228, 196]);
    map.insert("black", [0, 0, 0]);
    map.insert("blanchedalmond", [255, 235, 205]);
    map.insert("blue", [0, 0, 255]);
    map.insert("blueviolet", [138, 43, 226]);
    map.insert("brown", [165, 42, 42]);
    map.insert("burlywood", [222, 184, 135]);
    map.insert("cadetblue", [95, 158, 160]);
    map.insert("chartreuse", [127, 255, 0]);
    map.insert("chocolate", [210, 105, 30]);
    map.insert("coral", [255, 127, 80]);
    map.insert("cornflowerblue", [100, 149, 237]);
    map.insert("cornsilk", [255, 248, 220]);
    map.insert("crimson", [220, 20, 60]);
    map.insert("cyan", [0, 255, 255]);
    map.insert("darkblue", [0, 0, 139]);
    map.insert("darkcyan", [0, 139, 139]);
    map.insert("darkgoldenrod", [184, 134, 11]);
    map.insert("darkgray", [169, 169, 169]);
    map.insert("darkgreen", [0, 100, 0]);
    map.insert("darkgrey", [169, 169, 169]);
    map.insert("darkkhaki", [189, 183, 107]);
    map.insert("darkmagenta", [139, 0, 139]);
    map.insert("darkolivegreen", [85, 107, 47]);
    map.insert("darkorange", [255, 140, 0]);
    map.insert("darkorchid", [153, 50, 204]);
    map.insert("darkred", [139, 0, 0]);
    map.insert("darksalmon", [233, 150, 122]);
    map.insert("darkseagreen", [143, 188, 143]);
    map.insert("darkslateblue", [72, 61, 139]);
    map.insert("darkslategray", [47, 79, 79]);
    map.insert("darkslategrey", [47, 79, 79]);
    map.insert("darkturquoise", [0, 206, 209]);
    map.insert("darkviolet", [148, 0, 211]);
    map.insert("deeppink", [255, 20, 147]);
    map.insert("deepskyblue", [0, 191, 255]);
    map.insert("dimgray", [105, 105, 105]);
    map.insert("dimgrey", [105, 105, 105]);
    map.insert("dodgerblue", [30, 144, 255]);
    map.insert("firebrick", [178, 34, 34]);
    map.insert("floralwhite", [255, 250, 240]);
    map.insert("forestgreen", [34, 139, 34]);
    map.insert("fuchsia", [255, 0, 255]);
    map.insert("gainsboro", [220, 220, 220]);
    map.insert("ghostwhite", [248, 248, 255]);
    map.insert("gold", [255, 215, 0]);
    map.insert("goldenrod", [218, 165, 32]);
    map.insert("gray", [128, 128, 128]);
    map.insert("green", [0, 128, 0]);
    map.insert("greenyellow", [173, 255, 47]);
    map.insert("grey", [128, 128, 128]);
    map.insert("honeydew", [240, 255, 240]);
    map.insert("hotpink", [255, 105, 180]);
    map.insert("indianred", [205, 92, 92]);
    map.insert("indigo", [75, 0, 130]);
    map.insert("ivory", [255, 255, 240]);
    map.insert("khaki", [240, 230, 140]);
    map.insert("lavender", [230, 230, 250]);
    map.insert("lavenderblush", [255, 240, 245]);
    map.insert("lawngreen", [124, 252, 0]);
    map.insert("lemonchiffon", [255, 250, 205]);
    map.insert("lightblue", [173, 216, 230]);
    map.insert("lightcoral", [240, 128, 128]);
    map.insert("lightcyan", [224, 255, 255]);
    map.insert("lightgoldenrodyellow", [250, 250, 210]);
    map.insert("lightgray", [211, 211, 211]);
    map.insert("lightgreen", [144, 238, 144]);
    map.insert("lightgrey", [211, 211, 211]);
    map.insert("lightpink", [255, 182, 193]);
    map.insert("lightsalmon", [255, 160, 122]);
    map.insert("lightseagreen", [32, 178, 170]);
    map.insert("lightskyblue", [135, 206, 250]);
    map.insert("lightslategray", [119, 136, 153]);
    map.insert("lightslategrey", [119, 136, 153]);
    map.insert("lightsteelblue", [176, 196, 222]);
    map.insert("lightyellow", [255, 255, 224]);
    map.insert("lime", [0, 255, 0]);
    map.insert("limegreen", [50, 205, 50]);
    map.insert("linen", [250, 240, 230]);
    map.insert("magenta", [255, 0, 255]);
    map.insert("maroon", [128, 0, 0]);
    map.insert("mediumaquamarine", [102, 205, 170]);
    map.insert("mediumblue", [0, 0, 205]);
    map.insert("mediumorchid", [186, 85, 211]);
    map.insert("mediumpurple", [147, 112, 219]);
    map.insert("mediumseagreen", [60, 179, 113]);
    map.insert("mediumslateblue", [123, 104, 238]);
    map.insert("mediumspringgreen", [0, 250, 154]);
    map.insert("mediumturquoise", [72, 209, 204]);
    map.insert("mediumvioletred", [199, 21, 133]);
    map.insert("midnightblue", [25, 25, 112]);
    map.insert("mintcream", [245, 255, 250]);
    map.insert("mistyrose", [255, 228, 225]);
    map.insert("moccasin", [255, 228, 181]);
    map.insert("navajowhite", [255, 222, 173]);
    map.insert("navy", [0, 0, 128]);
    map.insert("oldlace", [253, 245, 230]);
    map.insert("olive", [128, 128, 0]);
    map.insert("olivedrab", [107, 142, 35]);
    map.insert("orange", [255, 165, 0]);
    map.insert("orangered", [255, 69, 0]);
    map.insert("orchid", [218, 112, 214]);
    map.insert("palegoldenrod", [238, 232, 170]);
    map.insert("palegreen", [152, 251, 152]);
    map.insert("paleturquoise", [175, 238, 238]);
    map.insert("palevioletred", [219, 112, 147]);
    map.insert("papayawhip", [255, 239, 213]);
    map.insert("peachpuff", [255, 218, 185]);
    map.insert("peru", [205, 133, 63]);
    map.insert("pink", [255, 192, 203]);
    map.insert("plum", [221, 160, 221]);
    map.insert("powderblue", [176, 224, 230]);
    map.insert("purple", [128, 0, 128]);
    map.insert("rebeccapurple", [102, 51, 153]);
    map.insert("red", [255, 0, 0]);
    map.insert("rosybrown", [188, 143, 143]);
    map.insert("royalblue", [65, 105, 225]);
    map.insert("saddlebrown", [139, 69, 19]);
    map.insert("salmon", [250, 128, 114]);
    map.insert("sandybrown", [244, 164, 96]);
    map.insert("seagreen", [46, 139, 87]);
    map.insert("seashell", [255, 245, 238]);
    map.insert("sienna", [160, 82, 45]);
    map.insert("silver", [192, 192, 192]);
    map.insert("skyblue", [135, 206, 235]);
    map.insert("slateblue", [106, 90, 205]);
    map.insert("slategray", [112, 128, 144]);
    map.insert("slategrey", [112, 128, 144]);
    map.insert("snow", [255, 250, 250]);
    map.insert("springgreen", [0, 255, 127]);
    map.insert("steelblue", [70, 130, 180]);
    map.insert("tan", [210, 180, 140]);
    map.insert("teal", [0, 128, 128]);
    map.insert("thistle", [216, 191, 216]);
    map.insert("tomato", [255, 99, 71]);
    map.insert("turquoise", [64, 224, 208]);
    map.insert("violet", [238, 130, 238]);
    map.insert("wheat", [245, 222, 179]);
    map.insert("white", [255, 255, 255]);
    map.insert("whitesmoke", [245, 245, 245]);
    map.insert("yellow", [255, 255, 0]);
    map.insert("yellowgreen", [154, 205, 50]);

    map
});

#[cfg(feature = "named-colors")]
pub static NAMED_COLORS: phf::Map<&'static str, [u8; 3]> = phf::phf_map! {
    "aliceblue" => [240, 248, 255],
    "antiquewhite" => [250, 235, 215],
    "aqua" => [0, 255, 255],
    "aquamarine" => [127, 255, 212],
    "azure" => [240, 255, 255],
    "beige" => [245, 245, 220],
    "bisque" => [255, 228, 196],
    "black" => [0, 0, 0],
    "blanchedalmond" => [255, 235, 205],
    "blue" => [0, 0, 255],
    "blueviolet" => [138, 43, 226],
    "brown" => [165, 42, 42],
    "burlywood" => [222, 184, 135],
    "cadetblue" => [95, 158, 160],
    "chartreuse" => [127, 255, 0],
    "chocolate" => [210, 105, 30],
    "coral" => [255, 127, 80],
    "cornflowerblue" => [100, 149, 237],
    "cornsilk" => [255, 248, 220],
    "crimson" => [220, 20, 60],
    "cyan" => [0, 255, 255],
    "darkblue" => [0, 0, 139],
    "darkcyan" => [0, 139, 139],
    "darkgoldenrod" => [184, 134, 11],
    "darkgray" => [169, 169, 169],
    "darkgreen" => [0, 100, 0],
    "darkgrey" => [169, 169, 169],
    "darkkhaki" => [189, 183, 107],
    "darkmagenta" => [139, 0, 139],
    "darkolivegreen" => [85, 107, 47],
    "darkorange" => [255, 140, 0],
    "darkorchid" => [153, 50, 204],
    "darkred" => [139, 0, 0],
    "darksalmon" => [233, 150, 122],
    "darkseagreen" => [143, 188, 143],
    "darkslateblue" => [72, 61, 139],
    "darkslategray" => [47, 79, 79],
    "darkslategrey" => [47, 79, 79],
    "darkturquoise" => [0, 206, 209],
    "darkviolet" => [148, 0, 211],
    "deeppink" => [255, 20, 147],
    "deepskyblue" => [0, 191, 255],
    "dimgray" => [105, 105, 105],
    "dimgrey" => [105, 105, 105],
    "dodgerblue" => [30, 144, 255],
    "firebrick" => [178, 34, 34],
    "floralwhite" => [255, 250, 240],
    "forestgreen" => [34, 139, 34],
    "fuchsia" => [255, 0, 255],
    "gainsboro" => [220, 220, 220],
    "ghostwhite" => [248, 248, 255],
    "gold" => [255, 215, 0],
    "goldenrod" => [218, 165, 32],
    "gray" => [128, 128, 128],
    "green" => [0, 128, 0],
    "greenyellow" => [173, 255, 47],
    "grey" => [128, 128, 128],
    "honeydew" => [240, 255, 240],
    "hotpink" => [255, 105, 180],
    "indianred" => [205, 92, 92],
    "indigo" => [75, 0, 130],
    "ivory" => [255, 255, 240],
    "khaki" => [240, 230, 140],
    "lavender" => [230, 230, 250],
    "lavenderblush" => [255, 240, 245],
    "lawngreen" => [124, 252, 0],
    "lemonchiffon" => [255, 250, 205],
    "lightblue" => [173, 216, 230],
    "lightcoral" => [240, 128, 128],
    "lightcyan" => [224, 255, 255],
    "lightgoldenrodyellow" => [250, 250, 210],
    "lightgray" => [211, 211, 211],
    "lightgreen" => [144, 238, 144],
    "lightgrey" => [211, 211, 211],
    "lightpink" => [255, 182, 193],
    "lightsalmon" => [255, 160, 122],
    "lightseagreen" => [32, 178, 170],
    "lightskyblue" => [135, 206, 250],
    "lightslategray" => [119, 136, 153],
    "lightslategrey" => [119, 136, 153],
    "lightsteelblue" => [176, 196, 222],
    "lightyellow" => [255, 255, 224],
    "lime" => [0, 255, 0],
    "limegreen" => [50, 205, 50],
    "linen" => [250, 240, 230],
    "magenta" => [255, 0, 255],
    "maroon" => [128, 0, 0],
    "mediumaquamarine" => [102, 205, 170],
    "mediumblue" => [0, 0, 205],
    "mediumorchid" => [186, 85, 211],
    "mediumpurple" => [147, 112, 219],
    "mediumseagreen" => [60, 179, 113],
    "mediumslateblue" => [123, 104, 238],
    "mediumspringgreen" => [0, 250, 154],
    "mediumturquoise" => [72, 209, 204],
    "mediumvioletred" => [199, 21, 133],
    "midnightblue" => [25, 25, 112],
    "mintcream" => [245, 255, 250],
    "mistyrose" => [255, 228, 225],
    "moccasin" => [255, 228, 181],
    "navajowhite" => [255, 222, 173],
    "navy" => [0, 0, 128],
    "oldlace" => [253, 245, 230],
    "olive" => [128, 128, 0],
    "olivedrab" => [107, 142, 35],
    "orange" => [255, 165, 0],
    "orangered" => [255, 69, 0],
    "orchid" => [218, 112, 214],
    "palegoldenrod" => [238, 232, 170],
    "palegreen" => [152, 251, 152],
    "paleturquoise" => [175, 238, 238],
    "palevioletred" => [219, 112, 147],
    "papayawhip" => [255, 239, 213],
    "peachpuff" => [255, 218, 185],
    "peru" => [205, 133, 63],
    "pink" => [255, 192, 203],
    "plum" => [221, 160, 221],
    "powderblue" => [176, 224, 230],
    "purple" => [128, 0, 128],
    "rebeccapurple" => [102, 51, 153],
    "red" => [255, 0, 0],
    "rosybrown" => [188, 143, 143],
    "royalblue" => [65, 105, 225],
    "saddlebrown" => [139, 69, 19],
    "salmon" => [250, 128, 114],
    "sandybrown" => [244, 164, 96],
    "seagreen" => [46, 139, 87],
    "seashell" => [255, 245, 238],
    "sienna" => [160, 82, 45],
    "silver" => [192, 192, 192],
    "skyblue" => [135, 206, 235],
    "slateblue" => [106, 90, 205],
    "slategray" => [112, 128, 144],
    "slategrey" => [112, 128, 144],
    "snow" => [255, 250, 250],
    "springgreen" => [0, 255, 127],
    "steelblue" => [70, 130, 180],
    "tan" => [210, 180, 140],
    "teal" => [0, 128, 128],
    "thistle" => [216, 191, 216],
    "tomato" => [255, 99, 71],
    "turquoise" => [64, 224, 208],
    "violet" => [238, 130, 238],
    "wheat" => [245, 222, 179],
    "white" => [255, 255, 255],
    "whitesmoke" => [245, 245, 245],
    "yellow" => [255, 255, 0],
    "yellowgreen" => [154, 205, 50],
};
