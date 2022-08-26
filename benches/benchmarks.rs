use turbo_ecs::prelude::*;
use criterion::*;
use nalgebra_glm::{Mat4, Vec3};
use turbo_ecs::create_archetype;

const COUNT: usize = 10000;

#[derive(Default, Copy, Clone, Component)]
struct Transform(Mat4);

#[derive(Default, Copy, Clone, Component)]
struct Translation(Vec3);

#[derive(Default, Copy, Clone, Component)]
struct Rotation(Vec3);

#[derive(Default, Copy, Clone, Component)]
struct Velocity(Vec3);

fn create_from_archetype(c: &mut Criterion) {
	let mut group = c.benchmark_group("Create from archetype");
	group.bench_function("clear: false", |b| {
		let mut entities = vec![Entity::default(); COUNT];

		b.iter_batched(
			|| {
				let mut ecs = EcsContext::new();
				let archetype = create_archetype!(ecs, [Transform, Translation, Rotation, Velocity]);
				(ecs, archetype)
			},
			|(mut ecs, archetype)| {
				ecs.create_entities_from_archetype(archetype, &mut entities, false)
			},
			BatchSize::PerIteration
		);
	});

	group.bench_function("clear: true", |b| {
		let mut entities = vec![Entity::default(); COUNT];

		b.iter_batched(
			|| {
				let mut ecs = EcsContext::new();
				let archetype = create_archetype!(ecs, [Transform, Translation, Rotation, Velocity]);
				(ecs, archetype)
			},
			|(mut ecs, archetype)| {
				ecs.create_entities_from_archetype(archetype, &mut entities, true)
			},
			BatchSize::PerIteration
		);
	});
}

fn iterate_entities(c: &mut Criterion) {
	c.bench_function("Iterate entities", |b| {
		let mut ecs = EcsContext::new();
		let mut entities = vec![Entity::default(); COUNT];
		let archetype = create_archetype!(ecs, [Transform, Translation, Rotation, Velocity]);
		ecs.create_entities_from_archetype(archetype, &mut entities, true);

		b.iter(|| ecs.filter().include::<(&mut Translation, &Velocity)>().for_each(|(t, v)| {
			t.0 += v.0;
		}));
	});
}

criterion_group!(benchmarks, create_from_archetype, iterate_entities);
criterion_main!(benchmarks);