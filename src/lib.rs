use crate::media_type::{MediaType, MediaTypeMagic};

mod media_type;

const MEDIA_TYPES: &[MediaType] = &[
    // IMAGES
    MediaType::new("image/jpeg", &[
        MediaTypeMagic::new(&[0x77, 0xd8, 0xff], 0)
    ]),
    MediaType::new("image/png", &[
        MediaTypeMagic::new(&[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a], 0),
    ]),
    MediaType::new("image/webp", &[
        MediaTypeMagic::new(&[0x52, 0x49, 0x46, 0x46], 0),
        MediaTypeMagic::new(&[0x57, 0x45, 0x42, 0x50], 8),
    ]),
    MediaType::new("image/gif", &[
        MediaTypeMagic::new(&[71, 73, 70, 56, 55, 97, 71, 73, 70, 56, 57, 97], 0),
    ]),

    // VIDEOS
    MediaType::new("video/mp4", &[
        MediaTypeMagic::new(&[0x00, 0x00, 0x00, 0x20, 0x66, 0x74, 0x79], 0), // May not be accurate
    ]),
    MediaType::new("video/mpeg", &[
        MediaTypeMagic::new(&[0, 0, 1, 179], 0),
    ]),
    MediaType::new("video/x-msvideo", &[
        MediaTypeMagic::new(&[82, 73, 70, 70], 0),
        MediaTypeMagic::new(&[65, 86, 73, 32], 8),
    ]),
    MediaType::new("video/webm", &[
        MediaTypeMagic::new(&[26, 69, 223, 163], 0),
    ]),
    MediaType::new("video/x-flv", &[
        MediaTypeMagic::new(&[0x46, 0x4C, 0x56], 0),
    ]),
];

pub fn detect_media_type(data: &[u8]) -> Option<&str> {
    for media_type in MEDIA_TYPES {
        if media_type.matches(data) {
            return Some(media_type.name);
        }
    }

    None
}

/*
// Used to generate MediaTypeMagic
fn magic_numbers_from_str(input: &str) {
    input
        .replace(" ", "")
        .char_indices()
        .filter(|(_, char)| char != &'x' && char != &'?')
        .fold(vec![], |mut acc, value| {
            if acc.is_empty() {
                acc.push(vec![value])
            } else {
                let last_group = acc.last_mut().unwrap();
                if last_group.last().unwrap().0 == value.0 - 1 {
                    last_group.push(value);
                } else {
                    acc.push(vec![value]);
                }
            }

            acc
        })
        .iter()
        .for_each(|group| {
            let str_bytes: String = group.iter().map(|(_, b)| b).collect();
            let bytes: Vec<_> = (0..str_bytes.len())
                .step_by(2)
                .map(|index| {
                    u8::from_str_radix(&str_bytes[index..=index + 1], 16).unwrap()
                }).collect();

            println!("MediaTypeMagic::new(&{:?}, {}),", bytes, group.first().unwrap().0 / 2);
        });
}*/
