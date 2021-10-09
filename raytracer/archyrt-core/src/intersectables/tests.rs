
mod sphere{
    use crate::{intersectables::sphere::Sphere, utilities::{math::Vec3, ray::{Intersectable, Ray}}};

    #[test]
    fn intersect() {
        let ray = Ray{
            origin: Vec3::new(1.0, 1.0, 1.0),
            direction: Vec3::new(0.0, 0.0, 1.0)
        };
        let sphere = Sphere{
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
    fn intersect_fail(){
        let ray = Ray{
            origin: Vec3::new(1.0, 1.0, 1.0),
            direction: Vec3::new(0.0, 0.0, 1.0)
        };
        //Sphere behind the ray
        let sphere = Sphere{
            origin: Vec3::new(1.0, 1.0, -4.5),
            radius: 0.5,
            ..Default::default()
        };
        assert!(sphere.intersect(ray).is_none())
    }
}