use std::os::raw::c_void;

#[derive(Debug)]
pub(crate) struct Mesh {
    vao: u32,
    vbo: Box<[u32]>,
    ibo: u32,
    index_count: i32,
}

impl Mesh {
    pub(crate) fn new(vertices: &[f32], indices: &[u32], attribute_sizes: &[i32]) -> Self {
        let mut vao = 0;
        let mut vbos = vec![0; attribute_sizes.len()].into_boxed_slice();
        let mut ibo = 0;
        let floats_per_vertex: gl::types::GLsizei = attribute_sizes.iter().sum();
        let stride = floats_per_vertex * size_of::<f32>() as i32;
        let mut offset = 0;
        unsafe {
            // Vertex Array Object stores the buffer object and attribute configurations
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // vertex indices
            gl::GenBuffers(1, &mut ibo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size_of_val(indices) as isize, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);

            // Vertex Buffer Object
            gl::GenBuffers(vbos.len() as i32, vbos.as_mut_ptr());
            for (location, vbo) in vbos.iter().enumerate() {
                gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
                // sets data for the current buffer object. Current = the one that is bound
                gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertices) as isize, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

                /*
                sets attributes for the current buffer object (describe the data):
                index of the vertex attribute to be modified, location from the shader file
                we have vertices of 3 floats (vec3)
                we don't want to normalize them (so false)
                no stride (0); stride can be used to store data as structs (i.e. coords, color, next coords, next color)
                no offset from the start
                */
                let size = attribute_sizes[location];
                let location = location as u32;
                gl::VertexAttribPointer(location, size, gl::FLOAT, gl::FALSE, stride, offset as *const c_void);
                gl::EnableVertexAttribArray(location);
                offset += size * size_of::<f32>() as i32;
            }

            // vao stores the last bound values, so it needs to be unbound first, otherwise we will store zeroes
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        };
        Self {vao, vbo: vbos, ibo, index_count: indices.len().try_into().unwrap()}
    }
}
impl IMesh for Mesh {
    fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.index_count, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }
}
impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            for vbo in &self.vbo {
                gl::DeleteBuffers(1, vbo);
            }
            gl::DeleteBuffers(1, &self.ibo);
        }
    }
}

#[derive(Debug)]
pub(crate) struct OldMesh {
    vao: u32,
    vbo: u32,
    vertex_count: i32,
}

impl OldMesh {
    pub(crate) fn new(vertices: &[f32]) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        const VERTEX_SIZE: i32 = 3;
        unsafe {
            // Vertex Array Object stores the buffer object and attribute configurations
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Vertex Buffer Object
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            // sets data for the current buffer object. Current = the one that is bound
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertices) as isize, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

            /*
            sets attributes for the current buffer object (describe the data):
            index of the vertex attribute to be modified, location from the shader file
            we have vertices of 3 floats (vec3)
            we don't want to normalize them (so false)
            no stride (0); stride can be used to store data as structs (i.e. coords, color, next coords, next color)
            no offset from the start
            */
            const LOCATION: gl::types::GLuint = 0;
            gl::VertexAttribPointer(LOCATION, VERTEX_SIZE, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(LOCATION);

            // vao stores the last bound values, so it needs to be unbound first, otherwise we will store zeroes
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        };
        let vertex_count = vertices.len() as i32 / VERTEX_SIZE;
        Self {vao, vbo, vertex_count}
    }
}
impl IMesh for OldMesh {
    fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
            gl::BindVertexArray(0);
        }
    }
}
impl Drop for OldMesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

pub(crate) trait IMesh {
    fn render(&self) {}
}