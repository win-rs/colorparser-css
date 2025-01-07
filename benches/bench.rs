use std::{collections::HashMap, sync::LazyLock};

use criterion::{Criterion, criterion_group, criterion_main};
use rustc_hash::FxHashMap;

pub static NAMED_COLORS_FXMAP: LazyLock<FxHashMap<&'static str, [u8; 3]>> = LazyLock::new(|| {
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

pub static NAMED_COLORS_MAP: LazyLock<HashMap<&'static str, [u8; 3]>> = LazyLock::new(|| {
    let mut map = HashMap::new();
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

pub fn match_color(color: &str) -> Option<[u8; 3]> {
    match color {
        "aliceblue" => Some([240, 248, 255]),
        "antiquewhite" => Some([250, 235, 215]),
        "aqua" => Some([0, 255, 255]),
        "aquamarine" => Some([127, 255, 212]),
        "azure" => Some([240, 255, 255]),
        "beige" => Some([245, 245, 220]),
        "bisque" => Some([255, 228, 196]),
        "black" => Some([0, 0, 0]),
        "blanchedalmond" => Some([255, 235, 205]),
        "blue" => Some([0, 0, 255]),
        "blueviolet" => Some([138, 43, 226]),
        "brown" => Some([165, 42, 42]),
        "burlywood" => Some([222, 184, 135]),
        "cadetblue" => Some([95, 158, 160]),
        "chartreuse" => Some([127, 255, 0]),
        "chocolate" => Some([210, 105, 30]),
        "coral" => Some([255, 127, 80]),
        "cornflowerblue" => Some([100, 149, 237]),
        "cornsilk" => Some([255, 248, 220]),
        "crimson" => Some([220, 20, 60]),
        "cyan" => Some([0, 255, 255]),
        "darkblue" => Some([0, 0, 139]),
        "darkcyan" => Some([0, 139, 139]),
        "darkgoldenrod" => Some([184, 134, 11]),
        "darkgray" => Some([169, 169, 169]),
        "darkgreen" => Some([0, 100, 0]),
        "darkgrey" => Some([169, 169, 169]),
        "darkkhaki" => Some([189, 183, 107]),
        "darkmagenta" => Some([139, 0, 139]),
        "darkolivegreen" => Some([85, 107, 47]),
        "darkorange" => Some([255, 140, 0]),
        "darkorchid" => Some([153, 50, 204]),
        "darkred" => Some([139, 0, 0]),
        "darksalmon" => Some([233, 150, 122]),
        "darkseagreen" => Some([143, 188, 143]),
        "darkslateblue" => Some([72, 61, 139]),
        "darkslategray" => Some([47, 79, 79]),
        "darkslategrey" => Some([47, 79, 79]),
        "darkturquoise" => Some([0, 206, 209]),
        "darkviolet" => Some([148, 0, 211]),
        "deeppink" => Some([255, 20, 147]),
        "deepskyblue" => Some([0, 191, 255]),
        "dimgray" => Some([105, 105, 105]),
        "dimgrey" => Some([105, 105, 105]),
        "dodgerblue" => Some([30, 144, 255]),
        "firebrick" => Some([178, 34, 34]),
        "floralwhite" => Some([255, 250, 240]),
        "forestgreen" => Some([34, 139, 34]),
        "fuchsia" => Some([255, 0, 255]),
        "gainsboro" => Some([220, 220, 220]),
        "ghostwhite" => Some([248, 248, 255]),
        "gold" => Some([255, 215, 0]),
        "goldenrod" => Some([218, 165, 32]),
        "gray" => Some([128, 128, 128]),
        "green" => Some([0, 128, 0]),
        "greenyellow" => Some([173, 255, 47]),
        "grey" => Some([128, 128, 128]),
        "honeydew" => Some([240, 255, 240]),
        "hotpink" => Some([255, 105, 180]),
        "indianred" => Some([205, 92, 92]),
        "indigo" => Some([75, 0, 130]),
        "ivory" => Some([255, 255, 240]),
        "khaki" => Some([240, 230, 140]),
        "lavender" => Some([230, 230, 250]),
        "lavenderblush" => Some([255, 240, 245]),
        "lawngreen" => Some([124, 252, 0]),
        "lemonchiffon" => Some([255, 250, 205]),
        "lightblue" => Some([173, 216, 230]),
        "lightcoral" => Some([240, 128, 128]),
        "lightcyan" => Some([224, 255, 255]),
        "lightgoldenrodyellow" => Some([250, 250, 210]),
        "lightgray" => Some([211, 211, 211]),
        "lightgreen" => Some([144, 238, 144]),
        "lightgrey" => Some([211, 211, 211]),
        "lightpink" => Some([255, 182, 193]),
        "lightsalmon" => Some([255, 160, 122]),
        "lightseagreen" => Some([32, 178, 170]),
        "lightskyblue" => Some([135, 206, 250]),
        "lightslategray" => Some([119, 136, 153]),
        "lightslategrey" => Some([119, 136, 153]),
        "lightsteelblue" => Some([176, 196, 222]),
        "lightyellow" => Some([255, 255, 224]),
        "lime" => Some([0, 255, 0]),
        "limegreen" => Some([50, 205, 50]),
        "linen" => Some([250, 240, 230]),
        "magenta" => Some([255, 0, 255]),
        "maroon" => Some([128, 0, 0]),
        "mediumaquamarine" => Some([102, 205, 170]),
        "mediumblue" => Some([0, 0, 205]),
        "mediumorchid" => Some([186, 85, 211]),
        "mediumpurple" => Some([147, 112, 219]),
        "mediumseagreen" => Some([60, 179, 113]),
        "mediumslateblue" => Some([123, 104, 238]),
        "mediumspringgreen" => Some([0, 250, 154]),
        "mediumturquoise" => Some([72, 209, 204]),
        "mediumvioletred" => Some([199, 21, 133]),
        "midnightblue" => Some([25, 25, 112]),
        "mintcream" => Some([245, 255, 250]),
        "mistyrose" => Some([255, 228, 225]),
        "moccasin" => Some([255, 228, 181]),
        "navajowhite" => Some([255, 222, 173]),
        "navy" => Some([0, 0, 128]),
        "oldlace" => Some([253, 245, 230]),
        "olive" => Some([128, 128, 0]),
        "olivedrab" => Some([107, 142, 35]),
        "orange" => Some([255, 165, 0]),
        "orangered" => Some([255, 69, 0]),
        "orchid" => Some([218, 112, 214]),
        "palegoldenrod" => Some([238, 232, 170]),
        "palegreen" => Some([152, 251, 152]),
        "paleturquoise" => Some([175, 238, 238]),
        "palevioletred" => Some([219, 112, 147]),
        "papayawhip" => Some([255, 239, 213]),
        "peachpuff" => Some([255, 218, 185]),
        "peru" => Some([205, 133, 63]),
        "pink" => Some([255, 192, 203]),
        "plum" => Some([221, 160, 221]),
        "powderblue" => Some([176, 224, 230]),
        "purple" => Some([128, 0, 128]),
        "rebeccapurple" => Some([102, 51, 153]),
        "red" => Some([255, 0, 0]),
        "rosybrown" => Some([188, 143, 143]),
        "royalblue" => Some([65, 105, 225]),
        "saddlebrown" => Some([139, 69, 19]),
        "salmon" => Some([250, 128, 114]),
        "sandybrown" => Some([244, 164, 96]),
        "seagreen" => Some([46, 139, 87]),
        "seashell" => Some([255, 245, 238]),
        "sienna" => Some([160, 82, 45]),
        "silver" => Some([192, 192, 192]),
        "skyblue" => Some([135, 206, 235]),
        "slateblue" => Some([106, 90, 205]),
        "slategray" => Some([112, 128, 144]),
        "slategrey" => Some([112, 128, 144]),
        "snow" => Some([255, 250, 250]),
        "springgreen" => Some([0, 255, 127]),
        "steelblue" => Some([70, 130, 180]),
        "tan" => Some([210, 180, 140]),
        "teal" => Some([0, 128, 128]),
        "thistle" => Some([216, 191, 216]),
        "tomato" => Some([255, 99, 71]),
        "turquoise" => Some([64, 224, 208]),
        "violet" => Some([238, 130, 238]),
        "wheat" => Some([245, 222, 179]),
        "white" => Some([255, 255, 255]),
        "whitesmoke" => Some([245, 245, 245]),
        "yellow" => Some([255, 255, 0]),
        "yellowgreen" => Some([154, 205, 50]),
        _ => None, // Handle unknown color names
    }
}

