use *;

pub fn create_tellar() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
        ],
        vec![
            ContainsPlanets,
            ContainsPlanet(Tellar),
            Sound,
        ]
    )
}

pub fn create_munos() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Munos),
        ],
        vec![
            ContainsPlanets,
            ContainsPlanet(Munos),
            Sound,
        ]
    )
}

pub fn create_and_assign_orbit() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateOrbit(OrbitName::B),
            AssignOrbit(Tellar, OrbitName::B),
        ],
        vec![
            HasOrbit(Tellar, OrbitName::B),
            Sound,
        ]
    )
}

pub fn create_and_assign_species() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateSpecies(Vatrax),
            AssignHomePlanet(Vatrax, Tellar),
        ],
        vec![
            ContainsSpecies(Tellar, Vatrax),
            Sound,
        ]
    )
}

pub fn live_on_same_planet() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateSpecies(Vatrax),
            CreateSpecies(Ralm),
            AssignHomePlanet(Vatrax, Tellar),
            AssignHomePlanet(Ralm, Tellar),
        ],
        vec![
            ContainsSpecies(Tellar, Vatrax),
            ContainsSpecies(Tellar, Ralm),
            LiveOnSamePlanet(Vatrax, Ralm),
            LiveOnSamePlanet(Ralm, Vatrax),
            Sound,
        ]
    )
}

pub fn assign_location_to_city() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateCity(Eldonar),
            AssignLocation(Eldonar, Tellar, LocationName::A),
        ],
        vec![
            HasLocation(Eldonar),
            Sound,
        ]
    )
}

pub fn count_cities_on_planet() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateCity(Eldonar),
            CreateCity(Tarat),
            AssignLocation(Eldonar, Tellar, LocationName::A),
            AssignLocation(Tarat, Tellar, LocationName::B),
        ],
        vec![
            HasLocation(Eldonar),
            HasLocation(Tarat),
            HasNumberOfCities(Tellar, 2),
            Sound,
        ]
    )
}

pub fn city_spaceport() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateCity(Eldonar),
            AssignLocation(Eldonar, Tellar, LocationName::A),
            CreateSpaceport(Tellar, LocationName::A),
        ],
        vec![
            CityHasSpaceport(Eldonar),
            Sound,
        ]
    )
}

pub fn count_spaceports_on_planet() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateCity(Eldonar),
            AssignLocation(Eldonar, Tellar, LocationName::A),
            CreateSpaceport(Tellar, LocationName::A),
            CreateSpaceport(Tellar, LocationName::B),
        ],
        vec![
            HasNumberOfSpaceports(Tellar, 2),
            Sound,
        ]
    )
}

pub fn has_no_space_travel_before_creating_spaceport() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateCity(Eldonar),
            AssignLocation(Eldonar, Tellar, LocationName::A),
        ],
        vec![
            HasNumberOfCities(Tellar, 1),
            HasSpaceTravel(Tellar, false),
            Sound,
        ]
    )
}

pub fn has_space_travel_after_creating_spaceport() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateCity(Eldonar),
            AssignLocation(Eldonar, Tellar, LocationName::A),
            CreateSpaceport(Tellar, LocationName::A),
        ],
        vec![
            HasNumberOfCities(Tellar, 1),
            HasSpaceTravel(Tellar, true),
            Sound,
        ]
    )
}

pub fn has_no_space_travel_after_destroying_spaceport() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateCity(Eldonar),
            AssignLocation(Eldonar, Tellar, LocationName::A),
            CreateSpaceport(Tellar, LocationName::A),
            DestroySpaceport(Tellar, LocationName::A),
        ],
        vec![
            HasNumberOfCities(Tellar, 1),
            HasSpaceTravel(Tellar, false),
            Sound,
        ]
    )
}

pub fn has_space_travel_after_rebuilding_spaceport() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateCity(Eldonar),
            AssignLocation(Eldonar, Tellar, LocationName::A),
            CreateSpaceport(Tellar, LocationName::A),
            DestroySpaceport(Tellar, LocationName::A),
            RebuildSpaceport(Tellar, LocationName::A),
        ],
        vec![
            HasNumberOfCities(Tellar, 1),
            HasSpaceTravel(Tellar, true),
            Sound,
        ]
    )
}

pub fn sum_population_for_planet() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),
            CreateSpecies(Vatrax),
            CreateCity(Eldonar),
            AssignLocation(Eldonar, Tellar, LocationName::A),
            PopulateCity(Eldonar, 100_000, Vatrax),
            CreateCity(Tarat),
            AssignLocation(Tarat, Tellar, LocationName::B),
            PopulateCity(Tarat, 200_000, Vatrax),
        ],
        vec![
            PlanetHasNumberOfPeople(Tellar, 300_000),
            Sound,
        ]
    )
}

