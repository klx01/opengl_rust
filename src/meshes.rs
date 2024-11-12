use crate::mesh::*;

pub(crate) fn rectangle() -> MeshWithIndices {
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
    MeshWithIndices::new_interleaved(&vertices, &indices, &[3])
}

pub(crate) fn rectangle_screen() -> MeshWithIndices {
    let vertices = [
        1.0,  1.0, 0.0,  // top right
        1.0, -1.0, 0.0,  // bottom right
        -1.0, -1.0, 0.0,  // bottom left
        -1.0,  1.0, 0.0   // top left
    ];
    let indices = [
        0, 1, 3,   // first triangle
        1, 2, 3,   // second triangle
    ];
    MeshWithIndices::new_interleaved(&vertices, &indices, &[3])
}

pub(crate) fn multi_attr_interleaved() -> MeshWithIndices {
    let vertices = [
        // positions      // colors
         0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
         0.0,  0.5, 0.0,  0.0, 0.0, 1.0    // top 
    ];
    let indices = [0, 1, 2];
    MeshWithIndices::new_interleaved(&vertices, &indices, &[3, 3])
}

pub(crate) fn multi_attr_batched() -> MeshWithIndices {
    let vertices = [
        // positions      
         0.5, -0.5, 0.0,  
        -0.5, -0.5, 0.0,  
         0.0,  0.5, 0.0,
        // colors
        1.0, 0.0, 0.0,   // bottom right
        0.0, 1.0, 0.0,   // bottom left
        0.0, 0.0, 1.0    // top
    ];
    let indices = [0, 1, 2];
    MeshWithIndices::new_batched(&vertices, &indices, &[3, 3])
}

pub(crate) fn rectangle_texture() -> MeshWithIndices {
    let vertices = [
        // positions       // colors        // texture coords
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 0.0,   // top right. top is 0, and bottom is 1. or instead flip the source image
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 1.0,   // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 1.0,   // bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 0.0    // top left
    ];
    let indices = [
        0, 1, 3,   // first triangle
        1, 2, 3,   // second triangle
    ];
    MeshWithIndices::new_interleaved(&vertices, &indices, &[3, 3, 2])
}

pub(crate) fn old_triangle() -> MeshNoIndices {
    let vertices = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0,
    ];
    MeshNoIndices::new(&vertices)
}

pub(crate) fn two_triangles_old() -> MeshNoIndices {
    let vertices = [
        -1.0, -1.0, 0.0,
        -0.0, -1.0, 0.0,
        -0.5,  0.0, 0.0,

        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.5, 1.0, 0.0,
    ];
    MeshNoIndices::new(&vertices)
}

pub(crate) fn two_triangles_old_split() -> [MeshNoIndices; 2] {
    let vertices = [
        -1.0, -1.0, 0.0,
        -0.0, -1.0, 0.0,
        -0.5,  0.0, 0.0,
    ];
    let first = MeshNoIndices::new(&vertices);
    let vertices = [
        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.5, 1.0, 0.0,
    ];
    let second = MeshNoIndices::new(&vertices);
    [first, second]
}