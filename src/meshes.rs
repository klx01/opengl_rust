use crate::mesh::*;

pub(crate) fn rectangle() -> Mesh {
    let vertices = [
        0.5,  0.5, 0.0,  // top right
        0.5, -0.5, 0.0,  // bottom right
        -0.5, -0.5, 0.0,  // bottom left
        -0.5,  0.5, 0.0   // top left
    ];
    let indices = [
        0, 1, 3,   // first triangle
        1, 2, 3,    // second triangle
    ];
    Mesh::new(&vertices, &indices, &[3])
}

pub(crate) fn rectangle_screen() -> Mesh {
    let vertices = [
        1.0,  1.0, 0.0,  // top right
        1.0, -1.0, 0.0,  // bottom right
        -1.0, -1.0, 0.0,  // bottom left
        -1.0,  1.0, 0.0   // top left
    ];
    let indices = [
        0, 1, 3,   // first triangle
        1, 2, 3,    // second triangle
    ];
    Mesh::new(&vertices, &indices, &[3])
}

pub(crate) fn multi_attr() -> Mesh {
    let vertices = [
        // positions         // colors
         0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
         0.0,  0.5, 0.0,  0.0, 0.0, 1.0    // top 
    ];
    let indices = [0, 1, 2];
    Mesh::new(&vertices, &indices, &[3, 3])
}

pub(crate) fn old_triangle() -> OldMesh {
    let vertices = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0,
    ];
    OldMesh::new(&vertices)
}

pub(crate) fn two_triangles_old() -> OldMesh {
    let vertices = [
        -1.0, -1.0, 0.0,
        -0.0, -1.0, 0.0,
        -0.5,  0.0, 0.0,

        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.5, 1.0, 0.0,
    ];
    OldMesh::new(&vertices)
}

pub(crate) fn two_triangles_old_split() -> [OldMesh; 2] {
    let vertices = [
        -1.0, -1.0, 0.0,
        -0.0, -1.0, 0.0,
        -0.5,  0.0, 0.0,
    ];
    let first = OldMesh::new(&vertices);
    let vertices = [
        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.5, 1.0, 0.0,
    ];
    let second = OldMesh::new(&vertices);
    [first, second]
}