extern crate monotonic_solver;

use monotonic_solver::{solve, solve_and_reduce};

use std::collections::HashSet;

use Expr::*;
use PlanetName::*;
use SpeciesName::*;
use CityName::*;
use world::*;

mod world;
pub mod test;

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

pub struct State {
    /// Reference to the Tellar planet.
    tellar: Option<usize>,
    /// Reference to the Munos planet.
    munos: Option<usize>,
    /// Reference to the Sand planet.
    sand: Option<usize>,
    /// Reference to the Produm planet.
    produm: Option<usize>,
    /// Reference to the Xir planet.
    xir: Option<usize>,
    /// Reference to the Ja planet.
    ja: Option<usize>,
    /// Reference to the Karalal planet.
    karalal: Option<usize>,
    /// Reference to orbit B.
    orbit_b: Option<usize>,
    /// Reference to orbit C.
    orbit_c: Option<usize>,
    /// Reference to orbit D.
    orbit_d: Option<usize>,
    /// Reference to orbit E.
    orbit_e: Option<usize>,
    /// Reference to orbit F.
    orbit_f: Option<usize>,
    /// Reference to orbit G.
    orbit_g: Option<usize>,
    /// Reference to orbit H.
    orbit_h: Option<usize>,
    /// Reference to the Vatrax species.
    vatrax: Option<usize>,
    /// Reference to the Ralm species.
    ralm: Option<usize>,
    /// Reference to the Protrak species.
    protrak: Option<usize>,
    /// Reference to the Eldonar city.
    eldonar: Option<usize>,
    /// Reference to the Tarat city.
    tarat: Option<usize>,
}

impl State {
    pub fn new() -> State {
        State {
            tellar: None,
            munos: None,
            sand: None,
            produm: None,
            xir: None,
            ja: None,
            karalal: None,
            orbit_b: None,
            orbit_c: None,
            orbit_d: None,
            orbit_e: None,
            orbit_f: None,
            orbit_g: None,
            orbit_h: None,
            vatrax: None,
            ralm: None,
            protrak: None,
            eldonar: None,
            tarat: None,
        }
    }

    pub fn planet_mut(&mut self, name: PlanetName) -> &mut Option<usize> {
        match name {
            Tellar => &mut self.tellar,
            Munos => &mut self.munos,
            Sand => &mut self.sand,
            Produm => &mut self.produm,
            Xir => &mut self.xir,
            Ja => &mut self.ja,
            Karalal => &mut self.karalal,
        }
    }

    pub fn create_planet(&mut self, name: PlanetName, world: &mut World) {
        let id = world.create_planet();
        *self.planet_mut(name) = Some(id);
    }

    pub fn orbit_mut(&mut self, orbit: OrbitName) -> &mut Option<usize> {
        match orbit {
            OrbitName::B => &mut self.orbit_b,
            OrbitName::C => &mut self.orbit_c,
            OrbitName::D => &mut self.orbit_d,
            OrbitName::E => &mut self.orbit_e,
            OrbitName::F => &mut self.orbit_f,
            OrbitName::G => &mut self.orbit_g,
            OrbitName::H => &mut self.orbit_h,
        }
    }

    pub fn create_orbit(&mut self, orbit: OrbitName, world: &mut World) {
        let id = world.create_orbit();
        *self.orbit_mut(orbit) = Some(id);
    }

    pub fn assign_orbit(
        &mut self,
        name: PlanetName,
        orbit: OrbitName,
        world: &mut World
    ) -> Result<(), ()> {
        let name_id = self.planet_mut(name).ok_or(())?;
        let orbit_id = self.orbit_mut(orbit).ok_or(())?;
        world.planets[name_id].orbit = Some(orbit_id);
        Ok(())
    }

    pub fn species_mut(&mut self, species: SpeciesName) -> &mut Option<usize> {
        match species {
            Vatrax => &mut self.vatrax,
            Ralm => &mut self.ralm,
            Protrak => &mut self.protrak,
        }
    }

    pub fn create_species(&mut self, species: SpeciesName, world: &mut World) {
        let id = world.create_species();
        *self.species_mut(species) = Some(id);
    }

    pub fn assign_home_planet(
        &mut self,
        species: SpeciesName,
        planet: PlanetName,
        world: &mut World
    ) -> Result<(), ()> {
        let species_id = self.species_mut(species).ok_or(())?;
        let planet_id = self.planet_mut(planet).ok_or(())?;
        world.species[species_id].home_planet = Some(planet_id);
        Ok(())
    }

    pub fn city_mut(&mut self, city: CityName) -> &mut Option<usize> {
        match city {
            Eldonar => &mut self.eldonar,
            Tarat => &mut self.tarat,
        }
    }

    pub fn create_city(&mut self, city: CityName, world: &mut World) {
        let id = world.create_city();
        *self.city_mut(city) = Some(id);
    }

