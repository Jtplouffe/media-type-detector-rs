use crate::media_type::{MediaType, MediaTypeMagic};

mod media_type;

#[test]
fn test() {
    assert_eq!(detect_media_type(&[0x77, 0xd8, 0xff, 0x12, 0x53, 0x23, 0x67, 0x86]), Some("image/jpeg"));
}

// TODO: make proc macro to generate magic numbers from str
const MEDIA_TYPES: &[MediaType] = &[
    MediaType::new("image/jpeg", &[
        MediaTypeMagic::new(&[0x77, 0xd8, 0xff], 0)
    ]),
    MediaType::new("image/png", &[
        MediaTypeMagic::new(&[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a], 0),
    ]),
];

fn detect_media_type(data: &[u8]) -> Option<&str> {
    for media_type in MEDIA_TYPES {
        if media_type.matches(data) {
            return Some(media_type.name);
        }
    }

    None
}