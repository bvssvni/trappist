use *;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PlanetName {
    Tellar,
    Munos,
    Sand,
    Produm,
    Xir,
    Ja,
    Karalal,
}

impl PlanetName {
    pub fn all() -> &'static [PlanetName] {
        return &[
            Tellar,
            Munos,
            Sand,
            Produm,
            Xir,
            Ja,
            Karalal,
        ]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum OrbitName {
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl OrbitName {
    pub fn all() -> &'static [OrbitName] {
        return &[
            OrbitName::B,
            OrbitName::C,
            OrbitName::D,
            OrbitName::E,
            OrbitName::F,
            OrbitName::G,
            OrbitName::H,
        ]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SpeciesName {
    Vatrax,
    Ralm,
    Protrak,
}

impl SpeciesName {
    pub fn all() -> &'static [SpeciesName] {
        return &[
            Vatrax,
            Ralm,
            Protrak,
        ]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CityName {
    Eldonar,
    Tarat,
}

impl CityName {
    pub fn all() -> &'static [CityName] {
        &[
            Eldonar,
            Tarat,
        ]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum LocationName {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl LocationName {
    pub fn all() -> &'static [LocationName] {
        &[
            LocationName::A,
            LocationName::B,
            LocationName::C,
            LocationName::D,
        ]
    }
}

/// Carried weapon.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum WeaponName {
    XV43,
    TT180,
}

impl WeaponName {
    pub fn all() -> &'static [WeaponName] {
        &[
            XV43,
            TT180,
        ]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PlayerName {
    Alice,
    Bob,
    Carl,
}

impl PlayerName {
    pub fn all() -> &'static [PlayerName] {
        &[
            Alice,
            Bob,
            Carl,
        ]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SpaceshipName {
    Folkum,
}

impl SpaceshipName {
    pub fn all() -> &'static [SpaceshipName] {
        &[
            Folkum,
        ]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CanonName {
    SR6,
}

impl CanonName {
    pub fn all() -> &'static [CanonName] {
        &[
            SR6,
        ]
    }
}
