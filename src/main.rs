use std::sync;
use std::sync::atomic::Ordering::Relaxed;

use actix_web::dev::Factory;
use actix_web::{error, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde;
use serde_json;
use serde::Serialize;
use serde::Deserialize;
use atomic_float::{self, AtomicF64};

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

    // TODO add error handeling
    fn update(&mut self) {
        if self.particles.len() == 0 {
            return;
        }

        let mut numerator = AtomicF64::new(0.0);
        let mut summed_weights = AtomicF64::new(0.0);

        self.particles
            .as_mut_slice()
            .into_par_iter()
            .for_each(|p| {p.update();});

        self.particles
            .as_mut_slice()
            .into_par_iter()
            .filter(|p| {p.average.is_some()})
            .for_each(|particle| {
                let w = particle.get_weight();
                numerator.fetch_add(particle.get_average().unwrap() * w, Relaxed);
                summed_weights.fetch_add(w, Relaxed);
            });

        self.average = Some(*numerator.get_mut() / *summed_weights.get_mut());
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

async fn get_particle(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

//async fn capture_mark(evt: web::Json<Particle>) -> Result<String> {
async fn capture_mark(evt: web::Json<Particle>) -> impl Responder {
    println!("{:?}", evt.average);
    format!("{:?}", evt.average)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/particle", web::get().to(get_particle))
            .route("/particle", web::post().to(capture_mark))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
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
        p1.weight = Some(1.0);

        let mut p2 = Particle::new(2.0, 4.0);

        let p = Particle::from_iter(vec![p1, p2]);

        assert_eq!(p.average.unwrap(), 3.0);
    }
}
