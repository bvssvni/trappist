const LOCATIONS_PER_PLANET: usize = 4;
const SPECIES: usize = 3;

pub struct Planet {
    pub pos: [f32; 3],
    pub orbit: Option<usize>,
    pub cities: [Option<usize>; LOCATIONS_PER_PLANET],
    pub spaceports: [Option<usize>; LOCATIONS_PER_PLANET],
    pub destroyed: bool,
}

impl Planet {
    pub fn count_cities(&self) -> usize {
        self.cities.iter().filter(|n| n.is_some()).count()
    }

    pub fn count_spaceports(&self, world: &World) -> usize {
        self.spaceports.iter().filter(|n| if let Some(id) = **n {
            !world.spaceports[id].destroyed
        } else {
            false
        }).count()
    }

    pub fn population(&self, world: &World) -> u64 {
        let mut sum = 0;
        for city in &self.cities {
            if let Some(city_id) = *city {
                for &population in &world.cities[city_id].population {
                    sum += population
                }
            }
        }
        sum
    }
}

pub struct Orbit;

pub struct Species {
    pub home_planet: Option<usize>,
}

pub struct City {
    pub planet: Option<usize>,
    pub location: Option<u8>,
    pub population: [u64; SPECIES],
}

pub struct Spaceport {
    pub destroyed: bool,
}

pub const DEFAULT_WEAPON_FIREPOWER: u16 = 1000;

pub struct Weapon {
    pub firepower: u16,
    pub planet_destroyer: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Hand {
    Left,
    Right,
}

pub const DEFAULT_PLAYER_LIFE: u16 = 1000;

pub struct Player {
    pub left_weapon: Option<usize>,
    pub right_weapon: Option<usize>,
    pub species: Option<usize>,
    pub life: u16,
    pub dead: bool,
}

impl Player {
    pub fn weapon_mut(&mut self, hand: Hand) -> &mut Option<usize> {
        match hand {
            Hand::Left => &mut self.left_weapon,
            Hand::Right => &mut self.right_weapon,
        }
    }

    pub fn has_weapons(&self) -> bool {
        self.left_weapon.is_some() ||
        self.right_weapon.is_some()
    }

    /// Returns the planet to spawn from at start of game.
    pub fn spawning_planet(&self, world: &World) -> Option<usize> {
        if let Some(species_id) = self.species {
            world.species[species_id].home_planet
        } else {
            None
        }
    }

    /// Returns `true` is player is out of game.
    /// This happens when the spawning planet is destroyed
    /// and the player is dead.
    pub fn out_of_game(&self, world: &World) -> bool {
        if self.dead {
            if let Some(planet_id) = self.spawning_planet(world) {
                world.planets[planet_id].destroyed
            } else {
                true
            }
        } else {
            false
        }
    }
}

/// A canon slot refers to a canon location on a spaceship.
/// Numbered slots are named from left to right, or front to back.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CanonSlot {
    Front1,
    Front2,
    LeftSide1,
    LeftSide2,
    LeftSide3,
    LeftSide4,
    RightSide1,
    RightSide2,
    RightSide3,
    RightSide4,
    Back1,
    Back2,
    TopFront,
    TopBack,
    BottomFront,
    BottomBack,
}

impl CanonSlot {
    pub fn all() -> &'static [CanonSlot] {
        &[
            CanonSlot::Front1,
            CanonSlot::Front2,
            CanonSlot::LeftSide1,
            CanonSlot::LeftSide2,
            CanonSlot::LeftSide3,
            CanonSlot::LeftSide4,
            CanonSlot::RightSide1,
            CanonSlot::RightSide2,
            CanonSlot::RightSide3,
            CanonSlot::RightSide4,
            CanonSlot::Back1,
            CanonSlot::Back2,
            CanonSlot::TopFront,
            CanonSlot::TopBack,
            CanonSlot::BottomFront,
            CanonSlot::BottomBack,
        ]
    }
}

