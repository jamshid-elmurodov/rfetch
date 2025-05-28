use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Unit {
    KiB,
    MiB,
    GiB,
}

impl Default for Unit {
    fn default() -> Self {
        Unit::GiB
    }
}