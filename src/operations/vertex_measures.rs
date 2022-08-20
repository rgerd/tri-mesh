//! See [Mesh](crate::mesh::Mesh).

use std::collections::HashSet;

use crate::prelude::*;

/// # Vertex measures
impl Mesh {
    /// Returns the normal of the vertex given as the average of the normals of the neighbouring faces.
    pub fn vertex_normal(&self, vertex_id: VertexID) -> Vec3 {
        let mut normal = Vec3::zero();
        let mut seen_faces = HashSet::<FaceID>::new();
        let mut face_count = 0;
        for halfedge_id in self.vertex_halfedge_iter(vertex_id) {
            if let Some(face_id) = self.walker_from_halfedge(halfedge_id).face_id() {
                if !seen_faces.insert(face_id) {
                    println!("Hit a loop!");
                    break;
                }
                normal += self.face_normal(face_id);
                face_count += 1;
            }
        }
        assert!(face_count > 0);
        if normal.magnitude2() == 0.0 {
            println!("Got a zero mag!");
            normal = Vec3::unit_z();
        }
        normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MeshBuilder;

    #[test]
    fn test_vertex_normal() {
        let mesh = MeshBuilder::new().subdivided_triangle().build().unwrap();
        let computed_normal = mesh.vertex_normal(unsafe { VertexID::new(0) });
        assert_eq!(0.0, computed_normal.x);
        assert_eq!(0.0, computed_normal.y);
        assert_eq!(1.0, computed_normal.z);
    }
}
