use image::{ImageBuffer, Rgba};

fn point_in_poly(x: f32, y: f32, poly: &[(f32, f32)]) -> bool {
    let mut inside = false;
    let n = poly.len();
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = poly[i];
        let (xj, yj) = poly[j];
        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn dist_to_segment(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let l2 = (b.0 - a.0).powi(2) + (b.1 - a.1).powi(2);
    if l2 == 0.0 {
        return ((p.0 - a.0).powi(2) + (p.1 - a.1).powi(2)).sqrt();
    }
    let t = (((p.0 - a.0) * (b.0 - a.0) + (p.1 - a.1) * (b.1 - a.1)) / l2).clamp(0.0, 1.0);
    let proj = (a.0 + t * (b.0 - a.0), a.1 + t * (b.1 - a.1));
    ((p.0 - proj.0).powi(2) + (p.1 - proj.1).powi(2)).sqrt()
}

fn dist_to_poly_boundary(p: (f32, f32), poly: &[(f32, f32)]) -> f32 {
    let mut min_d = f32::MAX;
    for i in 0..poly.len() {
        let next = (i + 1) % poly.len();
        let d = dist_to_segment(p, poly[i], poly[next]);
        if d < min_d {
            min_d = d;
        }
    }
    min_d
}

fn create_icon(size: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(size, size);
    let scale = size as f32 / 256.0;

    // Stylized lightning bolt polygon:
    // Top tip: sharp angle pointing up-left like '<'
    // Bottom tip: sharp angle pointing down-right like '>'
    let lightning_poly: Vec<(f32, f32)> = vec![
        // Top start
        (165.0 * scale, 35.0 * scale),
        // Angle of '<' (top vertex pointing left)
        (75.0 * scale, 95.0 * scale),
        // Inner notch from '<' into central bolt
        (130.0 * scale, 105.0 * scale),
        // Downward lightning step
        (100.0 * scale, 140.0 * scale),
        // Lightning right arm
        (158.0 * scale, 130.0 * scale),
        // Heading towards '>' (bottom vertex pointing right)
        (120.0 * scale, 168.0 * scale),
        (185.0 * scale, 175.0 * scale),
        // Final exit pointing back down like '>'
        (95.0 * scale, 222.0 * scale),
        // Return path
        (140.0 * scale, 172.0 * scale),
        (115.0 * scale, 165.0 * scale),
        (155.0 * scale, 122.0 * scale),
        (115.0 * scale, 115.0 * scale),
        (145.0 * scale, 75.0 * scale),
    ];

    let center = size as f32 / 2.0;
    let radius = size as f32 * 0.46;

    for y in 0..size {
        for x in 0..size {
            let px = x as f32;
            let py = y as f32;

            // Distance from center for rounded badge
            let dx = px - center;
            let dy = py - center;
            // Squircle distance: (x^4 + y^4)^(1/4)
            let squircle_dist = (dx.powi(4) + dy.powi(4)).powf(0.25);

            if squircle_dist > radius + 1.0 {
                // Transparent outer
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                continue;
            }

            // Background squircle badge with subtle dark gradient
            let bg_t = (py / size as f32).clamp(0.0, 1.0);
            let bg_r = (24.0 + bg_t * 8.0) as u8;
            let bg_g = (28.0 + bg_t * 6.0) as u8;
            let bg_b = (36.0 + bg_t * 12.0) as u8;
            let mut color = Rgba([bg_r, bg_g, bg_b, 255]);

            // Subtle border around squircle
            if squircle_dist > radius - 2.5 * scale {
                let border_alpha = ((radius + 1.0 - squircle_dist) / (3.5 * scale)).clamp(0.0, 1.0);
                color = Rgba([
                    (55.0 * border_alpha + bg_r as f32 * (1.0 - border_alpha)) as u8,
                    (75.0 * border_alpha + bg_g as f32 * (1.0 - border_alpha)) as u8,
                    (105.0 * border_alpha + bg_b as f32 * (1.0 - border_alpha)) as u8,
                    255,
                ]);
            }

            // Check lightning bolt
            let inside = point_in_poly(px, py, &lightning_poly);
            let dist_bound = dist_to_poly_boundary((px, py), &lightning_poly);

            // Glow around lightning
            if !inside && dist_bound < 8.0 * scale {
                let glow_strength = 1.0 - (dist_bound / (8.0 * scale));
                let t_vert = (py / size as f32).clamp(0.0, 1.0);
                // Cyan glow at top (<), Amber/Orange glow at bottom (>)
                let glow_r = 0.0 * (1.0 - t_vert) + 245.0 * t_vert;
                let glow_g = 200.0 * (1.0 - t_vert) + 160.0 * t_vert;
                let glow_b = 255.0 * (1.0 - t_vert) + 20.0 * t_vert;

                let blend = glow_strength * 0.45;
                color = Rgba([
                    (color[0] as f32 * (1.0 - blend) + glow_r * blend) as u8,
                    (color[1] as f32 * (1.0 - blend) + glow_g * blend) as u8,
                    (color[2] as f32 * (1.0 - blend) + glow_b * blend) as u8,
                    255,
                ]);
            }

            if inside || dist_bound < 1.0 * scale {
                let t = (py / size as f32).clamp(0.0, 1.0);
                // Gradient: Vibrant Electric Cyan (#00E5FF) to Amber Gold (#FFB300)
                let r1 = 0.0;
                let g1 = 229.0;
                let b1 = 255.0;

                let r2 = 255.0;
                let g2 = 180.0;
                let b2 = 10.0;

                let bolt_r = r1 * (1.0 - t) + r2 * t;
                let bolt_g = g1 * (1.0 - t) + g2 * t;
                let bolt_b = b1 * (1.0 - t) + b2 * t;

                // Center highlight (white-hot core)
                let core_t = (dist_bound / (6.0 * scale)).clamp(0.0, 1.0);
                let final_r = bolt_r * (1.0 - core_t * 0.5) + 255.0 * (core_t * 0.5);
                let final_g = bolt_g * (1.0 - core_t * 0.5) + 255.0 * (core_t * 0.5);
                let final_b = bolt_b * (1.0 - core_t * 0.5) + 255.0 * (core_t * 0.5);

                let aa = if inside {
                    1.0
                } else {
                    (1.0 - dist_bound / (1.0 * scale)).clamp(0.0, 1.0)
                };

                color = Rgba([
                    (color[0] as f32 * (1.0 - aa) + final_r * aa) as u8,
                    (color[1] as f32 * (1.0 - aa) + final_g * aa) as u8,
                    (color[2] as f32 * (1.0 - aa) + final_b * aa) as u8,
                    255,
                ]);
            }

            img.put_pixel(x, y, color);
        }
    }

    img
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating icon assets...");
    let icon_256 = create_icon(256);
    icon_256.save("icon.png")?;
    println!("Saved icon.png");

    icon_256.save("icon.ico")?;
    println!("Saved icon.ico successfully!");

    let icon_64 = create_icon(64);
    let mut raw_bytes = Vec::new();
    raw_bytes.extend_from_slice(&64u32.to_le_bytes());
    raw_bytes.extend_from_slice(&64u32.to_le_bytes());
    raw_bytes.extend_from_slice(icon_64.as_raw());
    std::fs::write("icon_raw.bin", raw_bytes)?;
    println!("Saved icon_raw.bin ({} bytes)", 8 + 64 * 64 * 4);
    Ok(())
}
