use crate::{Circle, Cpx, Generator};
use derivative::Derivative;
use nalgebra::Matrix2;
use ordered_float::NotNan;
use std::collections::BinaryHeap;

pub struct CircleQueue {
    queue: BinaryHeap<QueueItem>,
    gens: [Generator; 4],
}

impl CircleQueue {
    fn item(&self, matrix: Matrix2<Cpx>, last: u8) -> QueueItem {
        let ri = (matrix * self.gens[last as usize].circle).radius_inv();
        QueueItem {
            matrix,
            last,
            priority: NotNan::new(-ri).unwrap(),
        }
    }
    pub fn new(gens: [Generator; 4]) -> Self {
        let mut q = CircleQueue {
            queue: BinaryHeap::new(),
            gens,
        };
        for i in 0..4 {
            q.queue.push(q.item(Matrix2::identity(), i));
        }
        q
    }
    pub fn advance(&mut self) {
        let item = self.queue.pop().unwrap();
        let matrix = item.matrix * self.gens[item.last as usize].matrix;
        for i in 3..6 {
            self.queue.push(self.item(matrix, (item.last + i) % 4));
        }
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    pub fn circles(self) -> impl Iterator<Item = Circle> {
        let (queue, gens) = (self.queue, self.gens);
        queue
            .into_iter()
            .map(move |i| i.matrix * gens[i.last as usize].circle)
    }
}

#[derive(Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct QueueItem {
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    matrix: Matrix2<Cpx>,
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    last: u8,
    priority: NotNan<f64>,
}
