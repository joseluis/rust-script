#[derive(Copy, Clone, Debug)]
pub enum BuildKind {
    Normal,
    Test,
    Bench,
    Doc,
}

impl BuildKind {
    pub const fn exec_command(&self) -> &'static str {
        match *self {
            Self::Normal => "build",
            Self::Test => "test",
            Self::Bench => "bench",
            Self::Doc => "doc",
        }
    }

    pub fn from_flags(test: bool, bench: bool, doc: bool) -> Self {
        match (test, bench, doc) {
            (false, false, false) => Self::Normal,
            (true, false, false) => Self::Test,
            (false, true, false) => Self::Bench,
            (false, false, true) => Self::Doc,
            _ => panic!("must choose just one from test, bench and doc"),
        }
    }
}
