use std::fs::File;
use std::io::prelude::*;
use std::stream::FromIter;
use actix_web::middleware::normalize::TrailingSlash;
use serde;
use serde_json;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Particle {
    weight: Option<f64>,
    average: Option<f64>,
    particles: Vec<Particle>,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            weight: Some(1 as f64),
            average: None,
            particles: Vec::new(),
        }
    }
}

impl Particle {
    fn set_weight(&mut self, v: f64) {
        self.weight = Some(v);
    }

    fn get_weight(& self) -> f64 {
        match self.weight {
            Some(w) => w,
            None => 1 as f64,
        }
    }

    fn get_average(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        self.average
    }

    fn new(&mut self, particles: Vec<Particle>, weight: f64) -> Self {
        let mut numerator = 0 as f64;
        let mut summed_weights  = 0 as f64;

        for particle in particles.as_slice() {
            numerator += particle.get_average() * particle.get_weight();
            summed_weights += particle.get_weight();
        }

        Particle {
            weight: None,
            average: Some(numerator / summed_weights),
            particles: particles,
        }
    }


    fn update(&mut self) -> Self {
        let mut numerator = 0 as f64;
        let mut summed_weights  = 0 as f64;

        for particle in self.particles.as_slice() {
            particle.update();
            numerator += particle.get_average() * particle.get_weight();
            summed_weights += particle.get_weight();
        }

        Particle {
            weight: None,
            average: Some(numerator / summed_weights),
            particles: particles,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExameGroup {
    name: String,
    particle: Particle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UEGroup {
    name: String,
    particle: Particle,
    passed: Option<bool>,
    requirement: f64,
}

impl Default for UEGroup {
   fn default() -> Self {
        UEGroup {
            name: "",
            particle: Particle::empty(),
            passed: None,
            requirement: 10 as f64,
        }
   }
}

impl UEGroup {
    fn determmin_if_passed(&mut self) -> bool {
        unimplemented!("Determin if the student passed");
    }
}

impl FromIterator<ExameGroup> for UEGroup {
    fn from_iter<I: IntoIterator<Item=ExameGroup>>(iter: I) -> Self {
        let mut u = Self::empty();

        for exam in iter {
            u.particle.particles.add(exam.particle);
        }

        u.particle.update();

        u
    }
}


trait Transmitable {
    fn export_tp_path(& self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(path)?;

        let data = serde_json::to_string(&self)?;
        file.write_all(data.as_bytes())?;

        Ok(())
    }

    fn import_from_path(path: &str) -> Result<Self, Self::Error> {
        let file = File::open(path)?;
        let data = serde_json::from_reader(file)?;
        Ok(data)
    }
}

impl Transmitable for Particle {}
impl Transmitable for ExameGroup {}
impl Transmitable for UEGroup {}


fn main() { 
    let path: &str = "data.json";
    gen_example(path);

    let mut marks: UEGroup = UEGroup::import_from_path(path).unwrap();
    marks.compute_average();

    println!("{:?}", marks);
    marks.export_db(path).unwrap();
}
