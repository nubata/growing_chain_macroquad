
use std::f32::consts::PI;

use miniquad::date;

use macroquad::prelude::*;
use macroquad::rand::RandGenerator;

const MIN_N_NODES: usize = 30;

const MAX_N_NODES: usize = 1000;

struct Node {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    r: f32,
    intensity: f32
}

struct Edge {
    node_index1: usize,
    node_index2: usize,
    length: f32
}

pub struct Model {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    gripped_node_index: Option<usize>,
    rand_generator: RandGenerator
}

impl Model {

    pub fn new() -> Model {
        // Get screen size.
        let screen_width = screen_width();
        let screen_height = screen_height();

        // Initialize nodes.
        let mut nodes = Vec::with_capacity(MIN_N_NODES);

        for i in 0..MIN_N_NODES {
            let k = (i as f32) / (MIN_N_NODES as f32);
            let theta = PI * 2.0 * k;

            let x = screen_width / 2.0 + theta.cos() * 100.0;
            let y = screen_height / 2.0 + theta.sin() * 100.0;

            let r = 5.0;
            let intensity = 0.0;

            let node = Node {
                x: x,
                y: y,
                dx: 0.0,
                dy: 0.0,
                r: r,
                intensity: intensity
            };

            nodes.push(node);
        }

        // Initialize edges.
        let mut edges = Vec::with_capacity(MIN_N_NODES);

        for i in 0..MIN_N_NODES {
            let node_index1 = i;
            let node_index2 = (i + 1) % MIN_N_NODES;

            let length = 20.0;

            let edge = Edge {
                node_index1: node_index1,
                node_index2: node_index2,
                length: length
            };

            edges.push(edge);
        }

        // Initialize random generator.
        let rand_generator = RandGenerator::new();
        rand_generator.srand(date::now() as u64);

        // Initialize model.
        Model {
            nodes: nodes,
            edges: edges,
            gripped_node_index: None,
            rand_generator: rand_generator
        }
    }

