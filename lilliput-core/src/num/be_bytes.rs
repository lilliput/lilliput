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

pub trait WithPackedBeBytesIf: WithBeBytes {
    fn with_native_packed_be_bytes_if<T, P, F>(&self, predicate: P, f: F) -> T
    where
        P: Fn(&Self, &Self) -> bool,
        F: FnOnce(&[u8]) -> T;

    fn with_optimal_packed_be_bytes_if<T, P, F>(&self, predicate: P, f: F) -> T
    where
        P: Fn(&Self, &Self) -> bool,
        F: FnOnce(&[u8]) -> T;

    #[inline]
    fn with_packed_be_bytes_if<T, P, F>(&self, packing_mode: PackingMode, predicate: P, f: F) -> T
    where
        P: Fn(&Self, &Self) -> bool,
        F: FnOnce(&[u8]) -> T,
    {
        match packing_mode {
            PackingMode::None => self.with_be_bytes(f),
            PackingMode::Native => self.with_native_packed_be_bytes_if(predicate, f),
            PackingMode::Optimal => self.with_optimal_packed_be_bytes_if(predicate, f),
        }
    }
}