pub fn create_weapon() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(XV43),
        ],
        vec![
            ContainsWeapons,
            Sound,
        ]
    )
}

pub fn assign_weapon() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(XV43),
            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),
        ],
        vec![
            ContainsWeapons,
            ContainsPlayers,
            HasWeapon(Alice, XV43),
            Sound,
        ]
    )
}

pub fn hand_is_empty_after_dropping_weapon() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(XV43),
            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),
            DropWeapon(Alice, Hand::Left),
        ],
        vec![
            ContainsWeapons,
            ContainsPlayers,
            HandEmpty(Alice, Hand::Left),
            Sound,
        ]
    )
}

pub fn has_no_weapons_after_dropping_both_weapons() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(XV43),
            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),
            AssignWeapon(Alice, XV43, Hand::Right),
            DropWeapon(Alice, Hand::Left),
            DropWeapon(Alice, Hand::Right),
        ],
        vec![
            HasWeapons(Alice, false),
            Sound,
        ]
    )
}

pub fn has_weapon_after_dropping_only_one_weapon() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(XV43),
            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),
            AssignWeapon(Alice, XV43, Hand::Right),
            DropWeapon(Alice, Hand::Left),
        ],
        vec![
            HasWeapons(Alice, true),
            Sound,
        ]
    )
}

pub fn all_players_have_species() -> (Vec<Expr>, Vec<Expr>) {
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

pub fn all_players_have_species_and_weapons() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateSpecies(Vatrax),
            CreateSpecies(Ralm),
            CreateWeapon(XV43),
            CreatePlayer(Alice),
            AssignSpecies(Alice, Vatrax),
            AssignWeapon(Alice, XV43, Hand::Left),
            CreatePlayer(Bob),
            AssignSpecies(Bob, Ralm),
            AssignWeapon(Bob, XV43, Hand::Right),
        ],
        vec![
            AllPlayersHaveSpecies(true),
            AllPlayersHaveWeapons(true),
            Sound,
        ]
    )
}

pub fn spawning_planet() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            // Create two planets with two species.
            CreatePlanet(Tellar),
            CreatePlanet(Munos),
            CreateSpecies(Vatrax),
            CreateSpecies(Ralm),
            AssignHomePlanet(Vatrax, Tellar),
            AssignHomePlanet(Ralm, Munos),

            // Create two players.
            CreateWeapon(XV43),
            CreatePlayer(Alice),
            AssignSpecies(Alice, Vatrax),
            AssignWeapon(Alice, XV43, Hand::Left),
            CreatePlayer(Bob),
            AssignSpecies(Bob, Ralm),
            AssignWeapon(Bob, XV43, Hand::Right),
        ],
        vec![
            AllPlayersHaveSpecies(true),
            AllPlayersHaveWeapons(true),
            SpawningPlanet(Alice, Tellar),
            SpawningPlanet(Bob, Munos),
            Sound,
        ]
    )
}

pub fn out_of_game_when_killed_and_spawning_planet_is_destroyed() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            // Create two planets with two species.
            CreatePlanet(Tellar),
            CreatePlanet(Munos),
            CreateSpecies(Vatrax),
            CreateSpecies(Ralm),
            AssignHomePlanet(Vatrax, Tellar),
            AssignHomePlanet(Ralm, Munos),

            // Create two players.
            CreateWeapon(XV43),
            CreatePlayer(Alice),
            AssignSpecies(Alice, Vatrax),
            AssignWeapon(Alice, XV43, Hand::Left),
            CreatePlayer(Bob),
            AssignSpecies(Bob, Ralm),
            AssignWeapon(Bob, XV43, Hand::Right),

            // Alice's spawning planet gets destroyed and then she gets killed.
            DestroyPlanet(Tellar),
            Kill(Alice),
        ],
        vec![
            OutOfGame(Alice, true),
            OutOfGame(Bob, false),
            Sound,
        ]
    )
}

pub fn not_out_of_game_when_just_killed() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            // Create two planets with two species.
            CreatePlanet(Tellar),
            CreatePlanet(Munos),
            CreateSpecies(Vatrax),
            CreateSpecies(Ralm),
            AssignHomePlanet(Vatrax, Tellar),
            AssignHomePlanet(Ralm, Munos),

            // Create two players.
            CreateWeapon(XV43),
            CreatePlayer(Alice),
            AssignSpecies(Alice, Vatrax),
            AssignWeapon(Alice, XV43, Hand::Left),
            CreatePlayer(Bob),
            AssignSpecies(Bob, Ralm),
            AssignWeapon(Bob, XV43, Hand::Right),

            // Alice's spawning planet gets destroyed and then she gets killed.
            Kill(Alice),
        ],
        vec![
            OutOfGame(Alice, false),
            OutOfGame(Bob, false),
            Sound,
        ]
    )
}

