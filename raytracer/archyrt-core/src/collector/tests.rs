use crate::{
    api::fragment_render::{FragmentContext, FragmentRender},
    textures::texture_repo::TextureRepository,
    utilities::math::{Vec2, Vec3},
};

struct DummyRenderer {}

impl FragmentRender for DummyRenderer {
    fn render_fragment(&self, _: &FragmentContext, pos: Vec2) -> Vec3 {
        let c = (pos.x() + pos.y()) / 2.0;
        Vec3::new(c, c, c)
    }
}

mod array_collector {
    use crate::{
        api::fragment_collector::FragmentCollector,
        collector::{array_collector::ArrayCollector, tests::DummyRenderer},
        textures::texture_repo::TextureRepository,
        utilities::math::Vec3,
    };

    const EPSILON: f64 = 0.0001;

    fn image_eq(image1: &Vec<Vec<Vec3>>, image2: &Vec<Vec<Vec3>>) -> bool {
        if image1.len() != image2.len() {
            return false;
        }
        if image1[0].len() != image2[0].len() {
            return false;
        }
        for y in 0..image1.len() {
            for x in 0..image1[y].len() {
                let c1 = image1[y][x];
                let c2 = image2[y][x];
                if (c1 - c2).inner.iter().any(|a| *a > EPSILON) {
                    //Difference larger than error margin
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn render() {
        let repo = TextureRepository::new();
        let collector = ArrayCollector {};
        let renderer = DummyRenderer {};
        let image = collector.collect(renderer, &repo, 4, 4);
        let expected = vec![
            vec![0.0 / 6.0, 1.0 / 6.0, 2.0 / 6.0, 3.0 / 6.0],
            vec![1.0 / 6.0, 2.0 / 6.0, 3.0 / 6.0, 4.0 / 6.0],
            vec![2.0 / 6.0, 3.0 / 6.0, 4.0 / 6.0, 5.0 / 6.0],
            vec![3.0 / 6.0, 4.0 / 6.0, 5.0 / 6.0, 6.0 / 6.0],
        ];
        let expected: Vec<Vec<Vec3>> = expected
            .iter()
            .map(|row| row.iter().map(|c| Vec3::new(*c, *c, *c)).collect())
            .collect();
        //Make sure they are the same value, given a certain error margin
        assert!(image_eq(&expected, &image));
    }
}

mod image_collector {
    use crate::{
        api::fragment_collector::FragmentCollector, collector::image_collector::ImageCollector,
        textures::texture_repo::TextureRepository,
    };

    use super::DummyRenderer;

    #[test]
    fn render() {
        let repo = TextureRepository::new();
        let collector = ImageCollector {};
        let renderer = DummyRenderer {};
        let image = collector.collect(renderer, &repo, 4, 4).unwrap();
        assert_eq!(image.get_pixel(0, 0).0[0], 0);
        assert_eq!(image.get_pixel(1, 1).0[0], 85);
        assert_eq!(image.get_pixel(3, 1).0[0], 170);
    }
}
