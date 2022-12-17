/***
Copyright (c) 2022 Juan Medina

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
***/

use crate::valve::Valve;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cave {
    pub valves: Vec<Valve>,
    pub distances: Vec<Vec<u32>>,
}

impl Cave {
    fn new(valves: Vec<Valve>) -> Self {
        let last_index = Valve::index_from_name("ZZ");
        let distances: Vec<Vec<u32>> = vec![vec![0; last_index + 1]; last_index + 1];
        Self { valves, distances }
    }

    pub fn max_pressure(&mut self, with_elephant: bool) -> u32 {
        self.calculate_distances();
        let valves = self.non_zero_valves();
        let start = Valve::index_from_name("AA");

        const TIME: u32 = 30;
        if !with_elephant {
            self.best_path(valves, start, TIME)
        } else {
            const TRAINING: u32 = 4;
            const TIME_PAIRS: u32 = TIME - TRAINING;
            self.best_path_in_pair(valves, start, TIME_PAIRS, start, TIME_PAIRS)
        }
    }

    fn best_path(&self, valves: Vec<&Valve>, at_valve: usize, minutes_left: u32) -> u32 {
        let mut pressures: Vec<u32> = vec![];

        for valve in &valves {
            let distance = self.distances[at_valve][valve.index];

            if distance >= minutes_left {
                continue;
            }

            let pressure = valve.rate * (minutes_left - distance - 1);
            let remaining = self.filter_valve(&valves, valve.index);
            let new_pressure = self.best_path(remaining, valve.index, minutes_left - distance - 1);
            pressures.push(new_pressure + pressure);
        }

        *pressures.iter().max().unwrap_or(&0)
    }

    fn best_path_in_pair(
        &self,
        valves: Vec<&Valve>,
        pair1_at_valve: usize,
        pair1_minutes: u32,
        pair2_at_valve: usize,
        pair2_minutes: u32,
    ) -> u32 {
        let mut pressures: Vec<u32> = vec![];

        'pair_1_loop: for pair1_valve in &valves {
            let pair1_distance = self.distances[pair1_at_valve][pair1_valve.index];
            if pair1_distance >= pair1_minutes {
                continue 'pair_1_loop;
            }
            let pair1_pressure = pair1_valve.rate * (pair1_minutes - pair1_distance - 1);
            let remaining_1: Vec<&Valve> = self.filter_valve(&valves, pair1_valve.index);

            'pair_2_loop: for pair2_valve in &remaining_1 {
                let pair2_distance = self.distances[pair2_at_valve][pair2_valve.index];
                if pair2_distance >= pair2_minutes {
                    continue 'pair_2_loop;
                }

                let pair2_pressure = pair2_valve.rate * (pair2_minutes - pair2_distance - 1);
                let remaining_2: Vec<&Valve> = self.filter_valve(&remaining_1, pair2_valve.index);

                let new_pressure = self.best_path_in_pair(
                    remaining_2,
                    pair1_valve.index,
                    pair1_minutes - pair1_distance - 1,
                    pair2_valve.index,
                    pair2_minutes - pair2_distance - 1,
                );

                pressures.push(new_pressure + pair1_pressure + pair2_pressure);
            }
        }

        *pressures.iter().max().unwrap_or(&0)
    }

    fn filter_valve<'a>(&self, valves: &Vec<&'a Valve>, index: usize) -> Vec<&'a Valve> {
        valves
            .iter()
            .filter(|v| v.index != index)
            .cloned()
            .collect()
    }

    fn non_zero_valves(&self) -> Vec<&Valve> {
        self.valves.iter().filter(|v| v.rate > 0).collect()
    }

    fn calculate_distances(&mut self) {
        let last_index = Valve::index_from_name("ZZ");

        let mut edges: Vec<Vec<usize>> = vec![vec![]; last_index + 1];
        let vertices: Vec<usize> = self.valves.iter().map(|valve| valve.index).collect();

        for valve in &self.valves {
            for tunnel in &valve.tunnels_indexes {
                edges[valve.index].push(*tunnel);
            }
        }

        for valve in &self.valves {
            self.distances[valve.index] = self.dijkstra(&edges, &vertices, last_index, valve.index);
        }
    }

    fn dijkstra(
        &self,
        edges: &[Vec<usize>],
        vertices: &[usize],
        last_index: usize,
        start: usize,
    ) -> Vec<u32> {
        // initialize grid of "infinite" distances
        let mut distance_to: Vec<u32> = vec![u32::MAX - 1; last_index + 1];

        // queue up every coordinate
        let mut queue: HashSet<usize> = vertices.iter().cloned().collect();

        // set the first known distance: 0 from the start to the start
        distance_to[start] = 0;

        while !queue.is_empty() {
            // find the position in the queue with shortest distance from the starting valve
            let u = *queue
                .iter()
                .min_by(|&&a, &&b| distance_to[a].cmp(&distance_to[b]))
                .unwrap();

            queue.remove(&u);

            // get all valves adjacent to the starting one that are still in the queue
            let neighbours: Vec<usize> = edges[u]
                .iter()
                .filter(|valve| queue.contains(valve))
                .cloned()
                .collect();

            for v in neighbours {
                // a step to a neighbouring valve is always a distance of 1 (otherwise
                // we would've had to pass in edge weights to this function as well)
                let alt = distance_to[u] + 1;

                if alt < distance_to[v] {
                    distance_to[v] = alt;
                }
            }
        }

        distance_to
    }
}

pub fn new(valves: Vec<Valve>) -> Cave {
    Cave::new(valves)
}
