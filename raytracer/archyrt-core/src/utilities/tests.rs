#[cfg(test)]
mod vectors {
    use std::f64::consts::PI;

    use crate::{
        utilities::math::{Vec2, Vec3},
        vector,
    };

    #[test]
    fn creation() {
        let v = vector!(1.0, 2.0, 3.0);
        assert_eq!(v.inner, [1.0, 2.0, 3.0]);
    }
    #[test]
    fn equality() {
        let v1 = vector!(1.0, 2.0, 3.0);
        let v2 = vector!(1.0, 2.0, 3.0);
        let v3 = vector!(4.0, 5.0, 6.0);
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }
    #[test]
    fn addition() {
        let v1 = vector!(1.0, 2.0, 3.0);
        let v2 = vector!(4.0, 5.0, 6.0);
        assert_eq!(v1 + v2, vector!(5.0, 7.0, 9.0));
        let mut v1 = v1;
        v1 += v2;
        assert_eq!(v1, vector!(5.0, 7.0, 9.0));
    }
    #[test]
    fn mul() {
        let v1 = vector!(1.0, 2.0, 3.0);
        let v2 = vector!(4.0, 5.0, 6.0);
        assert_eq!(v1 * v2, vector!(4.0, 10.0, 18.0));
        let mut v1 = v1;
        v1 *= v2;
        assert_eq!(v1, vector!(4.0, 10.0, 18.0));
    }
    #[test]
    fn mul_scalar() {
        let v1 = vector!(1.0, 2.0, 3.0);
        assert_eq!(v1 * 2.0, vector!(2.0, 4.0, 6.0));
        assert_eq!(2.0 * v1, vector!(2.0, 4.0, 6.0));
        let mut v1 = v1;
        v1 *= 2.0;
        assert_eq!(v1, vector!(2.0, 4.0, 6.0));
    }
    #[test]
    fn div_scalar() {
        let v1 = vector!(1.0, 2.0, 3.0);
        assert_eq!(v1 / 2.0, vector!(0.5, 1.0, 1.5));
        let mut v1 = v1;
        v1 /= 2.0;
        assert_eq!(v1, vector!(0.5, 1.0, 1.5));
    }
    #[test]
    fn dot_product() {
        let v1 = vector!(1.0, 2.0, 3.0);
        let v2 = vector!(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(v2), 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0);
    }
    #[test]
    fn normalized() {
        const EPSILON: f64 = 0.01;
        let v1 = vector!(0.4, 0.3, 0.8);
        //Make sure distance from 1.0 is within error margins
        assert!((v1.normalized().length() - 1.0).abs() < EPSILON);
        let v2 = vector!(1.5, PI, 7.8);
        //Make sure distance from 1.0 is within error margins
        assert!((v2.normalized().length() - 1.0).abs() < EPSILON);
    }
    #[test]
    fn vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = vector!(1.0, 2.0, 3.0);

        assert_eq!(v1, v2);
        assert_eq!(v1.x(), 1.0);
        assert_eq!(v1.y(), 2.0);
        assert_eq!(v1.z(), 3.0);
    }
    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(v1.cross(v2), vector!(0.0, 0.0, 1.0));

        let v1 = Vec3::new(0.0, 0.0, 1.0);
        let v2 = Vec3::new(-1.0, 0.0, 0.0);
        assert_eq!(v1.cross(v2), vector!(0.0, -1.0, 0.0));

        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(v1.cross(v2), vector!(0.0, -1.0, 0.0));
    }
    #[test]
    fn vec2() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = vector!(1.0, 2.0);

        assert_eq!(v1, v2);
        assert_eq!(v1.x(), 1.0);
        assert_eq!(v1.y(), 2.0);
    }
}

#[cfg(test)]
mod matrices {
    use crate::{matrix, utilities::math::Matrix, vector};

