pub mod base;
pub mod partials;

pub type HeaderSegments = &'static [HeaderSegment];

pub static SEGMENTS: HeaderSegments = &[
    HeaderSegment::new("dashboard", "/dashboard"),
    HeaderSegment::new("team", "/team"),
    HeaderSegment::new("projects", "/projects"),
    HeaderSegment::new("calender", "/calender"),
];

pub struct HeaderSegment {
    pub name: &'static str,
    pub url: &'static str,
}

impl HeaderSegment {
    pub const fn new(name: &'static str, url: &'static str) -> Self {
        Self { name, url }
    }
}
