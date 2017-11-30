use *;

pub fn infer(cache: &HashSet<Expr>, filter_cache: &HashSet<Expr>, story: &[Expr]) -> Option<Expr> {
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

        if let CreateSpaceship(name) = *expr {
            state.create_spaceship(name, world);
        }

        if let CreateCanon(name) = *expr {
            state.create_canon(name, world);
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

        if let AssignCanon(spaceship, canon, canon_slot) = *expr {
            if !state.assign_canon(spaceship, canon, canon_slot, world).is_ok() {
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

        if let DestroyPlanet(planet) = *expr {
            if !state.destroy_planet(planet, world).is_ok() {
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

        if let Kill(player) = *expr {
            if !state.kill(player, world).is_ok() {
                return None;
            }
        }

        if let SetLife(player, life) = *expr {
            if let Some(player_id) = *state.player_mut(player) {
                world.players[player_id].life = life;
            }
        }

        if let SetWeaponFirepower(weapon, firepower) = *expr {
            if let Some(weapon_id) = *state.weapon_mut(weapon) {
                world.weapons[weapon_id].firepower = firepower;
            }
        }

        if let SetWeaponRechargeMilliseconds(weapon, recharge_milliseconds) = *expr {
            if let Some(weapon_id) = *state.weapon_mut(weapon) {
                world.weapons[weapon_id].recharge_milliseconds = recharge_milliseconds;
            }
        }

        if let SetWeaponPlanetDestroyer(weapon, value) = *expr {
            if let Some(weapon_id) = *state.weapon_mut(weapon) {
                world.weapons[weapon_id].planet_destroyer = value;
            }
        }

        if let SetCanonFirepower(canon, firepower) = *expr {
            if let Some(canon_id) = *state.canon_mut(canon) {
                world.canons[canon_id].firepower = firepower;
            }
        }

        if let ShootAtPlanet(player, hand, planet) = *expr {
            if let Some(player_id) = *state.player_mut(player) {
                if let Some(planet_id) = *state.planet_mut(planet) {
                    world.shoot_at_planet(player_id, hand, planet_id);
                }
            }
        }

        if let ShootAtPlayer(shooter, hand, target) = *expr {
            if let Some(shooter_id) = *state.player_mut(shooter) {
                if let Some(target_id) = *state.player_mut(target) {
                    world.shoot_at_player(shooter_id, hand, target_id);
                }
            }
        }

        if let ShootAtNothing(player, hand) = *expr {
            if let Some(player_id) = *state.player_mut(player) {
                world.shoot_at_nothing(player_id, hand);
            }
        }

        if let RechargeMilliseconds(player, hand, recharge_milliseconds) = *expr {
            if let Some(player_id) = *state.player_mut(player) {
                world.recharge_milliseconds(player_id, hand, recharge_milliseconds);
            }
        }

        if let RechargeMillisecondsAllWeapons(recharge_milliseconds) = *expr {
            world.recharge_milliseconds_all_weapons(recharge_milliseconds);
        }

        if let Spawn(player) = *expr {
            if let Some(player_id) = *state.player_mut(player) {
                world.spawn(player_id);
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

    let new_expr = NumberOfPlayersLeft(world.number_of_players_left());
    if can_add(&new_expr) {return Some(new_expr)};

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

            let new_expr = IsPlanetDestroyed(name, world.planets[planet_id].destroyed);
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

            if let Some(id) = world.team_match_winner() {
                if id == species_id {
                    let new_expr = TeamMatchWinner(species);
                    if can_add(&new_expr) {return Some(new_expr)};
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

    let death_match_winner = world.death_match_winner();
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

            let new_expr = HasWeapons(player, world.players[player_id].has_weapons());
            if can_add(&new_expr) {return Some(new_expr)};

            if let Some(planet_id) = world.players[player_id].spawning_planet(world) {
                for &planet in PlanetName::all() {
                    if let Some(id) = *state.planet_mut(planet) {
                        if id == planet_id {
                            let new_expr = SpawningPlanet(player, planet);
                            if can_add(&new_expr) {return Some(new_expr)};
                        }
                    }
                }
            }

            let out_of_game = world.players[player_id].out_of_game(world);
            let new_expr = OutOfGame(player, out_of_game);
            if can_add(&new_expr) {return Some(new_expr)};

            if let Some(id) = death_match_winner {
                if id == player_id {
                    let new_expr = DeathMatchWinner(player);
                    if can_add(&new_expr) {return Some(new_expr)};
                }
            }

            let new_expr = HasLife(player, world.players[player_id].life);
            if can_add(&new_expr) {return Some(new_expr)};

            let new_expr = IsDead(player, world.players[player_id].dead);
            if can_add(&new_expr) {return Some(new_expr)};

            let left_recharge_ms = world.players[player_id].left_recharge_milliseconds;
            let new_expr = MillisecondsToRecharge(player, Hand::Left, left_recharge_ms);
            if can_add(&new_expr) {return Some(new_expr)};

            let new_expr = CanShoot(player, Hand::Left, left_recharge_ms == 0);
            if can_add(&new_expr) {return Some(new_expr)};

            let right_recharge_ms = world.players[player_id].right_recharge_milliseconds;
            let new_expr = MillisecondsToRecharge(player, Hand::Right, right_recharge_ms);
            if can_add(&new_expr) {return Some(new_expr)};

            let new_expr = CanShoot(player, Hand::Right, right_recharge_ms == 0);
            if can_add(&new_expr) {return Some(new_expr)};

            if let Some(planet_id) = world.players[player_id].on_planet {
                for &planet in PlanetName::all() {
                    if let Some(id) = *state.planet_mut(planet) {
                        if id == planet_id {
                            let new_expr = IsOnPlanet(player, planet);
                            if can_add(&new_expr) {return Some(new_expr)};
                        }
                    }
                }
            }
        }
    }

    for &weapon in WeaponName::all() {
        if let Some(weapon_id) = *state.weapon_mut(weapon) {
            let new_expr = NumberOfWeaponUsers(weapon, world.number_of_weapon_users(weapon_id));
            if can_add(&new_expr) {return Some(new_expr)};
        }
    }

    let new_expr = AllPlayersHaveSpecies(world.all_players_have_species());
    if can_add(&new_expr) {return Some(new_expr)};

    let new_expr = AllPlayersHaveWeapons(world.all_players_have_weapons());
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
