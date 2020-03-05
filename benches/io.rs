use criterion::{criterion_group, criterion_main, Criterion};

// Write 1000 elements to 2 buffers. The new version seems to be 50% slower?
fn write(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("write"));
    group.bench_function("ours", |b| {
        use algebra_ours::{
            bls12_377::G1Affine, AffineCurve, CanonicalSerialize, ProjectiveCurve, UniformRand,
        };
        let el = <G1Affine as AffineCurve>::Projective::rand(&mut rand::thread_rng()).into_affine();
        let size = G1Affine::buffer_size();
        let mut writer = vec![0; 1000 * size];
        b.iter(|| {
            for i in 0..1000 {
                el.serialize(&[], &mut writer[i * size..(i + 1) * size])
                    .unwrap()
            }
        })
    });

    group.bench_function("theirs", |b| {
        use algebra_new::{
            bls12_377::G1Affine, AffineCurve, CanonicalSerialize, ProjectiveCurve, UniformRand,
        };
        let el = <G1Affine as AffineCurve>::Projective::rand(&mut rand::thread_rng()).into_affine();
        let mut writer = Vec::with_capacity(1000 * el.serialized_size());
        b.iter(|| {
            for _ in 0..1000 {
                el.serialize(&mut writer).unwrap()
            }
        })
    });
}

// Write 1000 elements to 2 buffers. The new version seems to be 2x slower?
fn write_uncompressed(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("write"));
    group.bench_function("ours", |b| {
        use algebra_ours::{
            bls12_377::G1Affine, AffineCurve, CanonicalSerialize, ProjectiveCurve, UniformRand, GroupSerialize
        };
        let el = <G1Affine as AffineCurve>::Projective::rand(&mut rand::thread_rng()).into_affine();
        let size = 2 * G1Affine::buffer_size();
        let mut writer = vec![0; 1000 * size];
        b.iter(|| {
            for i in 0..1000 {
                el.serialize_uncompressed(&mut writer[i * size..(i + 1) * size])
                    .unwrap()
            }
        })
    });

    group.bench_function("theirs", |b| {
        use algebra_new::{
            bls12_377::G1Affine, AffineCurve, CanonicalSerialize, ProjectiveCurve, UniformRand,
        };
        let el = <G1Affine as AffineCurve>::Projective::rand(&mut rand::thread_rng()).into_affine();
        let mut writer = Vec::with_capacity(1000 * el.uncompressed_size());
        b.iter(|| {
            for _ in 0..1000 {
                el.serialize_uncompressed(&mut writer).unwrap()
            }
        })
    });
}

criterion_group!(benches, write, write_uncompressed);
criterion_main!(benches);
