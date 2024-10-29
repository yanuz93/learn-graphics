#[allow(unused_imports)]
use glfw::*;

#[allow(internal_features)]
fn main() {
    let mut init = glfw::init_no_callbacks().expect("failed to initialize GLFW");

    init.window_hint(WindowHint::ContextVersion(4, 1));
    init.window_hint(WindowHint::OpenGlForwardCompat(true));
    init.window_hint(glfw::WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = init
        .create_window(800, 600, "Yanuz's Graphics", glfw::WindowMode::Windowed)
        .expect("failed to create window");

    window.make_current();
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    while !window.should_close() {
        init.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close => window.set_should_close(true),
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            assert_ne!(vao, 0);

            gl::BindVertexArray(vao);

            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            assert_ne!(vbo, 0);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            type Vertex = [f32; 3];
            const VERTICES: [Vertex; 3] = [[0.0, 0.5, 0.0], [0.5, -0.5, 0.0], [-0.5, -0.5, 0.0]];

            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&VERTICES) as isize,
                VERTICES.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>()
                    .try_into()
                    .expect("failed to count vertex size"),
                0 as *const _,
            );

            gl::EnableVertexAttribArray(0);

            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(vertex_shader, 0);

            const VERT_SHADER: &str = r#"#version 410 core// Vertex Shader
            layout(location = 0) in vec3 pos;
            void main() {
                gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
            }
            "#;

            gl::ShaderSource(
                vertex_shader,
                1,
                &(VERT_SHADER.as_ptr() as *const _),
                &(VERT_SHADER
                    .len()
                    .try_into()
                    .expect("failed to count vertex shader length")),
            );

            gl::CompileShader(vertex_shader);

            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == gl::FALSE as i32 {
                let mut len = 0;
                gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0; len as usize];
                gl::GetShaderInfoLog(
                    vertex_shader,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut _,
                );
                let error = String::from_utf8(buffer).expect("failed to convert error to string");
                eprintln!("Failed to compile vertex shader: {}", error);
            }

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            assert_ne!(fragment_shader, 0);

            const FRAG_SHADER: &str = r#"#version 410 core// Fragment Shader
            out vec4 color;
            void main() {
                color = vec4(1.0, 0.5, 0.2, 1.0);
            }
            "#;

            gl::ShaderSource(
                fragment_shader,
                1,
                &(FRAG_SHADER.as_ptr() as *const _),
                &(FRAG_SHADER
                    .len()
                    .try_into()
                    .expect("failed to count fragment shader length")),
            );

            gl::CompileShader(fragment_shader);

            let mut success_fragment = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success_fragment);
            if success_fragment == gl::FALSE as i32 {
                let mut len = 0;
                gl::GetShaderiv(fragment_shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0; len as usize];
                gl::GetShaderInfoLog(
                    fragment_shader,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut _,
                );
                let error = String::from_utf8(buffer).expect("failed to convert error to string");
                eprintln!("Failed to compile fragment shader: {}", error);
            }

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success_program = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success_program);
            if success_program == gl::FALSE as i32 {
                let mut len = 0;
                gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0; len as usize];
                gl::GetProgramInfoLog(
                    shader_program,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut _,
                );
                let error = String::from_utf8(buffer).expect("failed to convert error to string");
                eprintln!("Failed to link shader program: {}", error);
            };

            gl::UseProgram(shader_program);
            // gl::DeleteShader(vertex_shader);
            // gl::DeleteShader(fragment_shader);

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            window.swap_buffers();
        }
    }
}
