pub const VERTEX_SOURCE: &str = r##"#version 300 es
precision highp float;

layout(location=0) in vec2 vertex_coord;

void main() {
	gl_Position = vec4(vertex_coord, 0., 1.);
}
"##;

pub const FRAGMENT_SOURCE: &str = r##"#version 300 es
precision mediump float;

uniform vec3 color;
out vec4 outColor;

void main() {
	outColor = vec4(color, 1.);
}
"##;

pub const VERTEX_SOURCE2: &str = r##"#version 300 es
precision highp float;

layout(location=0) in vec2 vertex_coord;

void main() {
	gl_Position = vec4(-vertex_coord, 0., 1.);
}
"##;

pub const FRAGMENT_SOURCE2: &str = r##"#version 300 es
precision mediump float;

uniform vec3 color;
out vec4 outColor;

void main() {
	outColor = vec4(color, 1.);
}
"##;


pub const VERTEX_SOURCE3: &str = r##"#version 300 es
precision highp float;

layout(location=0) in vec2 vertex_coord;
layout(location=1) in vec2 instance_coord;
layout(location=2) in vec3 color;

out vec3 f_color;

void main() {
	f_color = color;
	gl_Position = vec4(vertex_coord + instance_coord, 0., 1.);
}
"##;

pub const FRAGMENT_SOURCE3: &str = r##"#version 300 es
precision mediump float;

in vec3 f_color;
out vec4 outColor;

void main() {
	outColor = vec4(f_color, 1.);
}
"##;