    fn process_mouse(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            if self.gripped_node_index.is_none() {

                // Try to grip a node.
                let mut gripped_node_index = None;
                let mut min_distance = 0.0;

                for (node_index, node) in self.nodes.iter().enumerate() {
                    let x = node.x;
                    let y = node.y;
                    let r = node.r;

                    let diff_x = x - mouse_x;
                    let diff_y = y - mouse_y;
                    let distance = (diff_x * diff_x + diff_y * diff_y).sqrt();

                    if distance <= r * 5.0 {
                        if gripped_node_index.is_none() || distance < min_distance {
                            gripped_node_index = Some(node_index);
                            min_distance = distance;
                        }
                    }
                }

                self.gripped_node_index = gripped_node_index;
            }

        } else {

            // Release the gripped node.
            self.gripped_node_index = None;

        }
    }

    fn swapn_node(&mut self) {
        let i = self.rand_generator.gen_range(0, self.edges.len() - 1);
        let edge1 = &self.edges[i];

        let node_index1 = edge1.node_index1;
        let node_index2 = edge1.node_index2;

        let node1 = &self.nodes[node_index1];
        let node2 = &self.nodes[node_index2];

        let node_index3 = self.nodes.len();

        let node3 = Node {
            x: (node1.x + node2.x) / 2.0,
            y: (node1.y + node2.y) / 2.0,
            dx: 0.0,
            dy: 0.0,
            r: (node1.r + node2.r) / 2.0,
            intensity: 1.0
        };

        self.nodes.push(node3);

        let edge2 = Edge {
            node_index1: node_index1,
            node_index2: node_index3,
            length: edge1.length
        };

        let edge3 = Edge {
            node_index1: node_index3,
            node_index2: node_index2,
            length: edge1.length
        };

        self.edges[i] = edge2;
        self.edges.push(edge3);
    }

    fn apply_damper_forces(&mut self, dt: f32) {
        for node in &mut self.nodes {
            let k = 0.1_f32.powf(dt);
            node.dx *= k;
            node.dy *= k;
        }
    }

    fn apply_repulsive_forces(&mut self, dt: f32) {
        for edge in &self.edges {
            let node_index1 = edge.node_index1;
            let node_index2 = edge.node_index2;

            for node_index3 in 0..self.nodes.len() {
                if node_index1 != node_index3 && node_index2 != node_index3 {
                    // Fetch the nodes.
                    let [node1, node2, node3] = self.nodes.get_disjoint_mut(
                        [node_index1, node_index2, node_index3]
                    ).unwrap();

                    // Calculate repulsive force.
                    let ax = node3.x - node1.x;
                    let ay = node3.y - node1.y;

                    let bx = node3.x - node2.x;
                    let by = node3.y - node2.y;

                    let cx = node2.x - node1.x;
                    let cy = node2.y - node1.y;

                    let a = (ax * ax + ay * ay).sqrt();
                    let b = (bx * bx + by * by).sqrt();
                    let c = (cx * cx + cy * cy).sqrt();

                    let k = ax * cx + ay * cy;

                    let d;

                    let theta = self.rand_generator.gen_range(0.0, 2.0 * PI);
                    let mut vx = theta.cos();
                    let mut vy = theta.sin();

                    if k <= 0.0 {

                        d = a;

                        if a > 0.0 {
                            vx = ax / a;
                            vy = ay / a;
                        }

                    } else if k < 1.0 {

                        d = (ax * cy - ay * cx) / (a * c);

                        let ex = node1.y - node2.y;
                        let ey = node2.x - node1.x;
                        let e = (ex * ex + ey * ey).sqrt();

                        if e > 0.0 {
                            vx = ex / e;
                            vy = ey / e;
                        }

                    } else {

                        d = b;

                        if b > 0.0 {
                            vx = bx / b;
                            vy = by / b;
                        }

                    }

                    let f = (d.max(1e-10) / 100.0).powi(-2).min(20.0);

                    let fx = vx * f;
                    let fy = vy * f;

                    node3.dx += fx * dt;
                    node3.dy += fy * dt;

                }
            }
        }
    }

    fn apply_elastic_forces(&mut self, dt: f32) {
        for edge in &self.edges {
            let node_index1 = edge.node_index1;
            let node_index2 = edge.node_index2;

            let [node1, node2] = self.nodes.get_disjoint_mut([node_index1, node_index2]).expect(
                "node_index1 should not be equal to node_index2"
            );

            let diff_x = node2.x - node1.x;
            let diff_y = node2.y - node1.y;
            let distance = (diff_x * diff_x + diff_y * diff_y).sqrt();

            if distance == 0.0 {
                continue;
            }

            let force = (distance - edge.length) * 50.0;
            let fx = force * diff_x / distance;
            let fy = force * diff_y / distance;

            node1.dx += fx * dt;
            node1.dy += fy * dt;

            node2.dx -= fx * dt;
            node2.dy -= fy * dt;
        }
    }

    fn update_node_positions(&mut self, dt: f32) {
        // Detect gripped node.
        let _gripped_node_index = match self.gripped_node_index {
            Some(node_index) => node_index,
            None => self.nodes.len()
        };

        // Update node positions.
        let (mouse_x, mouse_y) = mouse_position();

        let screen_width = screen_width();
        let screen_height = screen_height();

        for (node_index, node) in &mut self.nodes.iter_mut().enumerate() {
            if node_index == _gripped_node_index {
                node.x = mouse_x;
                node.y = mouse_y;
                node.dx = 0.0;
                node.dy = 0.0;
            } else {
                let v = (node.dx * node.dx + node.dy * node.dy).sqrt();
                let max_v = 500.0;
                if v > max_v {
                    let k = max_v / v;
                    node.dx *= k;
                    node.dy *= k;
                }

                node.x += node.dx * dt;
                node.y += node.dy * dt;

                node.dx *= (0.2_f32).powf(dt);
                node.dy *= (0.2_f32).powf(dt);

                if node.x > screen_width {
                    node.x = screen_width;
                } else if node.x < 0.0 {
                    node.x = 0.0;
                }
                if node.y > screen_height {
                    node.y = screen_height;
                } else if node.y < 0.0 {
                    node.y = 0.0;
                }

                node.intensity *= (0.5_f32).powf(dt);
            }
        }
    }

    pub fn update(&mut self) {
        // Calcuate time.
        let t = get_time();
        let dt = get_frame_time().min(1.0);

        // Process mouse.
        self.process_mouse();

        // Span nodes.
        while self.nodes.len() < MAX_N_NODES && (self.nodes.len() - MIN_N_NODES) <= (t / 0.1) as usize {
            self.swapn_node();
        }

        // Apply damper forces.
        self.apply_damper_forces(dt);

        // Apply repulsive forces.
        self.apply_repulsive_forces(dt);

        // Apply elastic forces.
        self.apply_elastic_forces(dt);

        // Update node positions.
        self.update_node_positions(dt);
    }

    pub fn draw(&self) {
        // Clear background.
        clear_background(BLACK);

        // Draw edges.
        for edge in &self.edges {
            let node1 = &self.nodes[edge.node_index1];
            let node2 = &self.nodes[edge.node_index2];

            draw_line(node1.x, node1.y, node2.x, node2.y, 3.0, WHITE);
        }

        // Draw nodes.
        for (node_index, node) in self.nodes.iter().enumerate() {
            let mut r = node.r;

            let mut col = Color {
                r: 1.0 - node.intensity,
                g: 1.0,
                b: 1.0 - node.intensity,
                a: 1.0
            };

            match self.gripped_node_index {
                Some(_node_index) => {
                    if node_index == _node_index {
                        r = node.r * 2.0;

                        col = Color {
                            r: 1.0,
                            g: 1.0,
                            b: 0.0,
                            a: 1.0
                        };
                    }
                }
                None => {}
            }

            draw_circle(node.x, node.y, r, BLACK);
            draw_circle_lines(node.x, node.y, r, 3.0, col);
        }
    }

}
