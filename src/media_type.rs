pub(crate) struct MediaType<'a> {
    // ex.: 'application/octet-stream'
    pub name: &'a str,
    magic_numbers: &'a [MediaTypeMagic<'a>],
}

impl<'a> MediaType<'a> {
    pub(crate) const fn new(name: &'a str, magic_numbers: &'a [MediaTypeMagic<'a>]) -> Self {
        Self {
            name,
            magic_numbers,
        }
    }

    pub(crate) fn matches(&self, header: &[u8]) -> bool {
        self.magic_numbers.iter().all(|magic| magic.matches(header))
    }
}

pub(crate) struct MediaTypeMagic<'a> {
    magic: &'a [u8],
    offset: usize,
}

impl<'a> MediaTypeMagic<'a> {
    pub(crate) const fn new(magic: &'a [u8], offset: usize) -> Self {
        Self {
            magic,
            offset,
        }
    }

    fn matches(&self, header: &[u8]) -> bool {
        if header.len() < self.magic.len() + self.offset {
            false
        } else {
            &header[self.offset..self.magic.len()] == self.magic
        }
    }
}
