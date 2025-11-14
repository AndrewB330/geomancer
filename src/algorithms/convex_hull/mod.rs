mod convex_hull_exact_impl;
mod convex_hull_impl;
use num_traits::{Float, Zero};
use std::borrow::Cow;

use crate::{
    common::GeometryError,
    traits::{
        Cross2D, DefaultKernel, Dot2D, ExactCompareNorm2D, ExactOrientation2D, Kernel2D, Norm2D,
        NormSqr2D,
    },
};

use convex_hull_exact_impl::convex_hull_exact_impl;
use convex_hull_impl::convex_hull_impl;

pub struct ConvexHull2D<'a, K>
where
    K: Kernel2D,
    K::Point: Clone,
{
    hull_indices: Vec<usize>,
    points: Cow<'a, [K::Point]>,
    degenerate: bool,
}

pub fn convex_hull<'a, V>(points: &'a [V]) -> Result<ConvexHull2D<'a, V::Kernel>, GeometryError>
where
    V: DefaultKernel + Clone,
    V::Kernel: Norm2D + NormSqr2D + Cross2D + Dot2D,
    <V::Kernel as Norm2D>::Real: Float,
    <V::Kernel as Kernel2D>::Scalar: PartialOrd,
    <V::Kernel as Kernel2D>::Point: Clone,
{
    ConvexHull2D::<<V as DefaultKernel>::Kernel>::from_points(points)
}

pub fn convex_hull_exact<'a, V>(
    points: &'a [V],
    include_collinear: bool,
) -> Result<ConvexHull2D<'a, V::Kernel>, GeometryError>
where
    V: DefaultKernel + Clone,
    V::Kernel: ExactCompareNorm2D + ExactOrientation2D,
    <V::Kernel as Kernel2D>::Point: PartialEq + Clone,
{
    ConvexHull2D::<<V as DefaultKernel>::Kernel>::from_points_exact(points, include_collinear)
}

impl<'a, K> ConvexHull2D<'a, K>
where
    K: Norm2D + NormSqr2D + Cross2D + Dot2D,
    K::Scalar: PartialOrd,
    K::Real: Float,
    K::Point: Clone,
{
    pub fn from_points_owned(
        points: impl IntoIterator<Item = K::Point>,
    ) -> Result<Self, GeometryError> {
        let points_owned = points.into_iter().collect::<Vec<_>>();
        let (hull_indices, degenerate) = convex_hull_impl::<K>(&points_owned)?;
        Ok(Self {
            degenerate,
            hull_indices,
            points: Cow::Owned(points_owned),
        })
    }

    pub fn from_points(points: &'a [K::Point]) -> Result<Self, GeometryError> {
        let (hull_indices, degenerate) = convex_hull_impl::<K>(points)?;
        Ok(Self {
            degenerate,
            hull_indices,
            points: Cow::Borrowed(points),
        })
    }
}

impl<'a, K> ConvexHull2D<'a, K>
where
    K: ExactCompareNorm2D + ExactOrientation2D,
    K::Point: PartialEq + Clone,
{
    pub fn from_points_owned_exact(
        points: impl IntoIterator<Item = K::Point>,
        include_collinear: bool,
    ) -> Result<Self, GeometryError> {
        let points_owned = points.into_iter().collect::<Vec<_>>();
        let (hull_indices, degenerate) =
            convex_hull_exact_impl::<K>(&points_owned, include_collinear)?;
        Ok(Self {
            degenerate,
            hull_indices,
            points: Cow::Owned(points_owned),
        })
    }

    pub fn from_points_exact(
        points: &'a [K::Point],
        include_collinear: bool,
    ) -> Result<Self, GeometryError> {
        let (hull_indices, degenerate) = convex_hull_exact_impl::<K>(points, include_collinear)?;
        Ok(Self {
            hull_indices,
            degenerate,
            points: Cow::Borrowed(points),
        })
    }
}

