//! Mekanism-inspired energy and machine-cycle primitives for Rust Industries.
//!
//! This module is deliberately independent of Horizon's host API. It models
//! buffer limits and one machine operation tick; the block-entity adapter can
//! be added separately and tested against these deterministic rules.

/// A bounded energy buffer. Values are measured in FE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergyBuffer {
    stored: i32,
    capacity: i32,
    max_insert: i32,
    max_extract: i32,
}

impl EnergyBuffer {
    /// Create a buffer and clamp its initial amount and transfer limits.
    #[must_use]
    pub const fn new(
        stored: i32,
        capacity: i32,
        max_insert: i32,
        max_extract: i32,
    ) -> Self {
        let capacity = if capacity < 0 { 0 } else { capacity };
        let stored = if stored < 0 {
            0
        } else if stored > capacity {
            capacity
        } else {
            stored
        };
        Self {
            stored,
            capacity,
            max_insert: if max_insert < 0 { 0 } else { max_insert },
            max_extract: if max_extract < 0 { 0 } else { max_extract },
        }
    }

    #[must_use]
    pub const fn stored(&self) -> i32 {
        self.stored
    }

    #[must_use]
    pub const fn capacity(&self) -> i32 {
        self.capacity
    }

    #[must_use]
    pub const fn space(&self) -> i32 {
        self.capacity - self.stored
    }

    /// Insert at most the offered amount, free capacity and per-call limit.
    /// Returns the amount actually accepted.
    pub fn insert(&mut self, offered: i32) -> i32 {
        let accepted = self.space().min(offered).min(self.max_insert).max(0);
        self.stored += accepted;
        accepted
    }

    /// Extract at most the requested amount, stored energy and per-call limit.
    /// Returns the amount actually extracted.
    pub fn extract(&mut self, requested: i32) -> i32 {
        let extracted = self.stored.min(requested).min(self.max_extract).max(0);
        self.stored -= extracted;
        extracted
    }

    /// Generate energy internally, up to the remaining capacity. This is
    /// independent of external insertion limits because generators are sources,
    /// not capability clients.
    pub fn generate(&mut self, amount: i32) -> i32 {
        let accepted = self.space().min(amount).max(0);
        self.stored += accepted;
        accepted
    }

    /// Spend energy internally. Internal machine use is not limited by the
    /// external extraction rate; it only requires enough stored FE.
    pub fn consume(&mut self, amount: i32) -> bool {
        if amount < 0 || self.stored < amount {
            return false;
        }
        self.stored -= amount;
        true
    }
}

/// Energy and duration for one machine operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Operation {
    /// Number of successful powered ticks needed to complete the operation.
    pub ticks: i32,
    /// FE consumed on each powered tick.
    pub energy_per_tick: i32,
}

/// Persistent progress for one machine process.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MachineProgress {
    /// Completed powered ticks in the current operation.
    pub ticks_done: i32,
}

/// Why a machine did not advance during this tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReason {
    NoInputOrRecipe,
    OutputBlocked,
    NoEnergy,
    InvalidOperation,
}

/// Result of one machine tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TickResult {
    Working,
    Completed,
    Idle(IdleReason),
}

/// Advance one machine operation by one game tick.
///
/// Missing input/recipe resets progress. A blocked output or insufficient
/// energy pauses progress. Energy is consumed only when the machine advances.
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

    if !output_has_room {
        return TickResult::Idle(IdleReason::OutputBlocked);
    }

    if !buffer.consume(operation.energy_per_tick) {
        return TickResult::Idle(IdleReason::NoEnergy);
    }

    progress.ticks_done = progress.ticks_done.saturating_add(1);
    if progress.ticks_done >= operation.ticks {
        progress.ticks_done = 0;
        TickResult::Completed
    } else {
        TickResult::Working
    }
}

#[cfg(test)]
mod tests {
    use super::{advance, EnergyBuffer, IdleReason, MachineProgress, Operation, TickResult};

    #[test]
    fn buffer_clamps_initial_values_and_respects_insert_rate() {
        let mut buffer = EnergyBuffer::new(150, 100, 20, 10);
        assert_eq!(buffer.stored(), 100);
        assert_eq!(buffer.insert(50), 0);
        assert_eq!(buffer.extract(50), 10);
        assert_eq!(buffer.insert(50), 10);
        assert_eq!(buffer.stored(), 100);
    }

    #[test]
    fn buffer_rejects_negative_transfer_amounts() {
        let mut buffer = EnergyBuffer::new(50, 100, 30, 30);
        assert_eq!(buffer.insert(-5), 0);
        assert_eq!(buffer.extract(-5), 0);
        assert_eq!(buffer.stored(), 50);
    }

    #[test]
    fn generator_fills_only_available_capacity() {
        let mut buffer = EnergyBuffer::new(90, 100, 0, 20);
        assert_eq!(buffer.generate(50), 10);
        assert_eq!(buffer.stored(), 100);
        assert_eq!(buffer.generate(50), 0);
        assert_eq!(buffer.stored(), 100);
        assert_eq!(buffer.generate(-5), 0);
    }

    #[test]
    fn operation_consumes_energy_and_completes_on_exact_tick() {
        let mut buffer = EnergyBuffer::new(30, 100, 100, 100);
        let mut progress = MachineProgress::default();
        let op = Operation { ticks: 3, energy_per_tick: 10 };

        assert_eq!(advance(&mut buffer, &mut progress, op, true, true), TickResult::Working);
        assert_eq!(advance(&mut buffer, &mut progress, op, true, true), TickResult::Working);
        assert_eq!(advance(&mut buffer, &mut progress, op, true, true), TickResult::Completed);
        assert_eq!(buffer.stored(), 0);
        assert_eq!(progress.ticks_done, 0);
    }

    #[test]
    fn insufficient_energy_pauses_without_losing_progress() {
        let mut buffer = EnergyBuffer::new(10, 100, 100, 100);
        let mut progress = MachineProgress { ticks_done: 2 };
        let op = Operation { ticks: 5, energy_per_tick: 20 };

        assert_eq!(advance(&mut buffer, &mut progress, op, true, true), TickResult::Idle(IdleReason::NoEnergy));
        assert_eq!(progress.ticks_done, 2);
        assert_eq!(buffer.stored(), 10);
    }

    #[test]
    fn missing_recipe_resets_progress_but_blocked_output_pauses() {
        let mut buffer = EnergyBuffer::new(100, 100, 100, 100);
        let mut progress = MachineProgress { ticks_done: 2 };
        let op = Operation { ticks: 5, energy_per_tick: 10 };

        assert_eq!(advance(&mut buffer, &mut progress, op, true, false), TickResult::Idle(IdleReason::OutputBlocked));
        assert_eq!(progress.ticks_done, 2);
        assert_eq!(advance(&mut buffer, &mut progress, op, false, true), TickResult::Idle(IdleReason::NoInputOrRecipe));
        assert_eq!(progress.ticks_done, 0);
        assert_eq!(buffer.stored(), 100);
    }

    #[test]
    fn invalid_operation_does_not_consume_energy() {
        let mut buffer = EnergyBuffer::new(100, 100, 100, 100);
        let mut progress = MachineProgress { ticks_done: 3 };
        let op = Operation { ticks: 0, energy_per_tick: 10 };

        assert_eq!(advance(&mut buffer, &mut progress, op, true, true), TickResult::Idle(IdleReason::InvalidOperation));
        assert_eq!(progress.ticks_done, 0);
        assert_eq!(buffer.stored(), 100);
    }
}
