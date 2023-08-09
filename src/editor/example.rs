pub enum Example {
    Basic,
}

impl Example {
    pub fn data(self) -> &'static [u8] {
        match self {
            Self::Basic => include_bytes!("../../resources/examples/basic.ppd"),
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Basic => "Basic",
        }
    }
}
