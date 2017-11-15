const LOCATIONS_PER_PLANET: usize = 4;
const SPECIES: usize = 3;

pub struct Planet {
    pub pos: [f32; 3],
    pub orbit: Option<usize>,
    pub cities: [Option<usize>; LOCATIONS_PER_PLANET],
    pub spaceports: [Option<usize>; LOCATIONS_PER_PLANET],
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

pub struct World {
    pub planets: Vec<Planet>,
    pub orbits: Vec<Orbit>,
    pub species: Vec<Species>,
    pub cities: Vec<City>,
    pub spaceports: Vec<Spaceport>,
}

impl World {
    pub fn new() -> World {
        World {
            planets: vec![],
            orbits: vec![],
            species: vec![],
            cities: vec![],
            spaceports: vec![],
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
}
