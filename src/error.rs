/// Errors encountered during absorbing, finalizing or squeezing from TurboShake instances.
#[derive(PartialEq)]
pub enum TurboShakeError {
    /// Xof instance is still in the data absorption phase; `finalize()` must be called to start squeezing output.
    StillInDataAbsorptionPhase,
    /// Attempted to absorb more data or finalize after the data absorption phase was already finalized.
    DataAbsorptionPhaseAlreadyFinalized,
}

impl std::fmt::Display for TurboShakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TurboShakeError::StillInDataAbsorptionPhase => write!(f, "Must call `finalize` to start squeezing output"),
            TurboShakeError::DataAbsorptionPhaseAlreadyFinalized => write!(f, "Already finalized, only squeezing is possible now"),
        }
    }
}

impl std::fmt::Debug for TurboShakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for TurboShakeError {}
