use rand_distr::{Normal, Distribution};
use rayon::prelude::*;

#[derive(Clone)]
struct Particle {
    x: f64,
    z: f64,
    x_velocity: f64,
    z_velocity: f64,
    mass: f64
}

struct Particles {
    particles: Vec<Particle>,
}

impl Particle {
    //this needs to take in a whole slice of every particle mutably, loop over them, and modify
    fn update_position(&mut self) {
        //update position of one Particle
        self.x += self.x_velocity;
        self.z += self.z_velocity;
    }

    fn update_velocity(&mut self, particles: &[Particle]) {
        let mut x_acceleration_sum: f64 = 0f64;
        let mut z_acceleration_sum: f64 = 0f64;

        //not mutable, everything fine
        for i in 0..particles.len() {
            let particle = &particles[i];

            //get distances
            let dx: f64 = particle.x - self.x;
            let dz: f64 = particle.z - self.z;
            let distance: f64 = (dx*dx + dz*dz).sqrt();

            //when they collide, create a new particle and delete the old two
            //...eventually
            if 1000.0 < distance {
                continue;
            }

            if distance < 1.0 {
                continue;
            }

            let force: f64 = (self.mass * particle.mass) / (distance*distance);
            let acceleration: f64 = force / self.mass;

            //calculate the direction the acceleration is applied in
            let theta: f64 = dz.atan2(dx);

            x_acceleration_sum += acceleration * theta.cos();
            z_acceleration_sum += acceleration * theta.sin();
        }

        //only now do we modify anything in particles
        self.x_velocity += x_acceleration_sum;
        self.z_velocity += z_acceleration_sum;

        //speed decay
        //self.x_velocity *= 0.99;
        //self.z_velocity *= 0.99;
    }
}

impl Particles {
    fn new() -> Particles {
        Particles {
            particles: Vec::new(),
        }
    }

    //not atomic, duh
    fn create_random_particles(&mut self, count: u64) {
        let normal = Normal::new(0.0, 100.0).unwrap();
        for _ in 0..count {
            let x: f64 = normal.sample(&mut rand::rng());
            let z: f64 = normal.sample(&mut rand::rng());

            let dist = (x * x + z * z).sqrt();
            let orbital_speed = 50.0 / dist.sqrt(); // tune this

            let x_velocity = (-z / dist) * orbital_speed;
            let z_velocity = (x / dist) * orbital_speed;

            self.particles.push(
                Particle {
                    x: x + 512.0,
                    z: z + 512.0,
                    x_velocity,
                    z_velocity,
                    mass: 0.01,
                }
            );
        }
    }

    fn update_positions(&mut self) {
        //for each particle, update velocity
        self.particles.par_iter_mut().for_each(|particle| particle.update_position())
    }

    fn update_velocities(&mut self) {
        //for each particle, update velocity
        let temp_particles = self.particles.clone();
        self.particles.par_iter_mut().for_each(|particle| particle.update_velocity(&temp_particles))
    }

    fn make_frame(&self, frame: usize) {
        let mut image = image::RgbImage::new(4096, 4096);
        image.pixels_mut().for_each(|pixel| *pixel = image::Rgb([0, 0, 0]));

        for particle in &self.particles {
            if 0.0 < particle.x &&
                particle.x < 1024.0 && 
                0.0 < particle.z &&
                particle.z < 1024.0 {
                image.put_pixel((4.0 * particle.x) as u32, (4.0 * particle.z) as u32, image::Rgb([255, 255, 255]));
            }
        }

        image.save(format!("/media/meox/NETAC/frames/frame{:05}.bmp", frame)).unwrap();
    }
}

fn main() {

    let mut particles = Particles::new();
    particles.create_random_particles(100000);
    particles.particles[0] = Particle {
        x: 512.0,
        z: 512.0,
        x_velocity: 0.0,
        z_velocity: 0.0,
        mass: 5000.0,
    };
    //update loop
    let mut i = 0;
    loop {
        particles.update_velocities();
        particles.update_positions();
        particles.make_frame(i);
        println!("update {} done", i);
        i += 1;
    }


    //println!("particles vector is {} Particle s long", particles.len());

}
