use crate::graph::Graph;
use crate::shapes::{Shapes, ANGLE_2PI};
use std::f64::consts::PI;

pub struct Builder {
    layers: i32,
    roomCounts: Vec<i32>,
    roomCountRatio: Vec<i32>,
    firstRoomInLayer: Vec<i32>,
}

impl Builder {
    pub fn new(layers: i32) -> Self {
        Builder {
            layers,
            roomCounts: vec![],
            roomCountRatio: vec![],
            firstRoomInLayer: vec![],
        }
    }

    pub fn build(&mut self) -> (Graph, Shapes) {
        let mut graph = Graph::new();

        let height = self.layers;
        let width = 2 * self.layers;

        self.computeRoomCounts();

        let rMax = computeRadius(self.layers);
        let mut shapes = Shapes::new(true, 2 * rMax, 2 * rMax, 10);

        for i in 0..self.layers {
            self.firstRoomInLayer.push(-1);
        }

        self.generateRooms(&mut graph, &mut shapes);
        self.generateConcentricWalls(&mut graph, &mut shapes);
        self.generateRadialWalls(&mut graph, &mut shapes);
        self.generateOuterWalls(&mut graph, &mut shapes);

        graph.start_room = 0;
        graph.target_room = graph.room_count() - 1;
        (graph, shapes)
    }

    fn computeRoomCounts(&mut self) {
        if self.layers > 0 {
            self.roomCounts.push(1);
            self.roomCountRatio.push(1);
            if self.layers > 1 {
                let roomCountInZeroLayer = 4;
                self.roomCounts.push(roomCountInZeroLayer);
                self.roomCountRatio.push(roomCountInZeroLayer);

                // all layers except the central layer
                for i in 2..self.layers {
                    let cnt = self.roomCounts[(i - 1) as usize];
                    let nextRoomIfDoubled = PI * (computeRadius(i - 1) / cnt) as f64;
                    let minimalRoomLength = 15;
                    if nextRoomIfDoubled < minimalRoomLength as f64 {
                        self.roomCounts.push(cnt);
                        self.roomCountRatio.push(1);
                    } else {
                        self.roomCounts.push(2 * cnt);
                        self.roomCountRatio.push(2);
                    }
                }
            }
        }
        assert!(self.roomCounts.len() == self.layers as usize);
        assert!(self.roomCountRatio.len() == self.layers as usize);
    }

    fn generateRooms(&mut self, graph: &mut Graph, shapes: &mut Shapes) {
        for r in 0..self.layers {
            self.generateRowOfRooms(r, graph, shapes);
        }
    }

    fn generateRowOfRooms(&mut self, layer: i32, graph: &mut Graph, shapes: &mut Shapes) {
        let cntMax = self.roomCntInOuterLayer();
        let cntThis = self.roomCounts[layer as usize];
        let roomRatio = cntMax / cntThis;
        for phi in 0..cntThis {
            let room = graph.add_room();
            if phi == 0 {
                self.firstRoomInLayer[layer as usize] = room;
            }

            let mut y = 0;
            if layer > 0 {
                y = (computeRadius(layer) + computeRadius(layer - 1)) / 2;
            }

            shapes.add_floor(room, self.mapPhiD((2*phi * roomRatio + roomRatio) as f64 / 2f64 ), y)
           
        }
    }
    fn generateConcentricWalls(&mut self, graph: &mut Graph, shapes: &mut Shapes) {
        // draw concentric wall at radius r
        for layer in 0..self.layers - 1 {
            //  LOGGER.log(Level.FINE, "generateConcentricWalls r=" + layer);

            // the next layer may have less rooms than this one
            let roomCntInner = self.roomCounts[layer as usize];
            let roomCntOuter = self.roomCounts[(layer + 1) as usize];
            let gRoomInner = self.firstRoomInLayer[layer as usize];
            let gRoomOuter = self.firstRoomInLayer[(layer + 1) as usize];

            let roomCntRatio = self.roomCountRatio[(layer + 1) as usize];
            for roomInner in 0..roomCntInner {
                for j in 0..roomCntRatio {
                    let roomOuter = roomInner * roomCntRatio + j;
                    let id = graph.add_wall(gRoomInner + roomInner, gRoomOuter + roomOuter);
                    let r = computeRadius(layer);
                    self.addWallShape(shapes, roomCntOuter, r, r, roomOuter, roomOuter + 1, id);
                }
            }
        }
    }

    fn addWallShape(
        &self,
        shapes: &mut Shapes,
        roomCntThisLayer: i32,
        r1: i32,
        r2: i32,
        phi1: i32,
        phi2: i32,
        id: i32,
    ) {
        let outerCnt = self.roomCntInOuterLayer();
        let roomMapRatio = outerCnt / roomCntThisLayer;
        let rPhi1 = (phi1 * roomMapRatio) % outerCnt;
        let rPhi2 = (phi2 * roomMapRatio) % outerCnt;
        shapes.add_inner_wall(
            id,
            self.mapPhiD(rPhi1 as f64),
            r1,
            self.mapPhiD(rPhi2 as f64),
            r2,
            -1,
            -1,
        );
    }

    fn generateRadialWalls(&mut self, graph: &mut Graph, shapes: &mut Shapes) {
        for layer in 1..self.layers {
            //LOGGER.log(Level.FINE, "generateRadialWalls i=" + layer);

            let cnt = self.roomCounts[layer as usize];
            if cnt <= 1 {
                continue;
            }

            let gr = self.firstRoomInLayer[layer as usize];
            for j in 0..cnt {
                let id = graph.add_wall(gr + j, gr + (j + 1) % cnt);
                // strange wall naming convention - wall 0 is between room 0 and
                // 1
                let phi = (j + 1) % cnt;
                self.addWallShape(
                    shapes,
                    cnt,
                    computeRadius(layer - 1),
                    computeRadius(layer),
                    phi,
                    phi,
                    id,
                );
            }
        }
    }

    fn generateOuterWalls(&mut self, graph: &mut Graph, shapes: &mut Shapes) {
        let r = computeRadius(self.layers - 1);
        shapes.add_outer_wall(0, r, 0, r, -1, -1);
    }

    fn roomCntInOuterLayer(&self) -> i32 {
        self.roomCounts[self.roomCounts.len() - 1]
    }

    fn mapPhiD(&self, phi: f64) -> i32 {
        (phi * (ANGLE_2PI as f64) / self.roomCntInOuterLayer() as f64).floor() as i32
    }
}

fn computeRadius(i: i32) -> i32 {
    let zeroLayerRadius = 200;
    let layerSize = 300;
    zeroLayerRadius + i * layerSize
}
