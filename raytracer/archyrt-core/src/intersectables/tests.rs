mod sphere {
    use crate::{
        intersectables::sphere::Sphere,
        utilities::{
            math::Vec3,
            ray::{Intersectable, Ray},
        },
    };

    #[test]
    fn intersect() {
        let ray = Ray {
            origin: Vec3::new(1.0, 1.0, 1.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let sphere = Sphere {
            origin: Vec3::new(1.0, 1.0, 4.5),
            radius: 0.5,
            ..Default::default()
        };
        let intersection = sphere.intersect(ray).unwrap();
        assert_eq!(intersection.get_distance(), 3.0);
        assert_eq!(intersection.get_pos(), Vec3::new(1.0, 1.0, 4.0));
        assert_eq!(intersection.get_normal(), Vec3::new(0.0, 0.0, -1.0));
    }
    #[test]
    fn intersect_fail() {
        let ray = Ray {
            origin: Vec3::new(1.0, 1.0, 1.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        //Sphere behind the ray
        let sphere = Sphere {
            origin: Vec3::new(1.0, 1.0, -4.5),
            radius: 0.5,
            ..Default::default()
        };
        assert!(sphere.intersect(ray).is_none())
    }
}

mod surface {
    use crate::{
        intersectables::surface::Surface,
        utilities::{
            math::Vec3,
            ray::{Intersectable, Ray},
        },
    };

    #[test]
    fn intersect() {
        let ray = Ray {
            origin: Vec3::new(0.0, 2.0, 0.0),
            direction: Vec3::new(0.0, -1.0, 0.0),
        };
        let surface = Surface {
            normal: Vec3::new(0.0, 1.0, 0.0),
            distance: 0.0,
            ..Default::default()
        };
        let intersection = surface.intersect(ray).unwrap();
        assert_eq!(intersection.get_distance(), 2.0);
        assert_eq!(intersection.get_distance_squared(), 4.0);
        assert_eq!(intersection.get_pos(), Vec3::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn intersect_from_points() {
        let ray = Ray {
            origin: Vec3::new(0.0, 2.0, 0.0),
            direction: Vec3::new(0.0, -1.0, 0.0),
        };
        let surface = Surface::from_points(
            [
                Vec3::new(0.0, -0.0, 0.0),
                Vec3::new(-1.0, -0.0, 1.0),
                Vec3::new(1.0, -0.0, 1.0),
            ],
            Default::default(),
        );
        let intersection = surface.intersect(ray).unwrap();
        assert_eq!(intersection.get_distance(), 2.0);
        assert_eq!(intersection.get_distance_squared(), 4.0);
        assert_eq!(intersection.get_pos(), Vec3::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn intersect_fail() {
        let ray = Ray {
            origin: Vec3::new(0.0, 2.0, 0.0),
            direction: Vec3::new(0.0, 1.0, 0.0),
        };
        let surface = Surface {
            normal: Vec3::new(0.0, 1.0, 0.0),
            distance: 0.0,
            ..Default::default()
        };
        let intersection = surface.intersect(ray);
        assert!(intersection.is_none());
    }
}

mod triangle {
    use crate::{
        intersectables::triangle::Triangle,
        utilities::{
            math::Vec3,
            ray::{Intersectable, Ray},
        },
        vector, textures::TextureID,
    };

    #[test]
    fn intersect() {
        let triangle = Triangle::new(
            [
                Vec3::new(0.0, -1.0, 1.0),
                Vec3::new(1.0, -1.0, -1.0),
                Vec3::new(-1.0, -1.0, -1.0),
            ],
            [vector!(0.0, 0.0), vector!(0.0, 1.0), vector!(1.0, 0.0)],
            TextureID(1),
        );
        let ray = Ray {
            origin: Vec3::from_single(0.0),
            direction: Vec3::new(0.0, -1.0, 0.0),
        };
        let intersection = triangle.intersect(ray).unwrap();
        assert_eq!(intersection.get_pos(), Vec3::new(0.0, -1.0, 0.0));
    }
    #[test]
    fn intersect_fail() {
        let triangle = Triangle::new(
            [
                Vec3::new(0.0, -1.0, 1.0),
                Vec3::new(1.0, -1.0, -1.0),
                Vec3::new(-1.0, -1.0, -1.0),
            ],
            [vector!(0.0, 0.0), vector!(0.0, 1.0), vector!(1.0, 0.0)],
            TextureID(1),
        );
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 5.0),
            direction: Vec3::new(0.0, -1.0, 0.0),
        };
        let intersection = triangle.intersect(ray);
        assert!(intersection.is_none());
    }
}

mod aabb {
    use crate::{
        intersectables::aabb::AABB,
        utilities::{math::Vec3, ray::Ray},
    };

    #[test]
    fn intersect() {
        let ray = Ray::new(Vec3::default(), Vec3::new(0.0, 0.0, 1.0));
        let aabb = AABB::new(Vec3::new(-1.0, -1.0, 5.0), Vec3::new(1.0, 1.0, 7.0));
        assert!(aabb.intersect(ray).is_some());
    }
    #[test]
    fn intersect_fail() {
        let ray = Ray::new(Vec3::default(), Vec3::new(0.0, 1.0, 0.0));
        let aabb = AABB::new(Vec3::new(-1.0, -1.0, 5.0), Vec3::new(1.0, 1.0, 7.0));
        assert!(aabb.intersect(ray).is_none());

        let ray = Ray::new(Vec3::default(), Vec3::new(0.0, 0.0, -1.0));
        let aabb = AABB::new(Vec3::new(-1.0, -1.0, 5.0), Vec3::new(1.0, 1.0, 7.0));
        assert!(aabb.intersect(ray).is_none());
    }
}
