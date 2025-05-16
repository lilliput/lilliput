use crate::config::PackingMode;

pub trait WithBeBytes {
    fn with_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T;
}

pub trait WithPackedBeBytes: WithBeBytes {
    fn with_native_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T;

    fn with_optimal_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T;

    #[inline]
    fn with_packed_be_bytes<T, F>(&self, packing_mode: PackingMode, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        match packing_mode {
            PackingMode::None => self.with_be_bytes(f),
            PackingMode::Native => self.with_native_packed_be_bytes(f),
            PackingMode::Optimal => self.with_optimal_packed_be_bytes(f),
        }
    }
}

pub trait WithValidatedPackedBeBytes: WithBeBytes {
    type Validator;

    fn with_validated_native_packed_be_bytes<T, F>(&self, validator: &Self::Validator, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T;

    fn with_validated_optimal_packed_be_bytes<T, F>(&self, validator: &Self::Validator, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T;

    #[inline]
    fn with_validated_packed_be_bytes<T, F>(
        &self,
        packing_mode: PackingMode,
        validator: &Self::Validator,
        f: F,
    ) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        match packing_mode {
            PackingMode::None => self.with_be_bytes(f),
            PackingMode::Native => self.with_validated_native_packed_be_bytes(validator, f),
            PackingMode::Optimal => self.with_validated_optimal_packed_be_bytes(validator, f),
        }
    }
}
