use *;

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
    /// Reference to the XV43 weapon.
    xv43: Option<usize>,
    /// Reference to the Alice player.
    alice: Option<usize>,
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
            xv43: None,
            alice: None,
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

    pub fn weapon_mut(&mut self, weapon: WeaponName) -> &mut Option<usize> {
        match weapon {
            XV43 => &mut self.xv43,
        }
    }

    pub fn create_weapon(&mut self, weapon: WeaponName, world: &mut World) {
        let id = world.create_weapon();
        *self.weapon_mut(weapon) = Some(id);
    }

    pub fn player_mut(&mut self, player: PlayerName) -> &mut Option<usize> {
        match player {
            Alice => &mut self.alice,
        }
    }

    pub fn create_player(&mut self, player: PlayerName, world: &mut World) {
        let id = world.create_player();
        *self.player_mut(player) = Some(id);
    }

    pub fn assign_weapon(
        &mut self,
        player: PlayerName,
        weapon: WeaponName,
        hand: Hand,
        world: &mut World
    ) -> Result<(), ()> {
        let player_id = self.player_mut(player).ok_or(())?;
        let weapon_id = self.weapon_mut(weapon).ok_or(())?;
        match hand {
            Hand::Left => world.players[player_id].left_weapon = Some(weapon_id),
            Hand::Right => world.players[player_id].right_weapon = Some(weapon_id),
        }
        Ok(())
    }

    pub fn drop_weapon(
        &mut self,
        player: PlayerName,
        hand: Hand,
        world: &mut World
    ) -> Result<(), ()> {
        let player_id = self.player_mut(player).ok_or(())?;
        match hand {
            Hand::Left => world.players[player_id].left_weapon = None,
            Hand::Right => world.players[player_id].right_weapon = None,
        }
        Ok(())
    }
}