    #[test]
    fn equal() {
        let m1 = matrix!(vector!(1.0, 2.0), vector!(3.0, 4.0), vector!(5.0, 6.0));
        let m2 = matrix!(vector!(1.0, 2.0), vector!(3.0, 4.0), vector!(5.0, 6.0));
        let m3 = matrix!(vector!(1.0, 2.0), vector!(3.0, 4.0), vector!(10.0, 10.0));
        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }
    #[test]
    fn indexing() {
        let m = matrix!(vector!(1.0, 2.0), vector!(3.0, 4.0), vector!(5.0, 6.0));
        assert_eq!(m[0], vector!(1.0, 2.0));
        assert_eq!(m[2], vector!(5.0, 6.0));
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[2][1], 6.0);
    }
    #[test]
    fn identity() {
        let size_2 = Matrix::<2, 2>::identity();
        assert_eq!(size_2, matrix!(vector!(1.0, 0.0), vector!(0.0, 1.0)));
        let size_3 = Matrix::<3, 3>::identity();
        assert_eq!(
            size_3,
            matrix!(
                vector!(1.0, 0.0, 0.0),
                vector!(0.0, 1.0, 0.0),
                vector!(0.0, 0.0, 1.0)
            )
        );
    }
    #[test]
    fn transpose() {
        let m1 = matrix!(vector!(0.0, 1.0), vector!(2.0, 3.0), vector!(4.0, 5.0));
        assert_eq!(
            m1.transpose(),
            matrix!(vector!(0.0, 2.0, 4.0), vector!(1.0, 3.0, 5.0))
        );
        let m2: Matrix<5, 5> = Matrix::identity();
        assert_eq!(m2, m2.transpose());
    }
    #[test]
    fn vector_mul() {
        let v = vector!(1.0, 2.0, 3.0);
        let m: Matrix<3, 3> = Matrix::identity();
        assert_eq!(m * v, v);
        let m: Matrix<4, 3> = matrix!(
            vector!(1.0, 2.0, 3.0, 4.0),
            vector!(5.0, 6.0, 7.0, 8.0),
            vector!(9.0, 10.0, 11.0, 12.0)
        );
        assert_eq!(m * v, vector!(38.0, 44.0, 50.0, 56.0));
    }
    #[test]
    fn matrix_mul() {
        let m1 = matrix!(
            vector!(1.0, 2.0, 3.0),
            vector!(4.0, 5.0, 6.0),
            vector!(7.0, 8.0, 9.0)
        );
        let m2: Matrix<3, 3> = Matrix::identity();
        assert_eq!(m2 * m1, m1);

        let m1 = matrix!(vector!(1.0, 2.0), vector!(5.0, 6.0), vector!(9.0, 10.0));
        let m2 = matrix!(vector!(1.0, 2.0, 3.0), vector!(6.0, 7.0, 8.0));
        let product = m1 * m2;
        assert_eq!(product[0], vector!(13.0, 16.0, 19.0));
        assert_eq!(product[1], vector!(41.0, 52.0, 63.0));
    }
}

#[cfg(test)]
mod rays {
    use crate::{
        utilities::{
            math::Vec3,
            ray::{IntersectionBuilder, Ray},
        },
        vector,
    };

    #[test]
    fn intersection_builder() {
        let color = Vec3::new(1.0, 2.0, 3.0);
        let normal = Vec3::new(4.0, 5.0, 6.0);
        let pos = Some(Vec3::new(7.0, 8.0, 9.0));
        let distance = Some(0.5);
        let distance_squared = Some(0.25);
        let ray = Ray {
            origin: vector!(0.1, 0.2, 0.3),
            direction: vector!(0.4, 0.5, 0.5),
        };
        let intersection = IntersectionBuilder {
            color,
            normal,
            distance,
            distance_squared,
            pos,
            ray,
        }
        .build();
        assert_eq!(color, intersection.get_color());
        assert_eq!(normal, intersection.get_normal());
        assert_eq!(pos.unwrap(), intersection.get_pos());
        assert_eq!(distance.unwrap(), intersection.get_distance());
        assert_eq!(
            distance_squared.unwrap(),
            intersection.get_distance_squared()
        );
    }
    #[test]
    fn intersection_conversion() {
        let i1 = IntersectionBuilder {
            ray: Ray {
                direction: vector!(0.0, 0.0, 1.0),
                ..Default::default()
            },
            pos: Some(Vec3::new(0.0, 0.0, 2.0)),
            ..Default::default()
        }
        .build();
        let i2 = IntersectionBuilder {
            ray: Ray {
                direction: vector!(0.0, 0.0, 1.0),
                ..Default::default()
            },
            distance: Some(2.0),
            ..Default::default()
        }
        .build();
        let i3 = IntersectionBuilder {
            ray: Ray {
                direction: vector!(0.0, 0.0, 1.0),
                ..Default::default()
            },
            distance_squared: Some(4.0),
            ..Default::default()
        }
        .build();

        assert_eq!(i1.get_distance_squared(), 4.0);
        assert_eq!(i1.get_distance(), 2.0);
        assert_eq!(i2.get_distance_squared(), 4.0);
        assert_eq!(i2.get_pos(), Vec3::new(0.0, 0.0, 2.0));
        assert_eq!(i3.get_distance(), 2.0);
        assert_eq!(i3.get_pos(), Vec3::new(0.0, 0.0, 2.0));
    }
}

mod quadratic {
    use crate::utilities::math::{solve_quadratic, QuadraticResult};

    #[test]
    fn solve() {
        //1. 3(x-5)(x-3)=3x^2-9x-15x+45=3x^2-24x+45
        if let QuadraticResult::TwoResults(a, b) = solve_quadratic(3.0, -24.0, 45.0) {
            if !((a == 5.0 && b == 3.0) || (a == 3.0) && (b == 5.0)) {
                panic!("1. test: The solutions are not 5.0 and 3.0");
            }
        } else {
            panic!("1. test: Invalid number of solutions");
        }
        //3. 3(x-5)(x-5)=3*(x^2-10x+25)=3x^2-30x+75
        if let QuadraticResult::OneResult(a) = solve_quadratic(3.0, -30.0, 75.0) {
            assert_eq!(a, 5.0);
        } else {
            panic!("2. test: Invalid number of solutions");
        }
        //3. 3(x-5i)(x+5i)=3*(x^2-(5i)^2)=3*x^2+0x+75
        if let QuadraticResult::NoResults = solve_quadratic(3.0, 0.0, 75.0) {
        } else {
            panic!("3. test: Invalid number of solutions");
        }
    }
}
