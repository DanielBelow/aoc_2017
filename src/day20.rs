use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::ops::{AddAssign, Mul};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Copy, Clone, Debug)]
#[display("<{x},{y},{z}>")]
pub struct Vec3d {
    x: i64,
    y: i64,
    z: i64,
}

impl AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul<i64> for Vec3d {
    type Output = Vec3d;

    fn mul(self, rhs: i64) -> Self::Output {
        let mut new_pos = self;

        new_pos.x *= rhs;
        new_pos.y *= rhs;
        new_pos.z *= rhs;

        new_pos
    }
}

#[derive(Display, FromStr, PartialEq, Eq, Hash, Copy, Clone, Debug)]
#[display("id={id}, p={position}, v={velocity}, a={acceleration}")]
pub struct Particle {
    id: usize,
    position: Vec3d,
    velocity: Vec3d,
    acceleration: Vec3d,
}

impl Particle {
    fn distance(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }
}

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> Vec<Particle> {
    inp.lines()
        .enumerate()
        .map(|(idx, it)| format!("id={idx}, {it}"))
        .filter_map(|it| it.parse().ok())
        .collect()
}

#[aoc(day20, part1)]
pub fn part1(particles: &[Particle]) -> Option<usize> {
    let mut particles = particles.to_vec();

    for _ in 0..1_000 {
        particles.iter_mut().for_each(Particle::update);
    }

    particles
        .iter()
        .min_by_key(|it| it.distance())
        .map(|it| it.id)
}

#[aoc(day20, part2)]
pub fn part2(particles: &[Particle]) -> usize {
    let mut particles = particles.to_vec();

    for _ in 0..1_000 {
        for p in &mut particles {
            p.update();
        }

        let positions = particles.iter().map(|it| it.position).counts();
        particles.retain(|it| positions.get(&it.position).map_or(true, |cnt| *cnt == 1));
    }

    particles.len()
}