    pub fn assign_location(
        &mut self,
        city: CityName,
        planet: PlanetName,
        location: LocationName,
        world: &mut World
    ) -> Result<(), ()> {
        let city_id = self.city_mut(city).ok_or(())?;
        let planet_id = self.planet_mut(planet).ok_or(())?;
        let ref mut city = world.cities[city_id];
        city.planet = Some(planet_id);
        city.location = Some(location as u8);
        world.planets[planet_id].cities[location as usize] = Some(city_id);
        Ok(())
    }

    pub fn create_spaceport(
        &mut self,
        planet: PlanetName,
        location: LocationName,
        world: &mut World
    ) -> Result<(), ()> {
        let id = world.create_spaceport();
        let planet_id = self.planet_mut(planet).ok_or(())?;
        world.planets[planet_id].spaceports[location as usize] = Some(id);
        Ok(())
    }

    pub fn destroy_spaceport(
        &mut self,
        planet: PlanetName,
        location: LocationName,
        world: &mut World
    ) -> Result<(), ()> {
        let planet_id = self.planet_mut(planet).ok_or(())?;
        let spaceport_id = world.planets[planet_id].spaceports[location as usize].ok_or(())?;
        world.spaceports[spaceport_id].destroyed = true;
        Ok(())
    }

    pub fn rebuild_spaceport(
        &mut self,
        planet: PlanetName,
        location: LocationName,
        world: &mut World
    ) -> Result<(), ()> {
        let planet_id = self.planet_mut(planet).ok_or(())?;
        let spaceport_id = world.planets[planet_id].spaceports[location as usize].ok_or(())?;
        world.spaceports[spaceport_id].destroyed = false;
        Ok(())
    }

