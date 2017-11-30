extern crate monotonic_solver;

use monotonic_solver::{solve, solve_and_reduce};

use std::collections::HashSet;

use Expr::*;
use PlanetName::*;
use SpeciesName::*;
use CityName::*;
use WeaponName::*;
use PlayerName::*;
use SpaceshipName::*;
use CanonName::*;
use world::*;
use state::*;
use names::*;
use inference::infer;

mod world;
mod state;
mod names;
mod inference;
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
    /// Create new spaceship.
    CreateSpaceship(SpaceshipName),
    /// Creates new canon.
    CreateCanon(CanonName),
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
    /// Assign a canon to spaceship.
    AssignCanon(SpaceshipName, CanonName, CanonSlot),
    /// Spawn player.
    Spawn(PlayerName),
    /// Destroy spaceport.
    DestroySpaceport(PlanetName, LocationName),
    /// Destroy planet.
    DestroyPlanet(PlanetName),
    /// Rebuild spaceport.
    RebuildSpaceport(PlanetName, LocationName),
    /// Populates city with a number of people.
    PopulateCity(CityName, u64, SpeciesName),
    /// Drops player's weapon by hand.
    DropWeapon(PlayerName, Hand),
    /// Player shoots at planet.
    ShootAtPlanet(PlayerName, Hand, PlanetName),
    /// Shoot at another player.
    ShootAtPlayer(PlayerName, Hand, PlayerName),
    /// Shoot at nothing.
    ShootAtNothing(PlayerName, Hand),
    /// Kills player.
    Kill(PlayerName),
    /// Recharges weapon with an amount of milliseconds.
    RechargeMilliseconds(PlayerName, Hand, u16),
    /// Recharges all weapons with an amount of milliseconds.
    RechargeMillisecondsAllWeapons(u16),
    /// Set player life.
    SetLife(PlayerName, u16),
    /// Set weapon firepower.
    SetWeaponFirepower(WeaponName, u16),
    /// Set weapon recharge in milliseconds.
    SetWeaponRechargeMilliseconds(WeaponName, u16),
    /// Set weapon to be planet destroyer.
    SetWeaponPlanetDestroyer(WeaponName, bool),
    /// Set canon firepower.
    SetCanonFirepower(CanonName, u16),
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
    /// All players have some weapon.
    AllPlayersHaveWeapons(bool),
    /// Which planet a player will spawn from.
    SpawningPlanet(PlayerName, PlanetName),
    /// Whether player is out of game.
    OutOfGame(PlayerName, bool),
    /// The number of players left.
    NumberOfPlayersLeft(usize),
    /// Which player won death match.
    DeathMatchWinner(PlayerName),
    /// Which species won death match.
    TeamMatchWinner(SpeciesName),
    /// The number of users per weapon.
    NumberOfWeaponUsers(WeaponName, usize),
    /// Whether planet is destroyed.
    IsPlanetDestroyed(PlanetName, bool),
    /// Whether player is dead.
    IsDead(PlayerName, bool),
    /// How much life a player has.
    HasLife(PlayerName, u16),
    /// How long time it takes for hand weapon to recharge.
    MillisecondsToRecharge(PlayerName, Hand, u16),
    /// Whether player can shoot with weapon.
    CanShoot(PlayerName, Hand, bool),
    /// Which planet player is currently on.
    IsOnPlanet(PlayerName, PlanetName),
}

pub fn test() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateSpecies(Ralm),
            AssignHomePlanet(Ralm, Tellar),

            CreatePlayer(Alice),
            AssignSpecies(Alice, Ralm),
            Spawn(Alice),
        ],
        vec![
            IsOnPlanet(Alice, Tellar),
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
            (test::all_players_have_species_and_weapons, true),
            (test::spawning_planet, true),
            (test::out_of_game_when_killed_and_spawning_planet_is_destroyed, true),
            (test::not_out_of_game_when_just_killed, true),
            (test::death_match_winner, true),
            (test::team_match_winner, true),
            (test::number_of_weapon_users, true),
            (test::create_spaceship, true),
            (test::assign_canon, true),
            (test::set_canon_firepower, true),
            // 30
            (test::set_weapon_firepower, true),
            (test::destroy_planet_with_planet_destroyer_weapon, true),
            (test::shoot_at_player, true),
            (test::shoot_player_dead, true),
            (test::recharge_when_shooting_at_nothing, true),
            (test::recharge_when_shooting_at_player, true),
            (test::recharge_when_shooting_at_planet, true),
            (test::recharge_when_shooting_at_planet_even_weapon_is_not_planet_destroyer, true),
            (test::cannot_shoot_while_weapon_is_recharging, true),
            (test::can_shoot_when_weapon_is_recharged, true),
            // 40
            (test::recharge_milliseconds_all_weapons, true),
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
