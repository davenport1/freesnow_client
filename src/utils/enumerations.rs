use bitflags::bitflags;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum DangerRating {
    NoRating,
    Low,
    Moderate,
    Considerable,
    High,
    Extreme,
}

#[derive(Debug, Serialize)]
pub enum Size {
    SmallLarge,
    LargeVeryLarge,
    VeryLargeHistoric,
}

#[derive(Debug, Serialize)]
pub enum Likelihood {
    None,
    Unlikely,
    Possible,
    Likely,
    VeryLikely,
    Certain,
}

#[derive(Debug, Serialize)]
pub enum ProblemTypes {
    None,
    WindSlab,
    StormSlab,
    PersistentSlab,
    LooseDry,
    PersistentWeakLayer,
    CorniceFall,
    Glide,
    WetSnow,
}

bitflags! {
    pub struct AspectFlags: u32 {
        const None = 0b00000000;
        const North = 0b00000001;
        const Northwest = 0b00000010;
        const West = 0b00000100;
        const Southwest = 0b00001000;
        const South = 0b00010000;
        const Southeast = 0b00100000;
        const East = 0b01000000;
        const Northeast = 0b10000000;
    }

    pub struct ElevationFlags: u32 {
        const None = 0b00000000;
        const BelowTreeline = 0b00000001;
        const AtTreeline = 0b00000010;
        const AboveTreeline = 0b00000100;
    }
}
