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

        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.5, 1.0, 0.0,
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

pub(crate) fn multi_attr_indices_interleaved() -> MeshWithIndices {
    let vertices = [
        // positions      // colors
         0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
         0.0,  0.5, 0.0,  0.0, 0.0, 1.0    // top 
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
        0.0, 0.0, 1.0    // top
    ];
    let indices = [0, 1, 2];
    MeshWithIndices::new_batched(&vertices, &indices, &[3, 3])
}

pub(crate) fn multi_attr_no_indices_interleaved() -> MeshNoIndices {
    let vertices = [
        // positions      // colors
         0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
         0.0,  0.5, 0.0,  0.0, 0.0, 1.0    // top 
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
        0.0, 0.0, 1.0    // top
    ];
    MeshNoIndices::new_batched(&vertices, &[3, 3])
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