pub fn death_match_winner() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            // Create two planets with two species.
            CreatePlanet(Tellar),
            CreatePlanet(Munos),
            CreateSpecies(Vatrax),
            CreateSpecies(Ralm),
            AssignHomePlanet(Vatrax, Tellar),
            AssignHomePlanet(Ralm, Munos),

            // Create two players.
            CreateWeapon(XV43),
            CreatePlayer(Alice),
            AssignSpecies(Alice, Vatrax),
            AssignWeapon(Alice, XV43, Hand::Left),
            CreatePlayer(Bob),
            AssignSpecies(Bob, Ralm),
            AssignWeapon(Bob, XV43, Hand::Right),

            // Alice's spawning planet gets destroyed and then she gets killed.
            DestroyPlanet(Tellar),
            Kill(Alice),
        ],
        vec![
            OutOfGame(Alice, true),
            OutOfGame(Bob, false),
            NumberOfPlayersLeft(1),
            DeathMatchWinner(Bob),
            Sound,
        ]
    )
}

pub fn team_match_winner() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            // Create two planets with two species.
            CreatePlanet(Tellar),
            CreatePlanet(Munos),
            CreateSpecies(Vatrax),
            CreateSpecies(Ralm),
            AssignHomePlanet(Vatrax, Tellar),
            AssignHomePlanet(Ralm, Munos),

            // Create three players on two teams.
            CreateWeapon(XV43),
            CreatePlayer(Alice),
            AssignSpecies(Alice, Vatrax),
            AssignWeapon(Alice, XV43, Hand::Left),
            CreatePlayer(Bob),
            AssignSpecies(Bob, Ralm),
            AssignWeapon(Bob, XV43, Hand::Right),
            CreatePlayer(Carl),
            AssignSpecies(Carl, Ralm),
            AssignWeapon(Carl, XV43, Hand::Left),

            // Alice's spawning planet gets destroyed and then she gets killed.
            DestroyPlanet(Tellar),
            Kill(Alice),
        ],
        vec![
            OutOfGame(Alice, true),
            OutOfGame(Bob, false),
            OutOfGame(Carl, false),
            NumberOfPlayersLeft(2),
            TeamMatchWinner(Ralm),
            Sound,
        ]
    )
}

pub fn number_of_weapon_users() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            // Create some players and assign different weapons.
            CreateWeapon(XV43),
            CreateWeapon(TT180),
            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),
            CreatePlayer(Bob),
            AssignWeapon(Bob, XV43, Hand::Right),
            CreatePlayer(Carl),
            AssignWeapon(Carl, TT180, Hand::Left),
        ],
        vec![
            NumberOfWeaponUsers(XV43, 2),
            NumberOfWeaponUsers(TT180, 1),
            Sound,
        ]
    )
}

pub fn create_spaceship() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateSpaceship(Folkum),
        ],
        vec![
            Sound,
        ]
    )
}

pub fn assign_canon() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateSpaceship(Folkum),
            CreateCanon(SR6),
            AssignCanon(Folkum, SR6, CanonSlot::Front1),
        ],
        vec![
            Sound,
        ]
    )
}

pub fn set_canon_firepower() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateSpaceship(Folkum),
            CreateCanon(SR6),
            SetCanonFirepower(SR6, 100),
            AssignCanon(Folkum, SR6, CanonSlot::Front1),
        ],
        vec![
            Sound,
        ]
    )
}

pub fn set_weapon_firepower() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(TT180),
            SetWeaponFirepower(TT180, 100),
        ],
        vec![
            Sound,
        ]
    )
}

pub fn destroy_planet_with_planet_destroyer_weapon() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(AM0),
            SetWeaponFirepower(AM0, 10000),
            SetWeaponPlanetDestroyer(AM0, true),

            CreatePlanet(Tellar),
            CreatePlayer(Alice),
            AssignWeapon(Alice, AM0, Hand::Left),
            ShootAtPlanet(Alice, Hand::Left, Tellar),
        ],
        vec![
            IsPlanetDestroyed(Tellar, true),
            Sound,
        ]
    )
}

pub fn shoot_at_player() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(XV43),
            SetWeaponFirepower(XV43, 100),

            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),
            CreatePlayer(Bob),
            SetLife(Bob, 200),

            ShootAtPlayer(Alice, Hand::Left, Bob),
        ],
        vec![
            HasLife(Bob, 100),
            Sound,
        ]
    )
}

