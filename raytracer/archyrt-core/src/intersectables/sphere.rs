use crate::{
    utilities::{
        math::{solve_quadratic, QuadraticResult, Vec3},
        ray::{Intersectable, Intersection, IntersectionBuilder, Ray},
    },
    vector,
};

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
    pub color: Vec3,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            origin: vector!(0.0, 0.0, 0.0),
            radius: 1.0,
            color: vector!(1.0, 1.0, 1.0),
        }
    }
}

fn find_closest(solutions: QuadraticResult) -> Option<f64> {
    match solutions {
        QuadraticResult::TwoResults(a, b) => {
            match (a, b) {
                //If they are both valid and b is smaller, return b
                (a, b) if a >= 0.0 && b >= 0.0 && b < a => Some(b),
                //Else, if they are both valid, return a
                (a, b) if a >= 0.0 && b >= 0.0 => Some(a),
                //Else, if a is valid, return a
                (a, _) if a >= 0.0 => Some(a),
                //Else, if b is valid, return a
                (_, b) if b >= 0.0 => Some(b),
                //If neither of them are valid
                _ => None,
            }
        }
        QuadraticResult::OneResult(a) => {
            if a >= 0.0 {
                Some(a)
            } else {
                None
            }
        }
        QuadraticResult::NoResults => None,
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        //Solving the equation |ray.origin+ray.direction*t-self.origin|=self.radius for t
        //Rewritten: t^2*(ray.direction^2) + t*(2*ray.origin*ray.direction-2*ray.direction*self.origin)+(ray.origin^2+self.origin^2-2*ray.origin*self.origin)=self.radius^2
        let t_: Vec<f64> = (0..3)
            .into_iter()
            .map(|i| ray.direction[i] * ray.direction[i])
            .collect();
        let t2 = t_.iter().sum();
        let t: f64 = (0..3)
            .into_iter()
            .map(|i| {
                2.0 * ray.origin[i] * ray.direction[i] - 2.0 * ray.direction[i] * self.origin[i]
            })
            .sum();
        let c: f64 = (0..3)
            .map(|i| {
                ray.origin[i] * ray.origin[i] + self.origin[i] * self.origin[i]
                    - 2.0 * ray.origin[i] * self.origin[i]
            })
            .sum::<f64>()
            - self.radius * self.radius;
        let solutions = solve_quadratic(t2, t, c);
        let distance = find_closest(solutions)?;
        let pos = ray.direction * distance + ray.origin;
        let normal = (pos - self.origin) / self.radius;

        Some(
            IntersectionBuilder {
                distance: Some(distance),
                pos: Some(pos),
                color: self.color,
                ray,
                normal,
                ..Default::default()
            }
            .build(),
        )
    }
}