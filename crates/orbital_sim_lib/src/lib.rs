use nalgebra::{vector, SVector};
use emojis;

pub type Vector8<T> = SVector<T, 8>;
pub type State = Vector8<f64>;

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeNode {
    pub emoji: String,
    pub color: String,
    pub mass: f64,                  // Mass affecting gravitational force
    pub initial_position: [f64; 4], // Initial (x, y, z, w)
    pub initial_velocity: [f64; 4], // Initial (vx, vy, vz, vw)
}

pub fn get_orbit_nodes(count: usize) -> Vec<ThemeNode> {
    let base_nodes = vec![
        ThemeNode {
            emoji: emojis::get_by_shortcode("rocket").unwrap().to_string(),
            color: "rgba(255, 0, 0, 0.8)".to_string(),
            mass: 1.0,
            initial_position: [1.0, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.5, 0.3, 0.2],
        },
        ThemeNode {
            emoji: emojis::get_by_shortcode("scroll").unwrap().to_string(),
            color: "rgba(255, 255, 0, 0.8)".to_string(),
            mass: 1.5,
            initial_position: [1.2, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.45, 0.25, 0.15],
        },
        ThemeNode {
            emoji: emojis::get_by_shortcode("mag").unwrap().to_string(),
            color: "rgba(0, 255, 255, 0.8)".to_string(),
            mass: 0.8,
            initial_position: [0.9, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.55, 0.35, 0.25],
        },
        ThemeNode {
            emoji: emojis::get_by_shortcode("speech_balloon")
                .unwrap()
                .to_string(),
            color: "rgba(255, 0, 255, 0.8)".to_string(),
            mass: 1.2,
            initial_position: [1.1, 0.0, 0.0, 0.0],
            initial_velocity: [0.0, 0.48, 0.28, 0.18],
        },
    ];
    base_nodes.into_iter().take(count).collect()
}

pub fn derivatives(state: &State, _t: f64, k: f64, m: f64) -> State {
    let x = state[0];
    let y = state[1];
    let z = state[2];
    let w = state[3];
    let vx = state[4];
    let vy = state[5];
    let vz = state[6];
    let vw = state[7];
    let r = (x * x + y * y + z * z + w * w).sqrt();
    let r_cubed = r.powi(3);
    let factor = -k / (m * r_cubed); // Mass affects acceleration
    let ax = factor * x;
    let ay = factor * y;
    let az = factor * z;
    let aw = factor * w;
    vector![vx, vy, vz, vw, ax, ay, az, aw]
}

pub fn rk4_step(state: &State, t: f64, dt: f64, k: f64, m: f64) -> State {
    let k1 = derivatives(state, t, k, m);
    let k2 = derivatives(&(state + 0.5 * dt * k1), t + 0.5 * dt, k, m);
    let k3 = derivatives(&(state + 0.5 * dt * k2), t + 0.5 * dt, k, m);
    let k4 = derivatives(&(state + dt * k3), t + dt, k, m);
    state + (dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
}

pub fn simulate_orbit(
    t_span: (f64, f64),
    n_steps: usize,
    initial_state: State,
    k: f64,
    m: f64,
) -> Vec<(f64, f64)> {
    let (t0, tf) = t_span;
    let dt = (tf - t0) / (n_steps as f64);
    let mut points = Vec::with_capacity(n_steps + 1);
    let mut state = initial_state;
    let mut t = t0;
    points.push((state[0], state[1]));
    for _ in 0..n_steps {
        state = rk4_step(&state, t, dt, k, m);
        t += dt;
        points.push((state[0], state[1]));
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4d_orbit_simulation() {
        let k = 1.0;
        let t_span = (0.0, 10.0);
        let n_steps = 1000;
        let nodes = get_orbit_nodes(4);

        let orbits: Vec<Vec<(f64, f64)>> = nodes
            .iter()
            .map(|node| {
                let initial_state = vector![
                    node.initial_position[0],
                    node.initial_position[1],
                    node.initial_position[2],
                    node.initial_position[3],
                    node.initial_velocity[0],
                    node.initial_velocity[1],
                    node.initial_velocity[2],
                    node.initial_velocity[3],
                ];
                simulate_orbit(t_span, n_steps, initial_state, k, node.mass)
            })
            .collect();

        let file = File::create("test_orbits_2d_projection.csv").unwrap();
        let mut wtr = Writer::from_writer(file);
        wtr.write_record(["node", "t", "x", "y"]).unwrap();
        let dt = (t_span.1 - t_span.0) / (n_steps as f64);
        for (i, orbit) in orbits.iter().enumerate() {
            for (j, &(x, y)) in orbit.iter().enumerate() {
                let t = t_span.0 + (j as f64) * dt;
                wtr.write_record([i.to_string(), t.to_string(), x.to_string(), y.to_string()])
                    .unwrap();
            }
        }
        wtr.flush().unwrap();

        assert_eq!(orbits.len(), 4, "Should have 4 orbits");
        assert!(
            orbits.iter().all(|o| o.len() == n_steps + 1),
            "Each orbit should have n_steps + 1 points"
        );
        assert!(
            orbits
                .iter()
                .flatten()
                .all(|&(x, y)| x.is_finite() && y.is_finite()),
            "Orbit points should be finite"
        );
    }
}
