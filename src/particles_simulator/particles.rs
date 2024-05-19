use graphics::math::{add, mul_scalar, Vec2d};
use piston_window::*;
use rand::prelude::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::time::Instant;

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

struct ReportingAllocator;

/// Executes the closure without logging on allocation
pub fn run_guarded<F>(f: F)
where
    F: FnOnce(),
{
    thread_local! {
        static GUARD: Cell<bool> = const { Cell::new(false) };
    }

    GUARD.with(|guard| {
        if !guard.replace(true) {
            f();
            guard.set(false);
        }
    })
}

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();

        run_guarded(|| eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos()));
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

pub struct World {
    pub current_turn: u64,
    pub particles: Vec<Box<Particle>>,
    pub height: f64,
    pub width: f64,
    pub rng: ThreadRng,
}

pub struct Particle {
    pub height: f64,
    pub width: f64,
    pub position: Vec2d<f64>,
    pub velocity: Vec2d<f64>,
    pub acceleration: Vec2d<f64>,
    pub color: [f32; 4],
}

impl Particle {
    pub fn new(world: &World) -> Particle {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;
        let x_velocity = 0.0;
        let y_velocity = rng.gen_range(-2.0..0.0);
        let x_acceleration = 0.0;
        let y_acceleration = rng.gen_range(0.0..0.15);

        Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y],
            velocity: [x_velocity, y_velocity],
            acceleration: [x_acceleration, y_acceleration],
            color: [1.0, 1.0, 1.0, 0.99],
        }
    }

    pub fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
        self.acceleration = mul_scalar(self.acceleration, 0.7);
        self.color[3] *= 0.995;
    }
}

impl World {
    pub fn new(width: f64, height: f64) -> World {
        World {
            current_turn: 0,
            particles: Vec::<Box<Particle>>::new(),
            height,
            width,
            rng: thread_rng(),
        }
    }

    pub fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let particle = Particle::new(self);
            let boxed_particle = Box::new(particle);
            self.particles.push(boxed_particle);
        }
    }

    pub fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut particle_iter = self.particles.iter().enumerate();

            if let Some((i, particle)) = particle_iter.next() {
                if particle.color[3] < 0.02 {
                    self.particles.remove(i);
                    break;
                } else {
                    self.particles.remove(0);
                }
            }
        }
    }

    pub fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3);

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        self.particles.shrink_to_fit();
        for shape in &mut self.particles {
            shape.update();
        }
        self.current_turn += 1;
    }
}

pub fn run_simulation() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create a window.");

    let mut world = World::new(width, height);
    world.add_shapes(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.height];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });
    }
}
