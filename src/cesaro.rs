// Copyright (c) 2015 William (B.J.) Snow Orvis
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Computations and abstractions needed for rendering a Césaro fractal

use common::{Point, Turtle};
use lindenmayer::{LindenmayerSystem, LindenmayerSystemDrawingParameters};

#[derive(Copy, Clone, Debug)]
pub struct CesaroFractal {
    iterations: u64,
}

#[derive(Copy, Clone, Debug)]
pub enum LSA {
    F, // move forward
    Q, // corner of the square
    L, // turn left X degrees
    R, // turn right X degrees
}

impl CesaroFractal {
    pub fn new(iterations: u64) -> Result<CesaroFractal, &'static str> {
        let lcc = CesaroFractal { iterations: iterations };
        Ok(lcc)
    }

    fn distance_forward(self) -> f64 {
        1.0 / (2.2 as f64).powf((self.iterations) as f64)
    }
}

impl LindenmayerSystem<LSA> for CesaroFractal {
    fn initial() -> Vec<LSA> {
        vec![LSA::F, LSA::Q, LSA::F, LSA::Q, LSA::F, LSA::Q, LSA::F, LSA::Q]
    }

    fn apply_rule(lstr: LSA) -> Vec<LSA> {
        match lstr {
            LSA::F => vec![LSA::F, LSA::L, LSA::F, LSA::R, LSA::R, LSA::F, LSA::L, LSA::F],
            x => vec![x],
        }
    }
}

impl LindenmayerSystemDrawingParameters<LSA> for CesaroFractal {
    fn iteration(&self) -> u64 {
        self.iterations
    }

    fn initialize_turtle(&self, turtle: &mut Turtle) {
        turtle.set_pos(Point { x: 0.0, y: -0.5 });
        turtle.set_rad(0.0);
    }

    fn interpret_symbol(&self, symbol: LSA, turtle: &mut Turtle) {
        match symbol {
            LSA::F => turtle.forward(self.distance_forward()),
            LSA::Q => turtle.turn_deg(90.0),
            LSA::L => turtle.turn_deg(85.0),
            LSA::R => turtle.turn_deg(-85.0),
        }
    }
}

#[cfg(test)]
mod test {
}