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
