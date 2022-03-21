use crate::graph::Graph;
use crate::shapes::{Shapes, ANGLE_2PI};
use std::f64::consts::PI;

pub struct Builder {
    layers: i32,
    room_counts: Vec<i32>,
    room_count_ratio: Vec<i32>,
    first_room_in_layer: Vec<i32>,
}

impl Builder {
    pub fn new(layers: i32) -> Self {
        Builder {
            layers,
            room_counts: vec![],
            room_count_ratio: vec![],
            first_room_in_layer: vec![],
        }
    }

    pub fn build(&mut self) -> (Graph, Shapes) {
        let mut graph = Graph::new();

        self.compute_room_counts();

        let r_max = compute_radius(self.layers);
        let mut shapes = Shapes::new(true, 2 * r_max, 2 * r_max, 10);

        for _ in 0..self.layers {
            self.first_room_in_layer.push(-1);
        }

        self.generate_rooms(&mut graph, &mut shapes);
        self.generate_concentric_walls(&mut graph, &mut shapes);
        self.generate_radial_walls(&mut graph, &mut shapes);
        self.generate_outer_walls(&mut shapes);

        graph.start_room = 0;
        graph.target_room = graph.room_count() - 1;
        (graph, shapes)
    }

    fn compute_room_counts(&mut self) {
        if self.layers > 0 {
            self.room_counts.push(1);
            self.room_count_ratio.push(1);
            if self.layers > 1 {
                let room_count_in_zero_layer = 4;
                self.room_counts.push(room_count_in_zero_layer);
                self.room_count_ratio.push(room_count_in_zero_layer);

                // all layers except the central layer
                for i in 2..self.layers {
                    let cnt = self.room_counts[(i - 1) as usize];
                    let next_room_if_doubled = PI * (compute_radius(i - 1) / cnt) as f64;
                    let minimal_room_length = 15;
                    if next_room_if_doubled < minimal_room_length as f64 {
                        self.room_counts.push(cnt);
                        self.room_count_ratio.push(1);
                    } else {
                        self.room_counts.push(2 * cnt);
                        self.room_count_ratio.push(2);
                    }
                }
            }
        }
        assert!(self.room_counts.len() == self.layers as usize);
        assert!(self.room_count_ratio.len() == self.layers as usize);
    }

    fn generate_rooms(&mut self, graph: &mut Graph, shapes: &mut Shapes) {
        for r in 0..self.layers {
            self.generate_row_of_rooms(r, graph, shapes);
        }
    }

    fn generate_row_of_rooms(&mut self, layer: i32, graph: &mut Graph, shapes: &mut Shapes) {
        let cnt_max = self.room_cnt_in_outer_layer();
        let cnt_this = self.room_counts[layer as usize];
        let room_ratio = cnt_max / cnt_this;
        for phi in 0..cnt_this {
            let room = graph.add_room();
            if phi == 0 {
                self.first_room_in_layer[layer as usize] = room;
            }

            let mut y = 0;
            if layer > 0 {
                y = (compute_radius(layer) + compute_radius(layer - 1)) / 2;
            }

            shapes.add_floor(room, self.map_phi_d((2*phi * room_ratio + room_ratio) as f64 / 2f64 ), y)
           
        }
    }
    fn generate_concentric_walls(&mut self, graph: &mut Graph, shapes: &mut Shapes) {
        // draw concentric wall at radius r
        for layer in 0..self.layers - 1 {
            //  LOGGER.log(Level.FINE, "generateConcentricWalls r=" + layer);

            // the next layer may have less rooms than this one
            let room_cnt_inner = self.room_counts[layer as usize];
            let room_cnt_outer = self.room_counts[(layer + 1) as usize];
            let g_room_inner = self.first_room_in_layer[layer as usize];
            let g_room_outer = self.first_room_in_layer[(layer + 1) as usize];

            let room_cnt_ratio = self.room_count_ratio[(layer + 1) as usize];
            for room_inner in 0..room_cnt_inner {
                for j in 0..room_cnt_ratio {
                    let room_outer = room_inner * room_cnt_ratio + j;
                    let id = graph.add_wall(g_room_inner + room_inner, g_room_outer + room_outer);
                    let r = compute_radius(layer);
                    self.add_wall_shape(shapes, room_cnt_outer, r, r, room_outer, room_outer + 1, id);
                }
            }
        }
    }

    fn add_wall_shape(
        &self,
        shapes: &mut Shapes,
        room_cnt_this_layer: i32,
        r1: i32,
        r2: i32,
        phi1: i32,
        phi2: i32,
        id: i32,
    ) {
        let outer_cnt = self.room_cnt_in_outer_layer();
        let room_map_ratio = outer_cnt / room_cnt_this_layer;
        let r_phi1 = (phi1 * room_map_ratio) % outer_cnt;
        let r_phi2 = (phi2 * room_map_ratio) % outer_cnt;
        shapes.add_inner_wall(
            id,
            self.map_phi_d(r_phi1 as f64),
            r1,
            self.map_phi_d(r_phi2 as f64),
            r2,
            -1,
            -1,
        );
    }

    fn generate_radial_walls(&mut self, graph: &mut Graph, shapes: &mut Shapes) {
        for layer in 1..self.layers {
            //LOGGER.log(Level.FINE, "generateRadialWalls i=" + layer);

            let cnt = self.room_counts[layer as usize];
            if cnt <= 1 {
                continue;
            }

            let gr = self.first_room_in_layer[layer as usize];
            for j in 0..cnt {
                let id = graph.add_wall(gr + j, gr + (j + 1) % cnt);
                // strange wall naming convention - wall 0 is between room 0 and
                // 1
                let phi = (j + 1) % cnt;
                self.add_wall_shape(
                    shapes,
                    cnt,
                    compute_radius(layer - 1),
                    compute_radius(layer),
                    phi,
                    phi,
                    id,
                );
            }
        }
    }

    fn generate_outer_walls(&mut self, shapes: &mut Shapes) {
        let r = compute_radius(self.layers - 1);
        shapes.add_outer_wall(0, r, 0, r, -1, -1);
    }

    fn room_cnt_in_outer_layer(&self) -> i32 {
        self.room_counts[self.room_counts.len() - 1]
    }

    fn map_phi_d(&self, phi: f64) -> i32 {
        (phi * (ANGLE_2PI as f64) / self.room_cnt_in_outer_layer() as f64).floor() as i32
    }
}

fn compute_radius(i: i32) -> i32 {
    let zero_layer_radius = 200;
    let layer_size = 300;
    zero_layer_radius + i*layer_size
}
