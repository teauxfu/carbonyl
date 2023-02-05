use crate::gfx::Color;

pub fn binarize(a: Color, b: Color, c: Color, d: Color) -> (&'static str, Color, Color) {
    // Step 1: grayscale
    let lum = Color::new(0.299, 0.587, 0.114);
    let (x, y, z, w) = (
        a.cast::<f32>().dot(lum),
        b.cast::<f32>().dot(lum),
        c.cast::<f32>().dot(lum),
        d.cast::<f32>().dot(lum),
    );
    // Step 2: luminance middlepoint
    let min = x.min(y).min(z).min(w);
    let max = x.max(y).max(z).max(w);
    let mid = min + (max - min) / 2.0;

    // Step 3: table lookup using binary threshold mask
    TABLE[((x > mid) as usize) << 0
        | ((y > mid) as usize) << 1
        | ((z > mid) as usize) << 2
        | ((w > mid) as usize) << 3](a, b, c, d)
}

const TABLE: [fn(Color, Color, Color, Color) -> (&'static str, Color, Color); 16] = [
    |x, y, z, w| (" ", x.avg_with(y).avg_with(z).avg_with(w), Color::black()),
    |x, y, z, w| ("▖", x.avg_with(y).avg_with(z), w),
    |x, y, z, w| ("▗", x.avg_with(y).avg_with(w), z),
    |x, y, z, w| ("▄", x.avg_with(y), z.avg_with(w)),
    |x, y, z, w| ("▝", x.avg_with(z).avg_with(w), y),
    |x, y, z, w| ("▞", x.avg_with(z), y.avg_with(w)),
    |x, y, z, w| ("▐", x.avg_with(w), y.avg_with(z)),
    |x, y, z, w| ("▟", x, y.avg_with(z).avg_with(w)),
    |x, y, z, w| ("▘", y.avg_with(z).avg_with(w), x),
    |x, y, z, w| ("▌", y.avg_with(z), x.avg_with(w)),
    |x, y, z, w| ("▚", y.avg_with(w), x.avg_with(z)),
    |x, y, z, w| ("▙", y, x.avg_with(z).avg_with(w)),
    |x, y, z, w| ("▄", x.avg_with(y), z.avg_with(w)),
    |x, y, z, w| ("▛", z, x.avg_with(y).avg_with(w)),
    |x, y, z, w| ("▜", w, x.avg_with(y).avg_with(z)),
    |x, y, z, w| ("█", Color::black(), x.avg_with(y).avg_with(z).avg_with(w)),
];
