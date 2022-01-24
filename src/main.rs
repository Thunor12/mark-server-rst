use std::fs::File;
use std::io::prelude::*;
//use std::stream::FromIter;
//use actix_web::middleware::normalize::TrailingSlash;
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

impl FromIterator<Particle> for Particle {
    fn from_iter<I: IntoIterator<Item=Particle>>(iter: I) -> Self {
        let mut u = Self::default();

        for p in iter {
            u.particles.push(p);
        }

        u.update();

        u
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

    fn get_average(& self) -> Result<f64, Box<dyn std::error::Error>> {
        let e: Box<dyn std::error::Error> = From::from("No average available".to_string());
        self.average.ok_or(e)
    }

    fn new(weight: f64, average: f64) -> Self {
        Particle {
            average: Some(average),
            weight: Some(weight),
            particles: vec![],
        }
    }

    // TODO Test with rayon
    // TODO add error handeling
    fn update(&mut self) {
        if(self.particles.len() == 0) {
            return;
        }

        let mut numerator = 0 as f64;
        let mut summed_weights  = 0 as f64;

        for particle in self.particles.as_mut_slice() {
            particle.update();
            if(particle.average.is_some()) {
                numerator += particle.get_average().unwrap() * particle.get_weight();
                summed_weights += particle.get_weight();
            }
        }

        self.average = Some(numerator / summed_weights);
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExameGroup {
    name: String,
    particle: Particle,
}

impl Default for ExameGroup {
    fn default() -> Self {
        ExameGroup {
            name: "".to_string(),
            particle: Particle::default(),
        }
    }
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
            name: "".to_string(),
            particle: Particle::default(),
            passed: None,
            requirement: 10 as f64,
        }
    }
}

impl UEGroup {
    fn determin_if_passed(&mut self) -> bool {
        unimplemented!("Determin if the student passed");
    }
}


trait ParticleContainer {
    fn get_particle_ref(&mut self) -> &mut Particle;

    fn update_particle(&mut self) {
        self.get_particle_ref().update();
    }
}


impl FromIterator<ExameGroup> for UEGroup {
    fn from_iter<I: IntoIterator<Item=ExameGroup>>(iter: I) -> Self {
        let mut u = Self::default();

        for exam in iter {
            u.particle.particles.push(exam.particle);
        }

        u.particle.update();

        u
    }
}


// trait Transmitable {
//     fn export_tp_path(& self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
//         let mut file = File::create(path)?;

//         let data = serde_json::to_string(&self)?;
//         file.write_all(data.as_bytes())?;

//         Ok(())
//     }

//     fn import_from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
//         let file = File::open(path)?;
//         let data = serde_json::from_reader(file)?;
//         Ok(data)
//     }
// }

// impl Transmitable for Particle {}
// impl Transmitable for ExameGroup {}
// impl Transmitable for UEGroup {}


fn main() {
    /*
    let path: &str = "data.json";
    gen_example(path);

    let mut marks: UEGroup = UEGroup::import_from_path(path).unwrap();
    marks.compute_average();

    println!("{:?}", marks);
    marks.export_db(path).unwrap();
     */
    println!("Writting Tests First !!");
}


#[cfg(test)]
mod tests {
    use std::vec;

    use crate::Particle;

    #[test]
    #[should_panic]
    fn test_panick_if_average_is_none() {
        let p = Particle::default();
        let a = p.get_average().unwrap();
    }

    #[test]
    fn test_can_access_average_if_some() {
        let p = Particle::new(1.0, 1.0);
        assert_eq!(p.get_average().unwrap(), 1.0)
    }

    #[test]
    fn test_average_update_on_one_particle() {
        let p = Particle::from_iter(
            vec![
                Particle::new(1.0, 1.0),
                Particle::new(2.0, 4.0),
            ]
        );

        assert_eq!(p.average.unwrap(), 3.0);
    }

    #[test]
    fn test_average_update_cascade_particle() {
        let mut p1 = Particle::from_iter(
            vec![
                Particle::new(1.0, 1.0),
                Particle::new(2.0, 1.0),
            ]
        );

        let mut p2 = Particle::new(2.0, 4.0);

        let p = Particle::from_iter(vec![p1, p2]);

        assert_eq!(p.average.unwrap(), 3.0);
    }
}
