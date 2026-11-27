//! Bounded FE storage and reusable powered-operation logic.

/// A bounded energy buffer. Values are measured in FE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergyBuffer {
    stored: i32,
    capacity: i32,
    max_insert: i32,
    max_extract: i32,
}

impl EnergyBuffer {
    #[must_use]
    pub const fn new(stored: i32, capacity: i32, max_insert: i32, max_extract: i32) -> Self {
        let capacity = if capacity < 0 { 0 } else { capacity };
        let stored = if stored < 0 { 0 } else if stored > capacity { capacity } else { stored };
        Self {
            stored,
            capacity,
            max_insert: if max_insert < 0 { 0 } else { max_insert },
            max_extract: if max_extract < 0 { 0 } else { max_extract },
        }
    }

    #[must_use]
    pub const fn stored(&self) -> i32 { self.stored }
    #[must_use]
    pub const fn capacity(&self) -> i32 { self.capacity }
    #[must_use]
    pub const fn space(&self) -> i32 { self.capacity - self.stored }

    pub fn insert(&mut self, offered: i32) -> i32 {
        let accepted = self.space().min(offered).min(self.max_insert).max(0);
        self.stored += accepted;
        accepted
    }

    pub fn extract(&mut self, requested: i32) -> i32 {
        let extracted = self.stored.min(requested).min(self.max_extract).max(0);
        self.stored -= extracted;
        extracted
    }

    pub fn generate(&mut self, amount: i32) -> i32 {
        let accepted = self.space().min(amount).max(0);
        self.stored += accepted;
        accepted
    }

    pub fn consume(&mut self, amount: i32) -> bool {
        if amount < 0 || self.stored < amount { return false; }
        self.stored -= amount;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Operation {
    pub ticks: i32,
    pub energy_per_tick: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MachineProgress { pub ticks_done: i32 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReason { NoInputOrRecipe, OutputBlocked, NoEnergy, InvalidOperation }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TickResult { Working, Completed, Idle(IdleReason) }

pub fn advance(
    buffer: &mut EnergyBuffer,
    progress: &mut MachineProgress,
    operation: Operation,
    has_input_and_recipe: bool,
    output_has_room: bool,
) -> TickResult {
    if operation.ticks <= 0 || operation.energy_per_tick < 0 {
        progress.ticks_done = 0;
        return TickResult::Idle(IdleReason::InvalidOperation);
    }
    if !has_input_and_recipe {
        progress.ticks_done = 0;
        return TickResult::Idle(IdleReason::NoInputOrRecipe);
    }
    if !output_has_room { return TickResult::Idle(IdleReason::OutputBlocked); }
    if !buffer.consume(operation.energy_per_tick) { return TickResult::Idle(IdleReason::NoEnergy); }
    progress.ticks_done = progress.ticks_done.saturating_add(1);
    if progress.ticks_done >= operation.ticks {
        progress.ticks_done = 0;
        TickResult::Completed
    } else { TickResult::Working }
}

#[cfg(test)]
mod tests {
    use super::{advance, EnergyBuffer, IdleReason, MachineProgress, Operation, TickResult};

    #[test]
    fn bounded_buffer_clamps_and_respects_limits() {
        let mut b = EnergyBuffer::new(150, 100, 20, 10);
        assert_eq!(b.stored(), 100);
        assert_eq!(b.insert(50), 0);
        assert_eq!(b.extract(50), 10);
        assert_eq!(b.insert(50), 10);
        assert_eq!(b.stored(), 100);
    }

    #[test]
    fn invalid_transfer_amounts_are_ignored() {
        let mut b = EnergyBuffer::new(50, 100, 30, 30);
        assert_eq!(b.insert(-5), 0);
        assert_eq!(b.extract(-5), 0);
        assert_eq!(b.stored(), 50);
    }

    #[test]
    fn generation_never_overfills() {
        let mut b = EnergyBuffer::new(90, 100, 0, 20);
        assert_eq!(b.generate(50), 10);
        assert_eq!(b.stored(), 100);
        assert_eq!(b.generate(50), 0);
    }

    #[test]
    fn operation_uses_energy_and_completes_exactly() {
        let mut b = EnergyBuffer::new(30, 100, 100, 100);
        let mut p = MachineProgress::default();
        let op = Operation { ticks: 3, energy_per_tick: 10 };
        assert_eq!(advance(&mut b, &mut p, op, true, true), TickResult::Working);
        assert_eq!(advance(&mut b, &mut p, op, true, true), TickResult::Working);
        assert_eq!(advance(&mut b, &mut p, op, true, true), TickResult::Completed);
        assert_eq!(b.stored(), 0);
        assert_eq!(p.ticks_done, 0);
    }

    #[test]
    fn insufficient_energy_pauses_without_losing_progress() {
        let mut b = EnergyBuffer::new(10, 100, 100, 100);
        let mut p = MachineProgress { ticks_done: 2 };
        let op = Operation { ticks: 5, energy_per_tick: 20 };
        assert_eq!(advance(&mut b, &mut p, op, true, true), TickResult::Idle(IdleReason::NoEnergy));
        assert_eq!(p.ticks_done, 2);
        assert_eq!(b.stored(), 10);
    }

    #[test]
    fn missing_recipe_resets_but_blocked_output_pauses() {
        let mut b = EnergyBuffer::new(100, 100, 100, 100);
        let mut p = MachineProgress { ticks_done: 2 };
        let op = Operation { ticks: 5, energy_per_tick: 10 };
        assert_eq!(advance(&mut b, &mut p, op, true, false), TickResult::Idle(IdleReason::OutputBlocked));
        assert_eq!(p.ticks_done, 2);
        assert_eq!(advance(&mut b, &mut p, op, false, true), TickResult::Idle(IdleReason::NoInputOrRecipe));
        assert_eq!(p.ticks_done, 0);
        assert_eq!(b.stored(), 100);
    }
}