pub fn shoot_player_dead() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(XV43),
            SetWeaponFirepower(XV43, 100),

            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),
            CreatePlayer(Bob),
            SetLife(Bob, 200),

            ShootAtPlayer(Alice, Hand::Left, Bob),
            ShootAtPlayer(Alice, Hand::Left, Bob),
        ],
        vec![
            HasLife(Bob, 0),
            NumberOfPlayersLeft(1),
            IsDead(Bob, true),
            IsDead(Alice, false),
            Sound,
        ]
    )
}

pub fn recharge_when_shooting_at_nothing() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(XV43),
            SetWeaponFirepower(XV43, 100),
            // Wait 1 second between each shot.
            SetWeaponRechargeMilliseconds(XV43, 1000),

            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),

            ShootAtNothing(Alice, Hand::Left),
        ],
        vec![
            MillisecondsToRecharge(Alice, Hand::Left, 1000),
            Sound,
        ]
    )
}

pub fn recharge_when_shooting_at_player() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(XV43),
            SetWeaponFirepower(XV43, 100),
            // Wait 1 second between each shot.
            SetWeaponRechargeMilliseconds(XV43, 1000),

            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),
            CreatePlayer(Bob),
            SetLife(Bob, 1000),

            ShootAtPlayer(Alice, Hand::Left, Bob),
        ],
        vec![
            MillisecondsToRecharge(Alice, Hand::Left, 1000),
            Sound,
        ]
    )
}

pub fn recharge_when_shooting_at_planet() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),

            CreateWeapon(XV43),
            SetWeaponPlanetDestroyer(XV43, true),
            // Wait 1 second between each shot.
            SetWeaponRechargeMilliseconds(XV43, 1000),

            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),

            ShootAtPlanet(Alice, Hand::Left, Tellar),
        ],
        vec![
            MillisecondsToRecharge(Alice, Hand::Left, 1000),
            Sound,
        ]
    )
}

pub fn recharge_when_shooting_at_planet_even_weapon_is_not_planet_destroyer()
-> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreatePlanet(Tellar),

            CreateWeapon(XV43),
            SetWeaponPlanetDestroyer(XV43, false),
            // Wait 1 second between each shot.
            SetWeaponRechargeMilliseconds(XV43, 1000),

            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),

            ShootAtPlanet(Alice, Hand::Left, Tellar),
        ],
        vec![
            MillisecondsToRecharge(Alice, Hand::Left, 1000),
            Sound,
        ]
    )
}

pub fn cannot_shoot_while_weapon_is_recharging() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(XV43),
            // Wait 1 second between each shot.
            SetWeaponRechargeMilliseconds(XV43, 1000),

            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),

            ShootAtNothing(Alice, Hand::Left),
        ],
        vec![
            MillisecondsToRecharge(Alice, Hand::Left, 1000),
            CanShoot(Alice, Hand::Left, false),
            Sound,
        ]
    )
}

pub fn can_shoot_when_weapon_is_recharged() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(XV43),
            // Wait 1 second between each shot.
            SetWeaponRechargeMilliseconds(XV43, 1000),

            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),

            ShootAtNothing(Alice, Hand::Left),
            RechargeMilliseconds(Alice, Hand::Left, 1000),
        ],
        vec![
            MillisecondsToRecharge(Alice, Hand::Left, 0),
            CanShoot(Alice, Hand::Left, true),
            Sound,
        ]
    )
}

pub fn recharge_milliseconds_all_weapons() -> (Vec<Expr>, Vec<Expr>) {
    (
        vec![
            CreateWeapon(XV43),
            SetWeaponFirepower(XV43, 100),
            // Wait 1 second between each shot.
            SetWeaponRechargeMilliseconds(XV43, 1000),

            CreatePlayer(Alice),
            AssignWeapon(Alice, XV43, Hand::Left),
            CreatePlayer(Bob),
            AssignWeapon(Bob, XV43, Hand::Right),

            ShootAtPlayer(Alice, Hand::Left, Bob),
            RechargeMillisecondsAllWeapons(200),
            ShootAtPlayer(Bob, Hand::Right, Alice),
            RechargeMillisecondsAllWeapons(800),
        ],
        vec![
            MillisecondsToRecharge(Alice, Hand::Left, 0),
            MillisecondsToRecharge(Bob, Hand::Right, 200),
            CanShoot(Alice, Hand::Left, true),
            CanShoot(Bob, Hand::Right, false),
            Sound,
        ]
    )
}

/// Checks a list of tests.
pub fn check(fs: &[(fn() -> (Vec<Expr>, Vec<Expr>), bool)]) {
    for (i, &(f, ok)) in fs.iter().enumerate() {
        let (start, goal) = f();
        let order_constraints = vec![];

        // Use `solve` because it's faster than reduction.
        let res = solve(
            &start,
            &goal,
            &[],
            &order_constraints,
            infer,
        );
        if res.is_ok() != ok {
            panic!("Failed check `{}`", i);
        }
    }
}
