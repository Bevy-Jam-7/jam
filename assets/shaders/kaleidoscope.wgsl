#import bevy_ui::ui_vertex_output::UiVertexOutput;

struct KaleidoscopeSettings {
    resolution: vec2<f32>,
    time: f32,
    _padding: f32,
};

@group(1) @binding(0) var<uniform> settings: KaleidoscopeSettings;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // Reference: https://www.shadertoy.com/view/fdS3Dy
    // Mess with different numbers and functions and see what you can cook up :)

    let resolution = settings.resolution;
    let time = settings.time;

    // Convert UV to pixel coordinates
    let frag_coord = in.uv * resolution.xy;

    // Center coordinates
    var coord = frag_coord - (resolution.xy * 0.5);
    let x = coord.x;
    let y = coord.y;

    // Time modulation
    let j_time = glsl_mod(4 * sin(0.5 * time), 261.8) + 4.0;
    coord *= pow(1.1, j_time);
    
    // Radial distances
    let eps = 0.001;
    let r2 = abs((x * x + y * y) / max(abs(x), eps));
    let r3 = abs((x * x + y * y) / max(abs(y), eps));
    let r4 = abs((x * x + y * y) / max(abs(x - y), eps)) * sqrt(2.0);
    let r5 = abs((x * x + y * y) / max(abs(x + y), eps)) * sqrt(2.0);

    // Pattern scaling
    let p2 = pow(16.0, 6.0 - ceil(log2(r2) / 4.0));
    let p3 = pow(16.0, 6.0 - ceil(log2(r3) / 4.0));
    let p4 = pow(16.0, 6.0 - ceil(log2(r4) / 4.0));
    let p5 = pow(16.0, 6.0 - ceil(log2(r5) / 4.0));

    // Integer patterns
    let a = i32(floor(r2 * p2));
    let b = i32(floor(r3 * p3));
    let c = i32(floor(r4 * p4));
    let d = i32(floor(r5 * p5));

    // Combine patterns with XOR
    let e = (a | b) ^ (c | d);
    
    // Color mapping
    let value = fract(f32(e) * 0.000003);
    let color = hsv2rgb(vec3(value + settings.time * 0.6, 0.6, 0.6));

    return vec4(color, 1.0);
}

fn glsl_mod(a: f32, b: f32) -> f32 {
    return a - b * floor(a / b);
}

fn hsv2rgb(c: vec3<f32>) -> vec3<f32> {
    let K = vec4(1.0, 2.0/3.0, 1.0/3.0, 3.0);
    let p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, vec3(0.0), vec3(1.0)), c.y);
}
