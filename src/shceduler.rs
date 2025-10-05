use crate::event::{Event, EventType};
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Scheduler {
    pub queue: BinaryHeap<Event>,
    pub current_time: f64,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            current_time: 0.0,
        }
    }
    pub fn add_event(&mut self, event: Event) {
        self.queue.push(event)
    }
    pub fn next_event(&mut self) -> Option<Event> {
        let next = self.queue.pop();
        if let Some(ev) = next {
            self.current_time = ev.time;
            Some(ev)
        } else {
            None
        }
    }
    pub fn peek_time(&self) -> f64 {
        self.queue.peek().map(|e| e.time).unwrap_or(f64::INFINITY)
    }
    pub fn queue_length(&self) -> usize {
        self.queue.len()
    }
    pub fn has_next(&self) -> bool {
        !self.queue.is_empty()
    }
    pub fn current_time(&self) -> f64 {
        self.current_time
    }
}

#[test]
fn test_add_and_next_event() {
    let mut sched = Scheduler::new();

    let e1 = Event::new(5.0, EventType::Arrival, 1);
    let e2 = Event::new(3.0, EventType::Arrival, 2);
    let e3 = Event::new(4.0, EventType::Arrival, 3);

    sched.add_event(e1);
    sched.add_event(e2);
    sched.add_event(e3);

    assert_eq!(sched.queue_length(), 3);
    assert_eq!(sched.peek_time(), 3.0);

    // let ev = sched.next_event().unwrap();
    // assert_eq!(ev.customer_id, 2);
    // assert_eq!(sched.current_time(), 3.0);

    // let ev = sched.next_event().unwrap();
    // assert_eq!(ev.customer_id, 3);
    // assert_eq!(sched.current_time(), 4.0);

    // let ev = sched.next_event().unwrap();
    // assert_eq!(ev.customer_id, 1);
    // assert_eq!(sched.current_time(), 5.0);

    // assert!(!sched.has_next());
}

#[test]
fn test_empty_scheduler() {
    let sched = Scheduler::new();
    assert!(!sched.has_next());
    assert_eq!(sched.peek_time(), f64::INFINITY);
}