    pub fn populate_city(
        &mut self,
        city: CityName,
        n: u64,
        species: SpeciesName,
        world: &mut World
    ) -> Result<(), ()> {
        let city_id = self.city_mut(city).ok_or(())?;
        let species_id = self.species_mut(species).ok_or(())?;
        world.cities[city_id].population[species_id as usize] = n;
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Expr {
    /// Create new planet.
    CreatePlanet(PlanetName),
    /// Create new orbit.
    CreateOrbit(OrbitName),
    /// Create new species.
    CreateSpecies(SpeciesName),
    /// Create new city.
    CreateCity(CityName),
    /// Create new spaceport.
    CreateSpaceport(PlanetName, LocationName),
    /// Assign an orbit to planet.
    AssignOrbit(PlanetName, OrbitName),
    /// Assign a home planet to species.
    AssignHomePlanet(SpeciesName, PlanetName),
    /// Assign a planet location to city.
    AssignLocation(CityName, PlanetName, LocationName),
    /// Destroy spaceport.
    DestroySpaceport(PlanetName, LocationName),
    /// Rebuild spaceport.
    RebuildSpaceport(PlanetName, LocationName),
    /// Populates city with a number of people.
    PopulateCity(CityName, u64, SpeciesName),
    /// The story works out.
    Sound,
    /// The world contains planets.
    ContainsPlanets,
    /// The world contains a specific planet.
    ContainsPlanet(PlanetName),
    /// A planet contains a species.
    ContainsSpecies(PlanetName, SpeciesName),
    /// A planet has an orbit.
    HasOrbit(PlanetName, OrbitName),
    /// A city has a location.
    HasLocation(CityName),
    /// The number of cities on a planet.
    HasNumberOfCities(PlanetName, usize),
    /// The number of spaceports on a planet.
    HasNumberOfSpaceports(PlanetName, usize),
    /// Two species live on same planet.
    LiveOnSamePlanet(SpeciesName, SpeciesName),
    /// A city has a spaceport.
    CityHasSpaceport(CityName),
    /// A planet has space travel technology.
    HasSpaceTravel(PlanetName, bool),
    /// A planet has number of people.
    PlanetHasNumberOfPeople(PlanetName, u64),
}

fn infer(cache: &HashSet<Expr>, filter_cache: &HashSet<Expr>, story: &[Expr]) -> Option<Expr> {
    let can_add = |new_expr: &Expr| {
        !cache.contains(new_expr) &&
        !filter_cache.contains(new_expr)
    };

    let ref mut world = World::new();
    let ref mut state = State::new();

    // Execute expressions on world.
    for expr in story {
        if let CreatePlanet(name) = *expr {
            state.create_planet(name, world);
        }

        if let CreateOrbit(orbit) = *expr {
            state.create_orbit(orbit, world);
        }

        if let CreateSpecies(name) = *expr {
            state.create_species(name, world);
        }

        if let CreateCity(name) = *expr {
            state.create_city(name, world);
        }

        if let CreateSpaceport(planet, location) = *expr {
            if !state.create_spaceport(planet, location, world).is_ok() {
                return None;
            }
        }

        if let AssignOrbit(name, orbit) = *expr {
            if !state.assign_orbit(name, orbit, world).is_ok() {
                return None;
            }
        }

        if let AssignHomePlanet(species, planet) = *expr {
            if !state.assign_home_planet(species, planet, world).is_ok() {
                return None;
            }
        }

        if let AssignLocation(city, planet, location) = *expr {
            if !state.assign_location(city, planet, location, world).is_ok() {
                return None;
            }
        }

        if let DestroySpaceport(planet, location) = *expr {
            if !state.destroy_spaceport(planet, location, world).is_ok() {
                return None;
            }
        }

        if let RebuildSpaceport(planet, location) = *expr {
            if !state.rebuild_spaceport(planet, location, world).is_ok() {
                return None;
            }
        }

        if let PopulateCity(name, n, species) = *expr {
            if !state.populate_city(name, n, species, world).is_ok() {
                return None;
            }
        }
    }

    if world.planets.len() > 0 {
        let new_expr = ContainsPlanets;
        if can_add(&new_expr) {return Some(new_expr)};
    }

    for &name in PlanetName::all() {
        if let Some(planet_id) = *state.planet_mut(name) {
            let new_expr = ContainsPlanet(name);
            if can_add(&new_expr) {return Some(new_expr)};

            if let Some(orbit_id) = world.planets[planet_id].orbit {
                for &orbit in OrbitName::all() {
                    if let Some(id) = *state.orbit_mut(orbit) {
                        if orbit_id == id {
                            let new_expr = HasOrbit(name, orbit);
                            if can_add(&new_expr) {return Some(new_expr)};
                        }
                    }
                }
            }

            let count_cities = world.planets[planet_id].count_cities();
            let new_expr = HasNumberOfCities(name, count_cities);
            if can_add(&new_expr) {return Some(new_expr)};

            let count_spaceports = world.planets[planet_id].count_spaceports(world);
            let new_expr = HasNumberOfSpaceports(name, count_spaceports);
            if can_add(&new_expr) {return Some(new_expr)};

            let population = world.planets[planet_id].population(world);
            let new_expr = PlanetHasNumberOfPeople(name, population);
            if can_add(&new_expr) {return Some(new_expr)};
        }
    }

    for &species in SpeciesName::all() {
        if let Some(species_id) = *state.species_mut(species) {
            if let Some(planet_id) = world.species[species_id].home_planet {
                for &planet in PlanetName::all() {
                    if let Some(id) = *state.planet_mut(planet) {
                        if planet_id == id {
                            let new_expr = ContainsSpecies(planet, species);
                            if can_add(&new_expr) {return Some(new_expr)};
                        }
                    }
                }
            }
        }
    }

    for &city in CityName::all() {
        if let Some(city_id) = *state.city_mut(city) {
            if world.cities[city_id].planet.is_some() &&
               world.cities[city_id].location.is_some()
            {
                let new_expr = HasLocation(city);
                if can_add(&new_expr) {return Some(new_expr)};
            }

            if world.city_spaceport(city_id).is_some() {
                let new_expr = CityHasSpaceport(city);
                if can_add(&new_expr) {return Some(new_expr)};
            }
        }
    }

    // Common sense inference.
    for expr in story {
        if let ContainsSpecies(planet_a, a) = *expr {
            for expr2 in story {
                if let ContainsSpecies(planet_b, b) = *expr2 {
                    if planet_a == planet_b {
                        let new_expr = LiveOnSamePlanet(a, b);
                        if can_add(&new_expr) {return Some(new_expr)};

                        let new_expr = LiveOnSamePlanet(b, a);
                        if can_add(&new_expr) {return Some(new_expr)};
                    }
                }
            }
        }

        if let HasNumberOfSpaceports(planet, n) = *expr {
            let new_expr = HasSpaceTravel(planet, n > 0);
            if can_add(&new_expr) {return Some(new_expr)};
        }
    }

    let new_expr = Sound;
    if can_add(&Sound) {return Some(new_expr)};

    None
}

pub fn test() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateSpecies(Vatrax),
            AssignHomePlanet(Vatrax, Tellar),

            CreateCity(Eldonar),
            AssignLocation(Eldonar, Tellar, LocationName::A),
            PopulateCity(Eldonar, 100_000, Vatrax),
        ],
        vec![
            PlanetHasNumberOfPeople(Tellar, 100_000),
            Sound,
        ]
    )
}

fn main() {
    test::check(&[
            (test::create_tellar, true),
            (test::create_munos, true),
            (test::create_and_assign_orbit, true),
            (test::create_and_assign_species, true),
            (test::live_on_same_planet, true),
            (test::assign_location_to_city, true),
            (test::count_cities_on_planet, true),
            (test::city_spaceport, true),
            (test::count_spaceports_on_planet, true),
            (test::has_no_space_travel_before_creating_spaceport, true),
            // 10
            (test::has_space_travel_after_creating_spaceport, true),
            (test::has_no_space_travel_after_destroying_spaceport, true),
            (test::has_space_travel_after_rebuilding_spaceport, true),
            (test::sum_population_for_planet, true),
        ]);

    let (start, goal) = test();
    let order_constraints = vec![
    ];

    let res = solve_and_reduce(
        &start,
        &goal,
        &[],
        &order_constraints,
        infer,
    );
    if res.is_ok() {
        println!("OK");
    } else {
        println!("ERROR");
    }
    match res {
        Ok(ref res) | Err(ref res) => {
            for r in res {
                println!("{:?}", r);
            }
        }
    }
}