impl<'a, K: Kernel2D> ConvexHull2D<'a, K>
where
    K::Point: Clone,
{
    pub fn to_owned(self) -> ConvexHull2D<'static, K> {
        ConvexHull2D::<'static, K> {
            degenerate: self.degenerate,
            hull_indices: self.hull_indices,
            points: Cow::Owned(self.points.into_owned()),
        }
    }

    pub fn hull_size(&self) -> usize {
        self.hull_indices.len()
    }

    pub fn hull_indices(&self) -> &Vec<usize> {
        &self.hull_indices
    }

    pub fn inside_indices(&self) -> Vec<usize> {
        // We are not storing indices of points that are inside of the convex hull
        // so we need to compute them every time the method is called.
        let mut all_indices = (0..self.points.len()).collect::<Vec<_>>();
        for index in &self.hull_indices {
            all_indices[*index] = usize::MAX;
        }
        all_indices.retain(|index| *index != usize::MAX);
        all_indices
    }

    pub fn hull_points(&self) -> impl Iterator<Item = &K::Point> {
        self.hull_indices().iter().map(|i| &self.points[*i])
    }

    pub fn inside_points(&self) -> Vec<&K::Point> {
        self.inside_indices()
            .iter()
            .map(|i| &self.points[*i])
            .collect()
    }

    pub fn is_degenerate(&self) -> bool {
        self.degenerate
    }

    pub fn area(&self) -> K::Scalar
    where
        K: Cross2D,
        K::Scalar: Zero,
    {
        let mut area = K::Scalar::zero();
        let n = self.hull_indices.len();
        for i in 1..(n - 1) {
            let a = &self.points[self.hull_indices[i]];
            let b = &self.points[self.hull_indices[(i + 1) % n]];
            area = area + K::cross_with_origin(a, b, &self.points[self.hull_indices[0]]);
        }
        area
    }

    pub fn perimeter(&self) -> K::Real
    where
        K: Norm2D,
    {
        let mut perimeter = K::Real::zero();
        let n = self.hull_indices.len();
        for i in 0..n {
            let a = &self.points[self.hull_indices[i]];
            let b = &self.points[self.hull_indices[(i + 1) % n]];
            perimeter = perimeter + K::distance(a, b);
        }
        perimeter
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use rand::{rngs::StdRng, Rng, SeedableRng};
    use rstest::rstest;

    use crate::{
        algorithms::convex_hull,
        kernels::GenericKernel2D,
        traits::{ExactCompareNorm2D, ExactOrientation2D},
    };

    use super::convex_hull_exact;

    use robust::{orient2d, Coord};

    fn assert_eq_cycle(a: Vec<usize>, b: Vec<usize>) {
        assert!(!a.is_empty());
        assert!(!b.is_empty());
        let mut offset = None;

        for i in 0..a.len() {
            if a[i] == b[0] {
                offset = Some(i);
                break;
            }
        }
        let offset = offset.unwrap();

        let a: Vec<usize> = a[offset..]
            .iter()
            .chain(a[..offset].iter())
            .cloned()
            .collect();

        assert_eq!(a, b);
    }

    /*#[rstest]
    fn robust_test(#[values(2048000)] n: usize) {
        let mut rng: StdRng = SeedableRng::seed_from_u64(n as u64);
        let runs = (1e8 / n as f32) as usize + 1;
        let range = rng.random_range(0.1..10.0f64).exp();
        for _ in 0..runs {
            let mut points = vec![];
            for _ in 0..n {
                let x = rng.random_range(-range..range);
                let y = rng.random_range(-range..range);
                points.push((x, y));
            }

            points.sort_by(|(ax, ay), (bx, by)| ax.partial_cmp(&bx).unwrap());

            let leftmost = points
                .iter()
                .min_by(|(x, _), (x2, _)| x.partial_cmp(&x2).unwrap())
                .cloned()
                .unwrap();

            if true {
                points.sort_by(|(ax, ay), (bx, by)| {
                    let ax = ax - leftmost.0;
                    let ay = ay - leftmost.1;
                    let bx = bx - leftmost.0;
                    let by = by - leftmost.1;
                    (ax * by).partial_cmp(&(ay * bx)).unwrap()
                });
            } else {
                points.sort_by(|(ax, ay), (bx, by)| {
                    orient2d(Coord {
                        x: *ax, y: *ay
                    }, Coord {
                        x: *bx, y: *by
                    }, Coord {
                        x: leftmost.0, y: leftmost.1
                    }).partial_cmp(&0.0).unwrap()
                });
            }

            assert!(points.len() > 0);
        }
    }*/

    // Temporary implementation for tests only TODO: remove or replace
    unsafe impl ExactCompareNorm2D for GenericKernel2D<(f32, f32)> {
        fn compare_distance(a: &Self::Point, b: &Self::Point, to: &Self::Point) -> Ordering {
            // Cast to f64 first
            let a = (a.0 as f64, a.1 as f64);
            let b = (b.0 as f64, b.1 as f64);
            let to = (to.0 as f64, to.1 as f64);
            let da = (a.0 - to.0) * (a.0 - to.0) + (a.1 - to.1) * (a.1 - to.1);
            let db = (b.0 - to.0) * (b.0 - to.0) + (b.1 - to.1) * (b.1 - to.1);
            da.partial_cmp(&db).unwrap()
        }

        fn compare_length(a: &Self::Point, b: &Self::Point) -> Ordering {
            // cast fo f64 first
            let a = (a.0 as f64, a.1 as f64);
            let b = (b.0 as f64, b.1 as f64);
            let da = a.0 * a.0 + a.1 * a.1;
            let db = b.0 * b.0 + b.1 * b.1;
            da.partial_cmp(&db).unwrap()
        }
    }

    unsafe impl ExactOrientation2D for GenericKernel2D<(f32, f32)> {
        fn orientation(
            a: &Self::Point,
            b: &Self::Point,
            c: &Self::Point,
        ) -> crate::common::Orientation2D {
            // Cast to f64 first
            let a = (a.0 as f64, a.1 as f64);
            let b = (b.0 as f64, b.1 as f64);
            let c = (c.0 as f64, c.1 as f64);
            let val = (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
            if val > 0.0 {
                crate::common::Orientation2D::CounterClockwise
            } else if val < 0.0 {
                crate::common::Orientation2D::Clockwise
            } else {
                crate::common::Orientation2D::Collinear
            }
        }
    }

    // Tiny stress test for sanity check.
    #[rstest]
    fn random_test(#[values(1, 2, 3, 4, 8, 16, 32, 64, 128, 512, 2048, 8192)] n: usize) {
        let mut rng: StdRng = SeedableRng::seed_from_u64(n as u64);

        let runs = if n < 4 || n > 128 { 20 } else { 500 };
        let range = rng.random_range(0.2..5.0f32).exp();
        for _ in 0..runs {
            let mut points = vec![];
            for _ in 0..n {
                let x = rng.random_range(-range..range);
                let y = rng.random_range(-range..range);
                points.push((x, y));
            }
            let res = convex_hull(&points);
            // todo:
            //let res_exact = convex_hull(&points);
            let res_exact = convex_hull_exact(&points, false);

            assert!(res.is_ok());
            assert!(res_exact.is_ok());

            let res = res.unwrap();
            let res_exact = res_exact.unwrap();

            if n <= 2 {
                assert!(res.is_degenerate());
                assert!(res_exact.is_degenerate());
            }

            if res.hull_size() == res_exact.hull_size() {
                assert_eq_cycle(res.hull_indices().clone(), res_exact.hull_indices().clone())
            } else {
                let perimeter = res.perimeter();
                let area = res.area();
                let area_exact = res_exact.area();
                let tolerance = 1e-5 * perimeter;
                assert!(
                    area_exact - area < tolerance,
                    "Diff: {} Tolerance: {}",
                    area_exact - area,
                    tolerance
                );
            }
        }
    }
}
