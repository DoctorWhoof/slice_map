use alloc::boxed::Box;
use slotmap::{Key, SecondaryMap, SlotMap, SparseSecondaryMap};

/// Trait to abstract operations on storage of slices
pub trait SliceStorage<K, V>: Default {
    fn insert(&mut self, value: V) -> K;
    fn remove(&mut self, key: K) -> Option<V>;
    fn get(&self, key: K) -> Option<&V>;
    fn iter(&self) -> Box<dyn Iterator<Item = (K, &V)> + '_>;
    fn values(&self) -> Box<dyn Iterator<Item = &V> + '_>;
    fn values_mut(&mut self) -> Box<dyn Iterator<Item = &mut V> + '_>;
    fn is_empty(&self) -> bool;
}

impl<K, V> SliceStorage<K, V> for SlotMap<K, V>
where
    K: Key,
{

    #[inline(always)]
    fn insert(&mut self, value: V) -> K {
        self.insert(value)
    }

    #[inline(always)]
    fn remove(&mut self, key: K) -> Option<V> {
        self.remove(key)
    }

    #[inline(always)]
    fn get(&self, key: K) -> Option<&V> {
        self.get(key)
    }

    #[inline(always)]
    fn iter(&self) -> Box<dyn Iterator<Item = (K, &V)> + '_> {
        Box::new(self.iter())
    }

    #[inline(always)]
    fn values(&self) -> Box<dyn Iterator<Item = &V> + '_> {
        Box::new(self.values())
    }

    #[inline(always)]
    fn values_mut(&mut self) -> Box<dyn Iterator<Item = &mut V> + '_> {
        Box::new(self.values_mut())
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> SliceStorage<K, V> for SecondaryMap<K, V>
where
    K: Key,
{
    #[inline(always)]
    fn insert(&mut self, _value: V) -> K {
        panic!("SecondaryMap does not support insert; keys must be pre-created")
    }

    #[inline(always)]
    fn remove(&mut self, key: K) -> Option<V> {
        self.remove(key)
    }

    #[inline(always)]
    fn get(&self, key: K) -> Option<&V> {
        self.get(key)
    }

    #[inline(always)]
    fn iter(&self) -> Box<dyn Iterator<Item = (K, &V)> + '_> {
        Box::new(self.iter())
    }

    #[inline(always)]
    fn values(&self) -> Box<dyn Iterator<Item = &V> + '_> {
        Box::new(self.values())
    }

    #[inline(always)]
    fn values_mut(&mut self) -> Box<dyn Iterator<Item = &mut V> + '_> {
        Box::new(self.values_mut())
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}


impl<K, V> SliceStorage<K, V> for SparseSecondaryMap<K, V>
where
    K: Key,
{
    #[inline(always)]
    fn insert(&mut self, _value: V) -> K {
        panic!("SecondaryMap does not support insert; keys must be pre-created")
    }

    #[inline(always)]
    fn remove(&mut self, key: K) -> Option<V> {
        self.remove(key)
    }

    #[inline(always)]
    fn get(&self, key: K) -> Option<&V> {
        self.get(key)
    }

    #[inline(always)]
    fn iter(&self) -> Box<dyn Iterator<Item = (K, &V)> + '_> {
        Box::new(self.iter())
    }

    #[inline(always)]
    fn values(&self) -> Box<dyn Iterator<Item = &V> + '_> {
        Box::new(self.values())
    }

    #[inline(always)]
    fn values_mut(&mut self) -> Box<dyn Iterator<Item = &mut V> + '_> {
        Box::new(self.values_mut())
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
