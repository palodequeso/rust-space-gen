use serde::{Deserialize, Serialize};
use rand_distr::{Normal, Distribution};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Deserialize, Serialize, Debug)]
pub enum BodyTypes {
    Star,
    Planet,
    Moon,
    Asteroid,
    Comet,
    DwarfPlanet,
    DwarfMoon,
    BlackHole,
    Other,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum StarTypes {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
    Neutron,
    Pulsar,
    Supernova,
    Other,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum PlanetTypes {
    GasGiant,
    Terrestrial,
    IceGiant,
    Rocky,
    Barren,
    Desert,
    Ocean,
    Other,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum HardBodyTypes {
    Rocky,
    Ice,
    Water,
    Ammonia,
    Iron,
    Other,
}

pub trait Body {
   fn get_type(&self) -> BodyTypes;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Star {
    pub name: String,
    pub star_type: StarTypes,
    pub seed: u64,
}

impl Star {
    pub fn new(name: String, star_type: StarTypes, seed: u64) -> Star {
        Star {
            name,
            star_type,
            seed,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Moon {
    pub name: String,
    pub hard_body_type: HardBodyTypes,
    pub seed: u64,
    pub radius: f64,
    pub mass: f64,
}

impl Moon {
    pub fn new(name: String, hard_body_type: HardBodyTypes, seed: u64, radius: f64, mass: f64) -> Moon {
        Moon {
            name,
            hard_body_type,
            seed,
            radius,
            mass,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Planet {
    pub name: String,
    pub planet_type: PlanetTypes,
    pub seed: u64,
    pub radius: f64,
    pub mass: f64,
    pub moons: Vec<Moon>,
}

impl Planet {
    pub fn new(name: String, planet_type: PlanetTypes, seed: u64, radius: f64, mass: f64) -> Planet {
        Planet {
            name,
            planet_type,
            seed,
            radius,
            mass,
            moons: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DwarfPlanet {
    pub name: String,
    pub hard_body_type: HardBodyTypes,
    pub seed: u64,
    pub radius: f64,
    pub mass: f64,
}

impl DwarfPlanet {
  pub fn new(name: String, hard_body_type: HardBodyTypes, seed: u64, radius: f64, mass: f64) -> DwarfPlanet {
      DwarfPlanet {
          name,
          hard_body_type,
          seed,
          radius,
          mass,
      }
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DwarfMoon {
    pub name: String,
    pub hard_body_type: HardBodyTypes,
    pub seed: u64,
    pub radius: f64,
    pub mass: f64,
}

impl DwarfMoon {
    pub fn new(name: String, hard_body_type: HardBodyTypes, seed: u64, radius: f64, mass: f64) -> DwarfMoon {
        DwarfMoon {
            name,
            hard_body_type,
            seed,
            radius,
            mass,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GalacticPosition {
    pub x: f64, // in light years
    pub y: f64, // in light years
}

impl GalacticPosition {
    pub fn new(x: f64, y: f64) -> GalacticPosition {
        GalacticPosition { x, y }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum GalaxyType {
    Spiral,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Galaxy {
    pub name: String,
    pub galaxy_type: GalaxyType,
    pub seed: u64,
}

impl Galaxy {
    pub fn new(name: String, galaxy_type: GalaxyType, seed: u64) -> Galaxy {
        Galaxy {
            name,
            galaxy_type,
            seed,
        }
    }

    pub fn get_nearby_stars(&self, pos: GalacticPosition) -> Vec<Star> {
        let mut rng = StdRng::seed_from_u64(self.seed); // <- Here we set the seed
        let mut stars = Vec::new();
        for _ in 0..10 {
            let star_seed = rng.gen::<u64>();
            stars.push(Star::new(
                "".to_string(),
                StarTypes::O,
                star_seed,
            ));
        }
        stars
    }
}
