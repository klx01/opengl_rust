use crate::mesh::*;

pub(crate) fn triangle() -> MeshNoIndices {
    let vertices = [
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0,
    ];
    MeshNoIndices::new_interleaved(&vertices, &[3])
}

pub(crate) fn two_triangles() -> MeshNoIndices {
    let vertices = [
        -1.0, -1.0, 0.0,
        -0.0, -1.0, 0.0,
        -0.5,  0.0, 0.0,

         0.0,  0.0, 0.0,
         1.0,  0.0, 0.0,
         0.5,  1.0, 0.0,
    ];
    MeshNoIndices::new_interleaved(&vertices, &[3])
}

pub(crate) fn two_triangles_split() -> [MeshNoIndices; 2] {
    let vertices = [
        -1.0, -1.0, 0.0,
        -0.0, -1.0, 0.0,
        -0.5,  0.0, 0.0,
    ];
    let first = MeshNoIndices::new_interleaved(&vertices, &[3]);
    let vertices = [
        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.5, 1.0, 0.0,
    ];
    let second = MeshNoIndices::new_interleaved(&vertices, &[3]);
    [first, second]
}

pub(crate) fn rectangle() -> MeshWithIndices {
    let vertices = [
         0.5,  0.5, 0.0,  // top right
         0.5, -0.5, 0.0,  // bottom right
        -0.5, -0.5, 0.0,  // bottom left
        -0.5,  0.5, 0.0,  // top left
    ];
    let indices = [
        0, 1, 3,   // first triangle
        1, 2, 3,   // second triangle
    ];
    MeshWithIndices::new_interleaved(&vertices, &indices, &[3])
}

pub(crate) fn rectangle_screen() -> MeshWithIndices {
    let vertices = [
         1.0,  1.0, 0.0,  // top right
         1.0, -1.0, 0.0,  // bottom right
        -1.0, -1.0, 0.0,  // bottom left
        -1.0,  1.0, 0.0,  // top left
    ];
    let indices = [
        0, 1, 3,   // first triangle
        1, 2, 3,   // second triangle
    ];
    MeshWithIndices::new_interleaved(&vertices, &indices, &[3])
}

pub(crate) fn multi_attr_indices_interleaved() -> MeshWithIndices {
    let vertices = [
        // positions      // colors
         0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
         0.0,  0.5, 0.0,  0.0, 0.0, 1.0,   // top 
    ];
    let indices = [0, 1, 2];
    MeshWithIndices::new_interleaved(&vertices, &indices, &[3, 3])
}

pub(crate) fn multi_attr_indices_batched() -> MeshWithIndices {
    let vertices = [
         // positions
         0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0,
         0.0,  0.5, 0.0,
         // colors
         1.0, 0.0, 0.0,   // bottom right
         0.0, 1.0, 0.0,   // bottom left
         0.0, 0.0, 1.0,   // top
    ];
    let indices = [0, 1, 2];
    MeshWithIndices::new_batched(&vertices, &indices, &[3, 3])
}

pub(crate) fn multi_attr_no_indices_interleaved() -> MeshNoIndices {
    let vertices = [
         // positions     // colors
         0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
         0.0,  0.5, 0.0,  0.0, 0.0, 1.0,   // top
    ];
    MeshNoIndices::new_interleaved(&vertices, &[3, 3])
}

pub(crate) fn multi_attr_no_indices_batched() -> MeshNoIndices {
    let vertices = [
         // positions
         0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0,
         0.0,  0.5, 0.0,
         // colors
         1.0, 0.0, 0.0,   // bottom right
         0.0, 1.0, 0.0,   // bottom left
         0.0, 0.0, 1.0,   // top
    ];
    MeshNoIndices::new_batched(&vertices, &[3, 3])
}

pub(crate) fn rectangle_texture() -> MeshWithIndices {
    let vertices = [
         // positions      // colors        // texture coords
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 0.0,   // top right. top is 0, and bottom is 1. or instead flip the source image
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 1.0,   // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 1.0,   // bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 0.0,   // top left
    ];
    let indices = [
        0, 1, 3,   // first triangle
        1, 2, 3,   // second triangle
    ];
    MeshWithIndices::new_interleaved(&vertices, &indices, &[3, 3, 2])
}

pub(crate) fn cube0() -> MeshNoIndices {
    let vertices = [
        -0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  1.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,

        -0.5, -0.5,  0.5,  0.0, 1.0,
         0.5, -0.5,  0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 1.0,

        -0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  1.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 1.0,
        -0.5,  0.5,  0.5,  1.0, 1.0,

         0.5,  0.5,  0.5,  1.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 0.0,
         0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5,  0.5,  0.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 1.0,

        -0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 1.0,
         0.5, -0.5,  0.5,  1.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,

        -0.5,  0.5, -0.5,  0.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 0.0,
    ];
    MeshNoIndices::new_interleaved(&vertices, &[3, 2])
}

pub(crate) fn cube() -> MeshWithIndices {
    let vertices = [
        -0.5, -0.5, -0.5,  0.0, 0.0, // left bottom far
         0.5, -0.5, -0.5,  1.0, 0.0, // right bottom far
         0.5,  0.5, -0.5,  1.0, 1.0, // right top far
        -0.5,  0.5, -0.5,  0.0, 1.0, // left top far
        -0.5, -0.5,  0.5,  0.0, 1.0, // left bottom close
         0.5, -0.5,  0.5,  1.0, 1.0, // right bottom close
         0.5,  0.5,  0.5,  1.0, 0.0, // right top close
        -0.5,  0.5,  0.5,  0.0, 0.0, // left top close
        // todo: duplicate vertices with different uvs, use their indices for left and right faces
    ];
    let indices = [
        4, 5, 6, 4, 6, 7, // close
        0, 1, 2, 0, 2, 3, // far
        2, 3, 6, 3, 6, 7, // top
        0, 1, 4, 1, 4, 5, // bottom
        0, 3, 4, 3, 4, 7, // left
        1, 2, 5, 2, 5, 6, // right
    ];
    MeshWithIndices::new_interleaved(&vertices, &indices, &[3, 2])
}