use crate::event::BattleEvent;

/// A simple FIFO queue for battle events
#[derive(Debug, Default)]
pub struct EventQueue {
    events: Vec<BattleEvent>,
}

impl EventQueue {
    /// Create a new empty queue
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Add a new event to the queue
    pub fn push(&mut self, event: BattleEvent) {
        self.events.push(event);
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Get the number of events in the queue
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Pop the first event (FIFO)
    pub fn pop(&mut self) -> Option<BattleEvent> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0))
        }
    }

    /// Drain all events from the queue
    pub fn drain(&mut self) -> Vec<BattleEvent> {
        self.events.drain(..).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creature::CreatureId;
    use crate::event::BattleEvent;

    #[test]
    fn push_and_pop_event() {
        let mut queue = EventQueue::new();
        let source = CreatureId::new();
        let target = CreatureId::new();

        queue.push(BattleEvent::Damage {
            source,
            target,
            amount: 15,
        });

        assert_eq!(queue.len(), 1);

        let event = queue.pop().unwrap();
        match event {
            BattleEvent::Damage { amount, .. } => assert_eq!(amount, 15),
            _ => panic!("Expected Damage event"),
        }

        assert!(queue.is_empty());
    }

    #[test]
    fn drain_events() {
        let mut queue = EventQueue::new();
        queue.push(BattleEvent::Custom {
            description: "First".to_string(),
        });
        queue.push(BattleEvent::Custom {
            description: "Second".to_string(),
        });

        let drained = queue.drain();
        assert_eq!(drained.len(), 2);
        assert!(queue.is_empty());
    }
}