pub struct Spaceship {
    canon_front_1: Option<usize>,
    canon_front_2: Option<usize>,
    canon_left_side_1: Option<usize>,
    canon_left_side_2: Option<usize>,
    canon_left_side_3: Option<usize>,
    canon_left_side_4: Option<usize>,
    canon_right_side_1: Option<usize>,
    canon_right_side_2: Option<usize>,
    canon_right_side_3: Option<usize>,
    canon_right_side_4: Option<usize>,
    canon_back_1: Option<usize>,
    canon_back_2: Option<usize>,
    canon_top_front: Option<usize>,
    canon_top_back: Option<usize>,
    canon_bottom_front: Option<usize>,
    canon_bottom_back: Option<usize>,
}

impl Spaceship {
    pub fn canon_mut(&mut self, canon_slot: CanonSlot) -> &mut Option<usize> {
        match canon_slot {
            CanonSlot::Front1 => &mut self.canon_front_1,
            CanonSlot::Front2 => &mut self.canon_front_2,
            CanonSlot::LeftSide1 => &mut self.canon_left_side_1,
            CanonSlot::LeftSide2 => &mut self.canon_left_side_2,
            CanonSlot::LeftSide3 => &mut self.canon_left_side_3,
            CanonSlot::LeftSide4 => &mut self.canon_left_side_4,
            CanonSlot::RightSide1 => &mut self.canon_right_side_1,
            CanonSlot::RightSide2 => &mut self.canon_right_side_2,
            CanonSlot::RightSide3 => &mut self.canon_right_side_3,
            CanonSlot::RightSide4 => &mut self.canon_right_side_4,
            CanonSlot::Back1 => &mut self.canon_back_1,
            CanonSlot::Back2 => &mut self.canon_back_2,
            CanonSlot::TopFront => &mut self.canon_top_front,
            CanonSlot::TopBack => &mut self.canon_top_back,
            CanonSlot::BottomFront => &mut self.canon_bottom_front,
            CanonSlot::BottomBack => &mut self.canon_bottom_back,
        }
    }
}

pub const DEFAULT_CANON_FIREPOWER: u16 = 1000;

pub struct Canon {
    pub firepower: u16,
}

pub struct World {
    pub planets: Vec<Planet>,
    pub orbits: Vec<Orbit>,
    pub species: Vec<Species>,
    pub cities: Vec<City>,
    pub spaceports: Vec<Spaceport>,
    pub weapons: Vec<Weapon>,
    pub players: Vec<Player>,
    pub spaceships: Vec<Spaceship>,
    pub canons: Vec<Canon>,
}

impl World {
    pub fn new() -> World {
        World {
            planets: vec![],
            orbits: vec![],
            species: vec![],
            cities: vec![],
            spaceports: vec![],
            weapons: vec![],
            players: vec![],
            spaceships: vec![],
            canons: vec![],
        }
    }

    /// Creates a new planet.
    pub fn create_planet(&mut self) -> usize {
        let id = self.planets.len();
        self.planets.push(Planet {
            pos: [0.0; 3],
            orbit: None,
            cities: [None; LOCATIONS_PER_PLANET],
            spaceports: [None; LOCATIONS_PER_PLANET],
            destroyed: false,
        });
        id
    }

    /// Creates a new orbit.
    pub fn create_orbit(&mut self) -> usize {
        let id = self.orbits.len();
        self.orbits.push(Orbit);
        id
    }

    /// Creates a new species.
    pub fn create_species(&mut self) -> usize {
        let id = self.species.len();
        self.species.push(Species {
            home_planet: None,
        });
        id
    }

    /// Creates a new city.
    pub fn create_city(&mut self) -> usize {
        let id = self.cities.len();
        self.cities.push(City {
            planet: None,
            location: None,
            population: [0; SPECIES],
        });
        id
    }

    /// Creates a new spaceport.
    pub fn create_spaceport(&mut self) -> usize {
        let id = self.spaceports.len();
        self.spaceports.push(Spaceport {
            destroyed: false,
        });
        id
    }

