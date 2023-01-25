use raqote::SolidSource;

pub fn get_color(dot: f32) -> SolidSource {
    let scaled = dot * 10.;
    let color = match scaled {
        scaled if scaled >= 10. => SolidSource {
            // very light
            r: 0xff,
            g: 0xff,
            b: 0xff,
            a: 0xff,
        },
        scaled if scaled >= 9. => SolidSource {
            r: 0xfd,
            g: 0xf8,
            b: 0xf7,
            a: 0xff,
        },
        scaled if scaled >= 8. => SolidSource {
            r: 0xf8,
            g: 0xea,
            b: 0xe7,
            a: 0xff,
        },
        scaled if scaled >= 7. => SolidSource {
            r: 0xf4,
            g: 0xdc,
            b: 0xd7,
            a: 0xff,
        },
        scaled if scaled >= 6. => SolidSource {
            r: 0xf0,
            g: 0xce,
            b: 0xc7,
            a: 0xff,
        },
        scaled if scaled >= 5. => SolidSource {
            r: 0xeb,
            g: 0xc0,
            b: 0xb7,
            a: 0xff,
        },
        scaled if scaled >= 4. => SolidSource {
            r: 0xe7,
            g: 0xb2,
            b: 0xa7,
            a: 0xff,
        },
        scaled if scaled >= 3. => SolidSource {
            r: 0xe3,
            g: 0xa4,
            b: 0x97,
            a: 0xff,
        },
        scaled if scaled >= 2. => SolidSource {
            r: 0xde,
            g: 0x96,
            b: 0x87,
            a: 0xff,
        },
        scaled if scaled >= 1. => SolidSource {
            r: 0xda,
            g: 0x88,
            b: 0x77,
            a: 0xff,
        },
        _ => SolidSource {
            r: 0xd6,
            g: 0x7a,
            b: 0x67,
            a: 0xff,
        },
    };

    color
}
