// #[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RGBA(pub u8, pub u8, pub u8, pub u8);

impl RGBA {
    pub fn has_alpha(&self) -> bool {
        self.3 != 0xff
    }
    pub fn from_name(name: &str) -> Option<RGBA> {
        match COLOR_MAP.binary_search_by_key(&name, |&(name, color)| name) {
            Ok(idx) => Some(COLOR_MAP[idx].1),
            _ => None
        }
    }
}

macro_rules! map {
    ($($key:expr => $val:expr,)*) => {
        &[$(($key, $val)),*]
    };
}

static COLOR_MAP: &[(&'static str, RGBA)] = map!{
    "black" => RGBA(0x00,0x00,0x00,0xff),
    "silver" => RGBA(0xc0,0xc0,0xc0,0xff),
    "gray" => RGBA(0x80,0x80,0x80,0xff),
    "white" => RGBA(0xff,0xff,0xff,0xff),
    "maroon" => RGBA(0x80,0x00,0x00,0xff),
    "red" => RGBA(0xff,0x00,0x00,0xff),
    "purple" => RGBA(0x80,0x00,0x80,0xff),
    "fuchsia" => RGBA(0xff,0x00,0xff,0xff),
    "green" => RGBA(0x00,0x80,0x00,0xff),
    "lime" => RGBA(0x00,0xff,0x00,0xff),
    "olive" => RGBA(0x80,0x80,0x00,0xff),
    "yellow" => RGBA(0xff,0xff,0x00,0xff),
    "navy" => RGBA(0x00,0x00,0x80,0xff),
    "blue" => RGBA(0x00,0x00,0xff,0xff),
    "teal" => RGBA(0x00,0x80,0x80,0xff),
    "aqua" => RGBA(0x00,0xff,0xff,0xff),
    "orange" => RGBA(0xff,0xa5,0x00,0xff),
    "aliceblue" => RGBA(0xf0,0xf8,0xff,0xff),
    "antiquewhite" => RGBA(0xfa,0xeb,0xd7,0xff),
    "aquamarine" => RGBA(0x7f,0xff,0xd4,0xff),
    "azure" => RGBA(0xf0,0xff,0xff,0xff),
    "beige" => RGBA(0xf5,0xf5,0xdc,0xff),
    "bisque" => RGBA(0xff,0xe4,0xc4,0xff),
    "blanchedalmond" => RGBA(0xff,0xeb,0xcd,0xff),
    "blueviolet" => RGBA(0x8a,0x2b,0xe2,0xff),
    "brown" => RGBA(0xa5,0x2a,0x2a,0xff),
    "burlywood" => RGBA(0xde,0xb8,0x87,0xff),
    "cadetblue" => RGBA(0x5f,0x9e,0xa0,0xff),
    "chartreuse" => RGBA(0x7f,0xff,0x00,0xff),
    "chocolate" => RGBA(0xd2,0x69,0x1e,0xff),
    "coral" => RGBA(0xff,0x7f,0x50,0xff),
    "cornflowerblue" => RGBA(0x64,0x95,0xed,0xff),
    "cornsilk" => RGBA(0xff,0xf8,0xdc,0xff),
    "crimson" => RGBA(0xdc,0x14,0x3c,0xff),
    "cyan" => RGBA(0x00,0xff,0xff,0xff),
    "darkblue" => RGBA(0x00,0x00,0x8b,0xff),
    "darkcyan" => RGBA(0x00,0x8b,0x8b,0xff),
    "darkgoldenrod" => RGBA(0xb8,0x86,0x0b,0xff),
    "darkgray" => RGBA(0xa9,0xa9,0xa9,0xff),
    "darkgreen" => RGBA(0x00,0x64,0x00,0xff),
    "darkgrey" => RGBA(0xa9,0xa9,0xa9,0xff),
    "darkkhaki" => RGBA(0xbd,0xb7,0x6b,0xff),
    "darkmagenta" => RGBA(0x8b,0x00,0x8b,0xff),
    "darkolivegreen" => RGBA(0x55,0x6b,0x2f,0xff),
    "darkorange" => RGBA(0xff,0x8c,0x00,0xff),
    "darkorchid" => RGBA(0x99,0x32,0xcc,0xff),
    "darkred" => RGBA(0x8b,0x00,0x00,0xff),
    "darksalmon" => RGBA(0xe9,0x96,0x7a,0xff),
    "darkseagreen" => RGBA(0x8f,0xbc,0x8f,0xff),
    "darkslateblue" => RGBA(0x48,0x3d,0x8b,0xff),
    "darkslategray" => RGBA(0x2f,0x4f,0x4f,0xff),
    "darkslategrey" => RGBA(0x2f,0x4f,0x4f,0xff),
    "darkturquoise" => RGBA(0x00,0xce,0xd1,0xff),
    "darkviolet" => RGBA(0x94,0x00,0xd3,0xff),
    "deeppink" => RGBA(0xff,0x14,0x93,0xff),
    "deepskyblue" => RGBA(0x00,0xbf,0xff,0xff),
    "dimgray" => RGBA(0x69,0x69,0x69,0xff),
    "dimgrey" => RGBA(0x69,0x69,0x69,0xff),
    "dodgerblue" => RGBA(0x1e,0x90,0xff,0xff),
    "firebrick" => RGBA(0xb2,0x22,0x22,0xff),
    "floralwhite" => RGBA(0xff,0xfa,0xf0,0xff),
    "forestgreen" => RGBA(0x22,0x8b,0x22,0xff),
    "gainsboro" => RGBA(0xdc,0xdc,0xdc,0xff),
    "ghostwhite" => RGBA(0xf8,0xf8,0xff,0xff),
    "gold" => RGBA(0xff,0xd7,0x00,0xff),
    "goldenrod" => RGBA(0xda,0xa5,0x20,0xff),
    "greenyellow" => RGBA(0xad,0xff,0x2f,0xff),
    "grey" => RGBA(0x80,0x80,0x80,0xff),
    "honeydew" => RGBA(0xf0,0xff,0xf0,0xff),
    "hotpink" => RGBA(0xff,0x69,0xb4,0xff),
    "indianred" => RGBA(0xcd,0x5c,0x5c,0xff),
    "indigo" => RGBA(0x4b,0x00,0x82,0xff),
    "ivory" => RGBA(0xff,0xff,0xf0,0xff),
    "khaki" => RGBA(0xf0,0xe6,0x8c,0xff),
    "lavender" => RGBA(0xe6,0xe6,0xfa,0xff),
    "lavenderblush" => RGBA(0xff,0xf0,0xf5,0xff),
    "lawngreen" => RGBA(0x7c,0xfc,0x00,0xff),
    "lemonchiffon" => RGBA(0xff,0xfa,0xcd,0xff),
    "lightblue" => RGBA(0xad,0xd8,0xe6,0xff),
    "lightcoral" => RGBA(0xf0,0x80,0x80,0xff),
    "lightcyan" => RGBA(0xe0,0xff,0xff,0xff),
    "lightgoldenrodyellow" => RGBA(0xfa,0xfa,0xd2,0xff),
    "lightgray" => RGBA(0xd3,0xd3,0xd3,0xff),
    "lightgreen" => RGBA(0x90,0xee,0x90,0xff),
    "lightgrey" => RGBA(0xd3,0xd3,0xd3,0xff),
    "lightpink" => RGBA(0xff,0xb6,0xc1,0xff),
    "lightsalmon" => RGBA(0xff,0xa0,0x7a,0xff),
    "lightseagreen" => RGBA(0x20,0xb2,0xaa,0xff),
    "lightskyblue" => RGBA(0x87,0xce,0xfa,0xff),
    "lightslategray" => RGBA(0x77,0x88,0x99,0xff),
    "lightslategrey" => RGBA(0x77,0x88,0x99,0xff),
    "lightsteelblue" => RGBA(0xb0,0xc4,0xde,0xff),
    "lightyellow" => RGBA(0xff,0xff,0xe0,0xff),
    "limegreen" => RGBA(0x32,0xcd,0x32,0xff),
    "linen" => RGBA(0xfa,0xf0,0xe6,0xff),
    "magenta" => RGBA(0xff,0x00,0xff,0xff),
    "mediumaquamarine" => RGBA(0x66,0xcd,0xaa,0xff),
    "mediumblue" => RGBA(0x00,0x00,0xcd,0xff),
    "mediumorchid" => RGBA(0xba,0x55,0xd3,0xff),
    "mediumpurple" => RGBA(0x93,0x70,0xdb,0xff),
    "mediumseagreen" => RGBA(0x3c,0xb3,0x71,0xff),
    "mediumslateblue" => RGBA(0x7b,0x68,0xee,0xff),
    "mediumspringgreen" => RGBA(0x00,0xfa,0x9a,0xff),
    "mediumturquoise" => RGBA(0x48,0xd1,0xcc,0xff),
    "mediumvioletred" => RGBA(0xc7,0x15,0x85,0xff),
    "midnightblue" => RGBA(0x19,0x19,0x70,0xff),
    "mintcream" => RGBA(0xf5,0xff,0xfa,0xff),
    "mistyrose" => RGBA(0xff,0xe4,0xe1,0xff),
    "moccasin" => RGBA(0xff,0xe4,0xb5,0xff),
    "navajowhite" => RGBA(0xff,0xde,0xad,0xff),
    "oldlace" => RGBA(0xfd,0xf5,0xe6,0xff),
    "olivedrab" => RGBA(0x6b,0x8e,0x23,0xff),
    "orangered" => RGBA(0xff,0x45,0x00,0xff),
    "orchid" => RGBA(0xda,0x70,0xd6,0xff),
    "palegoldenrod" => RGBA(0xee,0xe8,0xaa,0xff),
    "palegreen" => RGBA(0x98,0xfb,0x98,0xff),
    "paleturquoise" => RGBA(0xaf,0xee,0xee,0xff),
    "palevioletred" => RGBA(0xdb,0x70,0x93,0xff),
    "papayawhip" => RGBA(0xff,0xef,0xd5,0xff),
    "peachpuff" => RGBA(0xff,0xda,0xb9,0xff),
    "peru" => RGBA(0xcd,0x85,0x3f,0xff),
    "pink" => RGBA(0xff,0xc0,0xcb,0xff),
    "plum" => RGBA(0xdd,0xa0,0xdd,0xff),
    "powderblue" => RGBA(0xb0,0xe0,0xe6,0xff),
    "rosybrown" => RGBA(0xbc,0x8f,0x8f,0xff),
    "royalblue" => RGBA(0x41,0x69,0xe1,0xff),
    "saddlebrown" => RGBA(0x8b,0x45,0x13,0xff),
    "salmon" => RGBA(0xfa,0x80,0x72,0xff),
    "sandybrown" => RGBA(0xf4,0xa4,0x60,0xff),
    "seagreen" => RGBA(0x2e,0x8b,0x57,0xff),
    "seashell" => RGBA(0xff,0xf5,0xee,0xff),
    "sienna" => RGBA(0xa0,0x52,0x2d,0xff),
    "skyblue" => RGBA(0x87,0xce,0xeb,0xff),
    "slateblue" => RGBA(0x6a,0x5a,0xcd,0xff),
    "slategray" => RGBA(0x70,0x80,0x90,0xff),
    "slategrey" => RGBA(0x70,0x80,0x90,0xff),
    "snow" => RGBA(0xff,0xfa,0xfa,0xff),
    "springgreen" => RGBA(0x00,0xff,0x7f,0xff),
    "steelblue" => RGBA(0x46,0x82,0xb4,0xff),
    "tan" => RGBA(0xd2,0xb4,0x8c,0xff),
    "thistle" => RGBA(0xd8,0xbf,0xd8,0xff),
    "tomato" => RGBA(0xff,0x63,0x47,0xff),
    "turquoise" => RGBA(0x40,0xe0,0xd0,0xff),
    "violet" => RGBA(0xee,0x82,0xee,0xff),
    "wheat" => RGBA(0xf5,0xde,0xb3,0xff),
    "whitesmoke" => RGBA(0xf5,0xf5,0xf5,0xff),
    "yellowgreen" => RGBA(0x9a,0xcd,0x32,0xff),
    "rebeccapurple" => RGBA(0x66,0x33,0x99,0xff),
    "phantom" => RGBA(0x00,0x00,0x00,0x00),
    "transparent" => RGBA(0x00,0x00,0x00,0x00),
};