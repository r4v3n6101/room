use file::wad::parser::level::Level;

pub type Vec3 = [f32; 3];
pub struct Model {
    vertices: Vec<Vec3>,
    faces: Vec<Vec<usize>>,
}

impl Model {
    pub fn from_level(level: &Level) -> Self {
        Self {
            vertices: vec![],
            faces: vec![],
        }
    }

    pub fn into_obj_str(self) -> String {
        let mut output = String::new();
        self.vertices
            .into_iter()
            .for_each(|v| output.push_str(&format!("v {} {} {}\n", v[0], v[1], v[2])));
        self.faces.into_iter().for_each(|f| {
            output.push_str(&format!(
                "f {}\n",
                f.into_iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ))
        });
        output
    }
}
