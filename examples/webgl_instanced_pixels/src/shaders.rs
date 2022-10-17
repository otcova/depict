pub const VERTEX_SOURCE: &str = r##"#version 300 es
precision highp float;

layout(location=0) in vec2 vertex_coord;
layout(location=1) in vec3 instance_color;
layout(location=2) in mat3x2 transformation;

out vec3 color;

uniform vec2 screen_size;

void main() {
	color = instance_color;
	vec2 pos = (transformation * vec3(vertex_coord, 1.)).xy;
	gl_Position = vec4(pos / screen_size, 0., 1.);
}
"##;

pub const FRAGMENT_SOURCE: &str = r##"#version 300 es
precision mediump float;

in vec3 color;
out vec4 outColor;

void main() {
	outColor = vec4(color, 1.);
}
"##;
