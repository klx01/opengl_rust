use std::os::raw::c_void;

pub(crate) trait Mesh {
    fn render(&self);
}

fn create_and_bind_vao() -> u32 {
    // Vertex Array Object stores the buffer object and attribute configurations
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }
    vao
}

fn create_and_bind_vbo<T: Sized>(vertices: &[T]) -> u32 {
    // Vertex Buffer Object
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // sets data for the current buffer object. Current = the one that is bound
        gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertices) as isize, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);
    };
    vbo
}

fn create_and_bind_ibo(indices: &[u32]) -> u32 {
    // vertex indices
    let mut ibo = 0;
    unsafe {
        gl::GenBuffers(1, &mut ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size_of_val(indices) as isize, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);
    };
    ibo
}

fn set_attributes_interleaved(attribute_sizes: &[i32]) {
    let floats_per_vertex: i32 = attribute_sizes.iter().sum();
    let stride = floats_per_vertex * size_of::<f32>() as i32;
    let mut offset = 0;
    let float_size = size_of::<f32>() as i32;
    for (location, &size) in attribute_sizes.iter().enumerate() {
        /*
        sets attributes for the current buffer object (describe the data):
        index of the vertex attribute to be modified, location from the shader file
        we have vertices of 3 floats (vec3)
        we don't want to normalize them (so false)
        no stride (0); stride can be used to store data as structs (i.e. coords, color, next coords, next color)
        no offset from the start
        */
        let location = location as u32;
        unsafe {
            gl::VertexAttribPointer(location, size, gl::FLOAT, gl::FALSE, stride, offset as *const c_void);
            gl::EnableVertexAttribArray(location);
        }
        offset += size * float_size;
    }
}

fn set_attributes_batched(attribute_sizes: &[i32], floats_count: usize) {
    let floats_per_vertex: i32 = attribute_sizes.iter().sum();
    let vertices_count = floats_count as i32 / floats_per_vertex;
    let float_size = size_of::<f32>() as i32;
    let mut offset = 0;
    for (location, &size) in attribute_sizes.iter().enumerate() {
        let location = location as u32;
        println!("offset {offset}");
        unsafe {
            gl::VertexAttribPointer(location, size, gl::FLOAT, gl::FALSE, 0, offset as *const c_void);
            gl::EnableVertexAttribArray(location);
        }
        offset += size * float_size * vertices_count;
    }
}

#[derive(Debug)]
pub(crate) struct MeshWithIndices {
    vao: u32,
    vbo: u32,
    ibo: u32,
    index_count: i32,
}

impl MeshWithIndices {
    pub(crate) fn new_interleaved(vertices: &[f32], indices: &[u32], attribute_sizes: &[i32]) -> Self {
        let vao = create_and_bind_vao();
        let vbo = create_and_bind_vbo(vertices);
        let ibo = create_and_bind_ibo(indices);
        set_attributes_interleaved(attribute_sizes);
        Self::unbind();
        Self {vao, vbo, ibo, index_count: indices.len() as i32}
    }
    pub(crate) fn new_batched(vertices: &[f32], indices: &[u32], attribute_sizes: &[i32]) -> Self {
        let vao = create_and_bind_vao();
        let vbo = create_and_bind_vbo(vertices);
        let ibo = create_and_bind_ibo(indices);
        set_attributes_batched(attribute_sizes, vertices.len());
        Self::unbind();
        Self {vao, vbo, ibo, index_count: indices.len() as i32}
    }
    fn unbind() {
        unsafe {
            // vao stores the last bound values, so it needs to be unbound first, otherwise we will store zeroes
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        };
    }
}
impl Mesh for MeshWithIndices {
    fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.index_count, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }
}
impl Drop for MeshWithIndices {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ibo);
        }
    }
}

#[derive(Debug)]
pub(crate) struct MeshNoIndices {
    vao: u32,
    vbo: u32,
    vertex_count: i32,
}

impl MeshNoIndices {
    pub(crate) fn new(vertices: &[f32]) -> Self {
        let vao = create_and_bind_vao();
        let vbo = create_and_bind_vbo(vertices);
        // todo: do multi attributes
        const VERTEX_SIZE: i32 = 3;
        unsafe {
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
        };
        Self::unbind();
        let vertex_count = vertices.len() as i32 / VERTEX_SIZE;
        Self {vao, vbo, vertex_count}
    }
    fn unbind() {
        unsafe {
            // vao stores the last bound values, so it needs to be unbound first, otherwise we will store zeroes
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}
impl Mesh for MeshNoIndices {
    fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
            gl::BindVertexArray(0);
        }
    }
}
impl Drop for MeshNoIndices {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}