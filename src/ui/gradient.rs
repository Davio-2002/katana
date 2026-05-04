pub fn lerp(a: u8, b: u8, t: f32) -> u8 {
    let (a, b) = (a as f32, b as f32);
    (a + (b - a) * t) as u8
}

// 4-stop gradient
pub fn sea_color(t: f32) -> (u8, u8, u8) {
    let stops: [(u8, u8, u8); 4] = [
        (15, 15, 80),    // deep indigo ->
        (0, 80, 160),    // -> ocean blue ->
        (0, 180, 216),   // -> sea blue ->
        (202, 240, 248), // -> seafoam crest
    ];

    let scaled = t * (stops.len() - 1) as f32;
    let i = (scaled as usize).min(stops.len() - 2);
    let local_t = scaled - i as f32;

    let (r1, g1, b1) = stops[i];
    let (r2, g2, b2) = stops[i + 1];

    (
        lerp(r1, r2, local_t),
        lerp(g1, g2, local_t),
        lerp(b1, b2, local_t),
    )
}

pub fn gradient_line(line: &str, t_start: f32, t_end: f32) -> String {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    if len == 0 {
        return String::new();
    }
    chars
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let t = t_start + (t_end - t_start) * (i as f32 / len as f32);
            let (r, g, b) = sea_color(t);
            format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, c)
        })
        .collect()
}
