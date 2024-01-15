use crate::data::sample;
use crate::{Error, Result};

pub struct Take {
    pub id: usize,
    pub closed: bool,
    pub samples: Vec<sample::Sample>,
}

pub struct TakeManager {
    active_take: Option<usize>,
    takes: Vec<Take>,
}

impl TakeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            active_take: None,
            takes: Vec::new(),
        };
        manager.new_take();
        manager
    }

    pub fn active_take(&self) -> Option<usize> {
        self.active_take
    }

    pub fn new_take(&mut self) -> usize {
        let take = Take {
            id: self.takes.len() + 1,
            closed: false,
            samples: Vec::new(),
        };
        self.takes.push(take);
        self.active_take = Some(self.takes.len());
        self.active_take.unwrap()
    }

    pub fn update(&mut self, sample: sample::Sample) -> Result<()> {
        if let Some(take) = self.takes.last_mut() {
            if take.closed {
                return Err(Error::TakeClosed);
            }
            take.samples.push(sample);
        }
        Ok(())
    }
}