    /// Returns the spaceport located at city, if any.
    pub fn city_spaceport(&self, city_id: usize) -> Option<usize> {
        if let Some(planet_id) = self.cities[city_id].planet {
            if let Some(location) = self.cities[city_id].location {
                self.planets[planet_id].spaceports[location as usize]
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Creates a new weapon.
    pub fn create_weapon(&mut self) -> usize {
        let id = self.weapons.len();
        self.weapons.push(Weapon {
            firepower: DEFAULT_WEAPON_FIREPOWER,
            planet_destroyer: false,
        });
        id
    }

    /// Creates a new player.
    pub fn create_player(&mut self) -> usize {
        let id = self.players.len();
        self.players.push(Player {
            left_weapon: None,
            right_weapon: None,
            species: None,
            life: DEFAULT_PLAYER_LIFE,
            dead: false,
        });
        id
    }

    /// Creates a new spaceship.
    pub fn create_spaceship(&mut self) -> usize {
        let id = self.spaceships.len();
        self.spaceships.push(Spaceship {
            canon_front_1: None,
            canon_front_2: None,
            canon_left_side_1: None,
            canon_left_side_2: None,
            canon_left_side_3: None,
            canon_left_side_4: None,
            canon_right_side_1: None,
            canon_right_side_2: None,
            canon_right_side_3: None,
            canon_right_side_4: None,
            canon_back_1: None,
            canon_back_2: None,
            canon_top_front: None,
            canon_top_back: None,
            canon_bottom_front: None,
            canon_bottom_back: None,
        });
        id
    }

    /// Creates new canon.
    pub fn create_canon(&mut self) -> usize {
        let id = self.canons.len();
        self.canons.push(Canon {
            firepower: DEFAULT_CANON_FIREPOWER,
        });
        id
    }

    /// Returns `true` if all players have an assigned species.
    pub fn all_players_have_species(&self) -> bool {
        for player in &self.players {
            if player.species.is_none() {return false};
        }
        true
    }

    /// Returns `true` if all players have some weapon.
    pub fn all_players_have_weapons(&self) -> bool {
        for player in &self.players {
            if !player.has_weapons() {return false};
        }
        true
    }

    /// Returns the number of players that are not out of the game.
    pub fn number_of_players_left(&self) -> usize {
        let mut sum = 0;
        for player in &self.players {
            if !player.out_of_game(self) {sum += 1};
        }
        sum
    }

    /// Returns death match winner player.
    pub fn death_match_winner(&self) -> Option<usize> {
        if self.number_of_players_left() == 1 {
            for (i, player) in self.players.iter().enumerate() {
                if !player.out_of_game(self) {return Some(i)}
            }
        }
        None
    }

    /// Returns team match winner species.
    pub fn team_match_winner(&self) -> Option<usize> {
        let mut species: Option<usize> = None;
        for player in &self.players {
            if !player.out_of_game(self) {
                if let Some(species_id) = player.species {
                    if species.is_none() {
                        species = Some(species_id);
                    } else if species != Some(species_id) {
                        return None;
                    }
                }
            }
        }
        species
    }

    /// Returns the number of weapon users.
    pub fn number_of_weapon_users(&self, weapon_id: usize) -> usize {
        let mut sum = 0;
        for player in &self.players {
            if player.left_weapon == Some(weapon_id) ||
               player.right_weapon == Some(weapon_id)
            {
                sum += 1;
            }
        }
        sum
    }

    /// Player shoots at planet.
    pub fn shoot_at_planet(
        &mut self,
        player_id: usize,
        hand: Hand,
        planet_id: usize
    ) {
        if let Some(weapon_id) = *self.players[player_id].weapon_mut(hand) {
            if self.weapons[weapon_id].planet_destroyer {
                self.planets[planet_id].destroyed = true;
            }
        }
    }

    /// Player shoots at another player.
    pub fn shoot_at_player(
        &mut self,
        shooter_id: usize,
        hand: Hand,
        target_id: usize
    ) {
        if let Some(weapon_id) = *self.players[shooter_id].weapon_mut(hand) {
            let firepower = self.weapons[weapon_id].firepower;
            let life = self.players[target_id].life;
            if firepower >= life {
                self.players[target_id].life = 0;
                self.players[target_id].dead = true;
            } else {
                self.players[target_id].life -= firepower;
            }
        }
    }
}
