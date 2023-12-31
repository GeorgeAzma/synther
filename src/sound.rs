// Play a frequency
fn f(t: f32, freq: f32) -> f32 {
    return (t * std::f32::consts::TAU * freq).sin();
}

// Play harmonic frequencies
fn h(t: f32, base_freq: f32, harmonics: &[(f32, f32)]) -> f32 {
    let mut w = 0.0;
    for (freq, amp) in harmonics {
        w += f(t, base_freq * freq) * amp
    }
    w
}

// Envelope
fn e(t: f32, start: f32, dur: f32, atk: f32) -> f32 {
    let dt = t - start;
    let mut x = dt / atk;
    x = x.clamp(0.0, 1.0);
    x = x * x * (3.0 - 2.0 * x);
    let amp = x.min(((atk - dt) / dur.sqrt()).exp());
    amp * amp
}

fn piano(t: f32, freq: f32) -> f32 {
    h(
        t,
        freq,
        &[(1.0, 0.34), (2.0, 0.455), (4.0, 0.17), (6.0, 0.035)],
    )
}

fn steins_gate(mut t: f32) -> f32 {
    t = t / 2.5;
    let p = |freq, start, dur| piano(t, freq) * e(t, start, dur, 0.001);
    let s = p(392.0, 0.02, 0.04)
        + p(466.16, 0.03, 0.04)
        + p(622.25, 0.04, 0.04)
        + p(293.66, 0.12, 0.04)
        + p(392.0, 0.20, 0.04)
        + p(466.16, 0.28, 0.04)
        + p(587.33, 0.36, 0.04)
        + p(293.66, 0.44, 0.04)
        + p(392.0, 0.52, 0.04)
        + p(466.16, 0.60, 0.04)
        + p(554.37, 0.68, 0.04)
        + p(293.66, 0.76, 0.04)
        + p(392.0, 0.84, 0.04)
        + p(466.16, 0.92, 0.04)
        + p(587.33, 1.0, 0.04)
        + p(293.66, 1.08, 0.04)
        + p(392.0, 1.16, 0.04)
        + p(466.16, 1.24, 0.04)
        + p(196.0, 0.04, 1.28)
        + p(622.25, 1.32, 0.04)
        + p(440.0, 1.32, 0.04)
        + p(293.66, 1.40, 0.04)
        + p(392.0, 1.48, 0.04)
        + p(440.0, 1.56, 0.04)
        + p(587.33, 1.64, 0.04)
        + p(293.66, 1.72, 0.04)
        + p(392.0, 1.80, 0.04)
        + p(440.0, 1.88, 0.04)
        + p(554.37, 1.96, 0.04)
        + p(293.66, 2.04, 0.04)
        + p(392.0, 2.12, 0.04)
        + p(440.0, 2.20, 0.04)
        + p(587.33, 2.28, 0.04)
        + p(293.66, 2.36, 0.04)
        + p(392.0, 2.44, 0.04)
        + p(440.0, 2.52, 0.04)
        + p(196.0, 1.32, 1.28)
        + p(622.25, 2.60, 0.04)
        + p(415.30, 2.60, 0.04)
        + p(261.63, 2.68, 0.04)
        + p(392.0, 2.76, 0.04)
        + p(415.30, 2.84, 0.04)
        + p(587.33, 2.92, 0.04)
        + p(261.63, 3.0, 0.04)
        + p(392.0, 3.08, 0.04)
        + p(415.30, 3.16, 0.04)
        + p(523.25, 3.24, 0.04)
        + p(261.63, 3.32, 0.04)
        + p(392.0, 3.40, 0.04)
        + p(415.30, 3.48, 0.04)
        + p(587.33, 3.56, 0.04)
        + p(261.63, 3.64, 0.04)
        + p(392.0, 3.72, 0.04)
        + p(415.30, 3.80, 0.04)
        + p(174.61, 2.60, 1.28)
        + p(349.23, 3.88, 0.04)
        + p(523.25, 3.88, 0.04)
        + p(220.0, 3.96, 0.04)
        + p(349.23, 4.04, 0.04)
        + p(392.0, 4.12, 0.04)
        + p(523.25, 4.20, 0.04)
        + p(220.0, 4.28, 0.04)
        + p(349.23, 4.36, 0.04)
        + p(392.0, 4.44, 0.04)
        + p(146.83, 3.88, 0.68)
        + p(196.0, 4.52, 0.04)
        + p(587.33, 4.52, 0.04)
        + p(493.88, 4.52, 0.04)
        + p(293.66, 4.68, 0.04)
        + p(349.23, 4.76, 0.04)
        + p(174.61, 4.84, 0.04)
        + p(246.94, 4.86, 0.04)
        + p(698.46, 4.88, 0.04)
        + p(622.25, 4.96, 0.04)
        + p(587.33, 5.04, 0.04)
        + p(523.25, 5.12, 0.04)
        + p(196., 5.20, 0.04)
        + p(293.66, 5.28, 0.04)
        + p(440., 5.36, 0.04)
        + p(466.16, 5.44, 0.04)
        // UGLY PART
        + p(1244.51, 5.20, 0.32)
        + p(622.25, 5.20, 0.32)
        + p(196., 5.52, 0.04)
        + p(293.66, 5.60, 0.04)
        + p(440., 5.68, 0.04)
        + p(466.16, 5.76, 0.04)
        + p(587.33, 5.52, 0.32)
        + p(1174.66, 5.52, 0.32)
        + p(196., 5.84, 0.04)
        + p(293.66, 5.92, 0.04)
        + p(440., 6., 0.04)
        + p(466.16, 6.08, 0.04)
        + p(554.37, 5.84, 0.32)
        + p(1108.73, 5.84, 0.32)
        + p(196., 6.16, 0.04)
        + p(293.66, 6.24, 0.04)
        + p(440., 6.32, 0.04)
        + p(466.16, 6.40, 0.04)
        + p(587.33, 6.16, 0.32)
        + p(1174.66, 6.16, 0.32)
        + p(196., 6.48, 0.04)
        + p(277.18, 6.56, 0.04)
        + p(392., 6.64, 0.04)
        + p(440., 6.72, 0.04)
        + p(1244.51, 6.48, 0.32)
        + p(622.25, 6.48, 0.32)
        + p(196., 6.80, 0.04)
        + p(277.18, 6.88, 0.04)
        + p(392., 6.96, 0.04)
        + p(440., 7.04, 0.04)
        + p(1174.66, 6.80, 0.32)
        + p(587.33, 6.80, 0.32)
        + p(196., 7.12, 0.04)
        + p(277.18, 7.20, 0.04)
        + p(392., 7.28, 0.04)
        + p(440., 7.36, 0.04)
        + p(1108.73, 7.12, 0.32)
        + p(554.37, 7.12, 0.32)
        + p(196., 7.44, 0.04)
        + p(277.18, 7.52, 0.04)
        + p(392., 7.60, 0.04)
        + p(440., 7.68, 0.04)
        + p(587.33, 7.44, 0.32)
        + p(1174.66, 7.44, 0.32)
        + p(622.25, 7.72, 0.04)
        + p(830.61, 7.74, 0.04)
        + p(174.61, 7.76, 0.04)
        + p(261.63, 7.84, 0.04)
        + p(392., 7.92, 0.04)
        + p(415.30, 8., 0.04)
        + p(1046.50, 7.76, 0.32)
        + p(523.25, 8.16, 0.04)
        + p(698.46, 8.24, 0.04)
        + p(783.99, 8.32, 0.04)
        + p(207.65, 8.40, 0.32)
        + p(392., 8.40, 0.32)
        + p(523.25, 8.40, 0.32)
        + p(1046.50, 8.40, 0.32)
        + p(329.63, 8.40, 0.32)
        + p(196., 8.72, 0.32)
        + p(130.81, 8.72, 0.32)
        + p(932.33, 8.72, 0.32)
        + p(466.16, 8.72, 0.32)
        + p(174.61, 9.08, 0.04)
        + p(261.63, 9.16, 0.04)
        + p(311.13, 9.24, 0.04)
        + p(392., 9.32, 0.04)
        + p(415.30, 9.04, 0.68)
        + p(523.25, 9.06, 0.66)
        + p(783.99, 9.08, 0.64)
        + p(87.31, 9.72, 0.68)
        + p(196., 9.72, 1.28)
        + p(261.63, 9.72, 1.28)
        + p(207.65, 9.72, 1.28)
        + p(130.81, 10.84, 0.16)
        + p(87.31, 11., 0.24)
        + p(349.23, 11., 0.24)
        + p(207.65, 11.24, 0.40)
        + p(196., 11.24, 0.40)
        + p(130.81, 11.24, 0.40)
        + p(87.31, 11.64, 0.24)
        + p(261.63, 11.24, 0.80)
        + p(349.23, 12.04, 0.04)
        + p(261.63, 12.12, 0.04)
        + p(349.23, 12.20, 0.04)
        + p(130.81, 11.88, 0.40)
        + p(196., 11.88, 0.40)
        + p(207.65, 11.88, 0.40)
        + p(130.81, 12.28, 0.24)
        + p(392., 12.28, 0.24)
        + p(196., 12.52, 0.40)
        + p(155.56, 12.52, 0.40)
        + p(233.08, 12.52, 0.40)
        + p(130.81, 12.92, 0.24)
        + p(261.63, 12.52, 0.80)
        + p(392., 13.32, 0.04)
        + p(261.63, 13.40, 0.04)
        + p(392., 13.48, 0.04)
        + p(196., 13.16, 0.40)
        + p(233.08, 13.16, 0.40)
        + p(155.56, 13.16, 0.40)
        + p(155.56, 13.56, 0.24)
        + p(415.30, 13.56, 0.40)
        + p(415.30, 13.96, 0.04)
        + p(466.16, 14.04, 0.04)
        + p(415.30, 14.12, 0.04)
        + p(233.08, 13.80, 0.40)
        + p(207.65, 13.80, 0.40)
        + p(174.61, 13.80, 0.40)
        + p(77.78, 14.20, 0.24)
        + p(392., 14.20, 0.40)
        + p(392., 14.60, 0.04)
        + p(622.25, 14.68, 0.04)
        + p(523.25, 14.76, 0.04)
        + p(155.56, 14.44, 0.40)
        + p(196., 14.44, 0.40)
        + p(233.08, 14.44, 0.40)
        + p(87.31, 14.84, 0.24)
        + p(233.08, 15.32, 0.04)
        + p(196., 15.08, 0.36);
    s
}

pub fn sound(t: f32) -> f32 {
    steins_gate(t) * 0.5
}
