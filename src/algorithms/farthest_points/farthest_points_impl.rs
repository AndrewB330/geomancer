use crate::{
    common::GeometryError,
    prelude::ConvexHull2D,
    traits::{Cross2D, Dot2D, Norm2D, NormSqr2D},
};

pub(super) fn farthest_points_impl<K>(points: &[K::Point]) -> Result<(usize, usize), GeometryError>
where
    K: Norm2D + NormSqr2D + Dot2D + Cross2D,
    K::Point: Clone,
    K::Scalar: PartialOrd,
{
    let chull = ConvexHull2D::<K>::from_points(points)?;
    let indices = chull.hull_indices();
    let mut pointer = 0;
    let next = |pointer: usize| {
        if pointer + 1 < indices.len() {
            pointer + 1
        } else {
            0
        }
    };
    let mut max_distance = K::distance_sqr(&points[indices[0]], &points[indices[1]]);
    let mut pair = (0, 1);
    for i in 0..indices.len() {
        let mut distance = K::distance_sqr(&points[indices[i]], &points[indices[pointer]]);
        loop {
            let next_distance =
                K::distance_sqr(&points[indices[i]], &points[indices[next(pointer)]]);
            if next_distance < distance {
                break;
            }
            distance = next_distance;
            pointer = next(pointer);
        }
        if distance > max_distance {
            max_distance = distance;
            pair = (i, pointer);
        }
    }

    Ok((indices[pair.0], indices[pair.1]))
}

#[cfg(test)]
mod tests {
    use crate::{
        algorithms::farthest_points::farthest_points_impl::farthest_points_impl,
        kernels::TupleKernel2D, traits::NormSqr2D,
    };
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use rstest::rstest;

    fn assert_farthest_points_f32(points: &[(f32, f32)], expected: (usize, usize)) {
        let (i, j) = farthest_points_impl::<TupleKernel2D<f32>>(&points).unwrap();
        // Compare distance sqr using NormSqr trait
        let dist_ij = TupleKernel2D::<f32>::distance_sqr(&points[i], &points[j]);
        let dist_expected =
            TupleKernel2D::<f32>::distance_sqr(&points[expected.0], &points[expected.1]);
        //assert!((dist_ij - dist_expected).abs() < f32::EPSILON * 10.0);
        assert_eq!(dist_ij, dist_expected);
    }

    #[test]
    fn test_simple() {
        assert_farthest_points_f32(&[(0.1, 0.2), (10.0, -1.0), (5.0, 2.0), (7.0, 7.0)], (0, 1));
    }

    #[test]
    fn test_square() {
        let points = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        assert_farthest_points_f32(&points, (0, 2));
    }

    #[rstest]
    fn random_points_impl_vs_naive(#[values(2, 4, 64, 256, 1024, 8192)] n: usize) {
        let mut rng: StdRng = SeedableRng::seed_from_u64(n as u64);

        // Generate random points first
        let mut points = vec![];
        let n = 10000;
        while points.len() < n {
            let x = rng.random_range(-1.0..1.0);
            let y = rng.random_range(-1.0..1.0);
            if x * x + y * y > 1.0 {
                continue;
            }
            points.push((x, y));
        }
        // Naive solution
        let mut max_distance = 0.0;
        let mut expected = (0, 0);
        for i in 0..n {
            for j in i + 1..n {
                let dist = TupleKernel2D::<f32>::distance_sqr(&points[i], &points[j]);
                if dist > max_distance {
                    max_distance = dist;
                    expected = (i, j);
                }
            }
        }
        println!(
            "Expected farthest points: {:?}, Distance: {}",
            expected, max_distance
        );
        assert_farthest_points_f32(&points, expected);
    }
}
