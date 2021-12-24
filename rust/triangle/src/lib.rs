#[derive(PartialEq)]
enum TriangleType {
    Equilateral,
    Isosceles,
    Scalene
}

pub struct Triangle {
    sides: [u64; 3],
    triangle_type: TriangleType
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut sides_vec = sides;
        sides_vec.sort_unstable();

        if sides_vec[2] > sides_vec[1] + sides_vec[0] || sides_vec.contains(&0){
            return None;
        }

        let triangle_type = match (sides_vec[0] == sides_vec[1], sides_vec[1] == sides_vec[2], sides_vec[2] == sides_vec[0]) {
            (true, true, true) => TriangleType::Equilateral,
            (false, false, false) => TriangleType::Scalene,
            _ => TriangleType::Isosceles
        };

        Some(Triangle{sides: sides_vec, triangle_type})
    }

    pub fn is_equilateral(&self) -> bool {
        self.triangle_type == TriangleType::Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.triangle_type == TriangleType::Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.triangle_type == TriangleType::Isosceles
    }
}
