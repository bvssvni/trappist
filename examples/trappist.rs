extern crate monotonic_solver;

use monotonic_solver::{solve, solve_and_reduce};

use std::collections::HashSet;

use Expr::*;
use PlanetName::*;
use SpeciesName::*;
use CityName::*;
use WeaponName::*;
use PlayerName::*;
use world::*;
use state::*;
use names::*;

mod world;
mod state;
mod names;
pub mod test;

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
    /// Create new weapon.
    CreateWeapon(WeaponName),
    /// Create new player.
    CreatePlayer(PlayerName),
    /// Assign an orbit to planet.
    AssignOrbit(PlanetName, OrbitName),
    /// Assign a home planet to species.
    AssignHomePlanet(SpeciesName, PlanetName),
    /// Assign a planet location to city.
    AssignLocation(CityName, PlanetName, LocationName),
    /// Assign a weapon to player.
    AssignWeapon(PlayerName, WeaponName, Hand),
    /// Assign species to player.
    AssignSpecies(PlayerName, SpeciesName),
    /// Destroy spaceport.
    DestroySpaceport(PlanetName, LocationName),
    /// Rebuild spaceport.
    RebuildSpaceport(PlanetName, LocationName),
    /// Populates city with a number of people.
    PopulateCity(CityName, u64, SpeciesName),
    /// Drops player's weapon by hand.
    DropWeapon(PlayerName, Hand),
    /// The story works out.
    Sound,
    /// The world contains planets.
    ContainsPlanets,
    /// The world contains weapons.
    ContainsWeapons,
    /// The world contains players.
    ContainsPlayers,
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
    /// A player carries a weapon.
    HasWeapon(PlayerName, WeaponName),
    /// Player's hand is empty.
    HandEmpty(PlayerName, Hand),
    /// Whether a player has any weapons.
    HasWeapons(PlayerName, bool),
    /// All players have an assigned species.
    AllPlayersHaveSpecies(bool),
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

        if let CreateWeapon(name) = *expr {
            state.create_weapon(name, world);
        }

        if let CreatePlayer(name) = *expr {
            state.create_player(name, world);
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

        if let AssignWeapon(player, weapon, hand) = *expr {
            if !state.assign_weapon(player, weapon, hand, world).is_ok() {
                return None;
            }
        }

        if let AssignSpecies(player, species) = *expr {
            if !state.assign_species(player, species, world).is_ok() {
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

        if let DropWeapon(player, hand) = *expr {
            if !state.drop_weapon(player, hand, world).is_ok() {
                return None;
            }
        }
    }

    if world.planets.len() > 0 {
        let new_expr = ContainsPlanets;
        if can_add(&new_expr) {return Some(new_expr)};
    }

    if world.weapons.len() > 0 {
        let new_expr = ContainsWeapons;
        if can_add(&new_expr) {return Some(new_expr)};
    }

    if world.players.len() > 0 {
        let new_expr = ContainsPlayers;
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

    for &player in PlayerName::all() {
        if let Some(player_id) = *state.player_mut(player) {
            if world.players[player_id].left_weapon.is_none() {
                let new_expr = HandEmpty(player, Hand::Left);
                if can_add(&new_expr) {return Some(new_expr)};
            }
            if world.players[player_id].right_weapon.is_none() {
                let new_expr = HandEmpty(player, Hand::Right);
                if can_add(&new_expr) {return Some(new_expr)};
            }

            for &weapon in WeaponName::all() {
                if let Some(id) = *state.weapon_mut(weapon) {
                    if let Some(weapon_id) = world.players[player_id].left_weapon {
                        if id == weapon_id {
                            let new_expr = HasWeapon(player, weapon);
                            if can_add(&new_expr) {return Some(new_expr)};
                        }
                    }

                    if let Some(weapon_id) = world.players[player_id].right_weapon {
                        if id == weapon_id {
                            let new_expr = HasWeapon(player, weapon);
                            if can_add(&new_expr) {return Some(new_expr)};
                        }
                     }
                }
            }

            let has_weapons = world.players[player_id].left_weapon.is_some() ||
                              world.players[player_id].right_weapon.is_some();
            let new_expr = HasWeapons(player, has_weapons);
            if can_add(&new_expr) {return Some(new_expr)};
        }
    }

    let new_expr = AllPlayersHaveSpecies(world.all_players_have_species());
    if can_add(&new_expr) {return Some(new_expr)};

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
            CreateSpecies(Vatrax),
            CreateSpecies(Ralm),
            CreatePlayer(Alice),
            AssignSpecies(Alice, Vatrax),
            CreatePlayer(Bob),
            AssignSpecies(Bob, Ralm),
        ],
        vec![
            AllPlayersHaveSpecies(true),
            Sound,
        ]
    )
}

fn main() {
    test::check(&[
            // 0
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
            (test::create_weapon, true),
            (test::assign_weapon, true),
            (test::hand_is_empty_after_dropping_weapon, true),
            (test::has_no_weapons_after_dropping_both_weapons, true),
            (test::has_weapon_after_dropping_only_one_weapon, true),
            (test::all_players_have_species, true),
            // 20
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
