
use std::f32::consts::PI;

use itertools::Itertools;

use miniquad::date;

use macroquad::prelude::*;
use macroquad::rand::RandGenerator;

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
        let num_nodes = 100;
        let mut nodes = Vec::with_capacity(num_nodes);

        for i in 0..num_nodes {
            let k = (i as f32) / (num_nodes as f32);
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
        let mut edges = Vec::with_capacity(num_nodes);

        for i in 0..num_nodes {
            let node_index1 = i;
            let node_index2 = (i + 1) % num_nodes;

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

    fn calculate_node_velocities(&mut self) {
        for (node_index1, node_index2) in (0..self.nodes.len()).tuple_combinations() {
            let [node1, node2] = self.nodes.get_disjoint_mut([node_index1, node_index2]).expect(
                "node_index1 should not equal to node_index2"
            );

            let diff_x = node1.x - node2.x;
            let diff_y = node1.y - node2.y;
            let distance = (diff_x * diff_x + diff_y * diff_y).sqrt();

            if distance > 100.0 {
                continue;
            } else if distance == 0.0 {
                continue;
            }

            let force = 1.0 * (distance / 10.0).powf(-2.0);
            let fx = force * diff_x / distance;
            let fy = force * diff_y / distance;

            node1.dx += fx;
            node1.dy += fy;

            node2.dx -= fx;
            node2.dy -= fy;
        }
    }

    fn calculate_edge_velocities(&mut self) {
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

            let force = (distance - edge.length) * 5.0;
            let fx = force * diff_x / distance;
            let fy = force * diff_y / distance;

            node1.dx += fx;
            node1.dy += fy;

            node2.dx -= fx;
            node2.dy -= fy;
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
        let dt = get_frame_time();

        // Process mouse.
        self.process_mouse();

        // Span nodes.
        while (self.edges.len() - 100) <= (t / 0.1) as usize {
            self.swapn_node();
        }

        // Update node velocities.
        self.calculate_node_velocities();

        // Calculate edge velocities.
        self.calculate_edge_velocities();

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