// Benchmark function for `phf::Map`
fn bench_phf_map(c: &mut Criterion) {
    let keys = ["wheat", "white", "blue", "red", "green"];
    c.bench_function("phf_map_lookup", |b| {
        b.iter(|| {
            let key = keys[rand::random::<usize>() % keys.len()];
            let result = NAMED_COLORS.get(key);
            assert!(result.is_some()); // Validate output
        })
    });
}

// Benchmark function for `match`
fn bench_hash_map(c: &mut Criterion) {
    let keys = ["wheat", "white", "blue", "red", "green"];
    c.bench_function("hash_map_lookup", |b| {
        b.iter(|| {
            let key = keys[rand::random::<usize>() % keys.len()];
            let result = NAMED_COLORS_MAP.get(key);
            assert!(result.is_some()); // Validate output
        })
    });
}

fn bench_fxhash_map(c: &mut Criterion) {
    let keys = ["wheat", "white", "blue", "red", "green"];
    c.bench_function("fxhash_map_lookup", |b| {
        b.iter(|| {
            let key = keys[rand::random::<usize>() % keys.len()];
            let result = NAMED_COLORS_FXMAP.get(key);
            assert!(result.is_some()); // Validate output
        })
    });
}

// Benchmark function for `match`
fn bench_match(c: &mut Criterion) {
    let keys = ["wheat", "white", "blue", "red", "green"];
    c.bench_function("match_lookup", |b| {
        b.iter(|| {
            let key = keys[rand::random::<usize>() % keys.len()];
            let result = match_color(key);
            assert!(result.is_some()); // Validate output
        })
    });
}

criterion_group!(
    benches,
    bench_phf_map,
    bench_hash_map,
    bench_fxhash_map,
    bench_match
);
criterion_main!(benches);
