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

pub struct Weapon;

pub struct Player {
    pub left_weapon: Option<usize>,
    pub right_weapon: Option<usize>,
    pub species: Option<usize>,
    pub dead: bool,
}

impl Player {
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

pub struct World {
    pub planets: Vec<Planet>,
    pub orbits: Vec<Orbit>,
    pub species: Vec<Species>,
    pub cities: Vec<City>,
    pub spaceports: Vec<Spaceport>,
    pub weapons: Vec<Weapon>,
    pub players: Vec<Player>,
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
        self.weapons.push(Weapon);
        id
    }

    /// Creates a new player.
    pub fn create_player(&mut self) -> usize {
        let id = self.players.len();
        self.players.push(Player {
            left_weapon: None,
            right_weapon: None,
            species: None,
            dead: false,
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
}
