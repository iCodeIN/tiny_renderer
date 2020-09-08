use super::core::Vec3f;

pub fn cross_product(vec1: Vec3f, vec2: Vec3f) -> Vec3f {
    Vec3f([
        vec1.y() * vec2.z() - vec1.z() * vec2.y(),
        vec1.z() * vec2.x() - vec1.x() * vec2.z(),
        vec1.x() * vec2.y() - vec1.y() * vec2.x(),
    ])
}

pub fn dot_product(vec1: Vec3f, vec2: Vec3f) -> f64 {
    (vec1.x() * vec2.x()) + (vec1.y() * vec2.y()) + (vec1.z() * vec2.z())
}

pub fn barycentric(simplex: Vec<&Vec3f>, point: &Vec3f) -> Vec3f {
    let perpendicular_vec = cross_product(
        Vec3f([
            simplex[2][0] - simplex[0][0],
            simplex[1][0] - simplex[0][0],
            simplex[0][0] - point[0],
        ]),
        Vec3f([
            simplex[2][1] - simplex[0][1],
            simplex[1][1] - simplex[0][1],
            simplex[0][1] - point[1],
        ]),
    );

    if perpendicular_vec[2].abs() < 1.0 {
        return Vec3f([-1.0, 1.0, 1.0]);
    } else {
        let normalized = perpendicular_vec / perpendicular_vec[2];
        Vec3f([
            1.0 - (normalized[0] + normalized[1]),
            normalized[1],
            normalized[0],
        ])
    }
}
