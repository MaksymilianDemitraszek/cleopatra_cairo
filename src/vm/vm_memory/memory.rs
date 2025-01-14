use crate::vm::errors::memory_errors::MemoryError;
use crate::{types::relocatable::MaybeRelocatable, utils::from_relocatable_to_indexes};

#[derive(Clone)]
pub struct Memory {
    pub data: Vec<Vec<Option<MaybeRelocatable>>>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: Vec::<Vec<Option<MaybeRelocatable>>>::new(),
        }
    }
    ///Inserts an MaybeRelocatable value into an address given by a MaybeRelocatable::Relocatable
    /// Will panic if the segment index given by the address corresponds to a non-allocated segment
    /// If the address isnt contiguous with previously inserted data, memory gaps will be represented by inserting None values
    pub fn insert(
        &mut self,
        key: &MaybeRelocatable,
        val: &MaybeRelocatable,
    ) -> Result<(), MemoryError> {
        if let MaybeRelocatable::RelocatableValue(relocatable) = key {
            let (i, j) = from_relocatable_to_indexes(relocatable.clone());
            //Check that the memory segment exists
            if self.data.len() < i + 1 {
                return Err(MemoryError::UnallocatedSegment(self.data.len(), i + 1));
            }
            //Check if the element is inserted next to the last one on the segment
            //Forgoing this check would allow data to be inserted in a different index
            if self.data[i].len() < j {
                //Insert none values to represent gaps in memory
                for _ in 0..(j - self.data[i].len()) {
                    self.data[i].push(None)
                }
            }
            self.data[i].push(Some(val.clone()))
        } else {
            return Err(MemoryError::AddressNotRelocatable);
        }
        Ok(())
    }

    pub fn get(&self, key: &MaybeRelocatable) -> Result<Option<&MaybeRelocatable>, MemoryError> {
        if let MaybeRelocatable::RelocatableValue(relocatable) = key {
            let (i, j) = from_relocatable_to_indexes(relocatable.clone());
            if self.data.len() > i && self.data[i].len() > j {
                if let Some(ref element) = self.data[i][j] {
                    return Ok(Some(element));
                }
            }
            Ok(None)
        } else {
            Err(MemoryError::AddressNotRelocatable)
        }
    }

    #[allow(dead_code)]
    pub fn from(
        key_val_list: Vec<(MaybeRelocatable, MaybeRelocatable)>,
        num_segements: usize,
    ) -> Result<Memory, MemoryError> {
        let mut memory = Memory::new();
        for _ in 0..num_segements {
            memory.data.push(Vec::new());
        }
        for (key, val) in key_val_list.iter() {
            memory.insert(key, val)?;
        }
        Ok(memory)
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod memory_tests {
    use crate::bigint;

    use super::*;
    use num_bigint::BigInt;
    use num_traits::FromPrimitive;

    #[test]
    fn insert_and_get_succesful() {
        let key = MaybeRelocatable::from((0, 0));
        let val = MaybeRelocatable::from(bigint!(5));
        let mut memory = Memory::new();
        memory.data.push(Vec::new());
        memory.insert(&key, &val).unwrap();
        assert_eq!(
            memory.get(&key).unwrap(),
            Some(&MaybeRelocatable::from(bigint!(5)))
        );
    }

    #[test]
    fn get_non_allocated_memory() {
        let key = MaybeRelocatable::from((0, 0));
        let memory = Memory::new();
        assert_eq!(memory.get(&key).unwrap(), None);
    }

    #[test]
    fn get_non_existant_element() {
        let key = MaybeRelocatable::from((0, 0));
        let memory = Memory::new();
        assert_eq!(memory.get(&key).unwrap(), None);
    }

    #[test]
    #[should_panic]
    fn get_non_relocatable_key() {
        let key = MaybeRelocatable::from(bigint!(0));
        let memory = Memory::new();
        memory.get(&key).unwrap();
    }

    #[test]
    #[should_panic]
    fn insert_non_allocated_memory() {
        let key = MaybeRelocatable::from((0, 0));
        let val = MaybeRelocatable::from(bigint!(5));
        let mut memory = Memory::new();
        memory.insert(&key, &val).unwrap();
    }

    #[test]
    fn insert_non_contiguous_element() {
        let key_a = MaybeRelocatable::from((0, 0));
        let key_b = MaybeRelocatable::from((0, 2));
        let val = MaybeRelocatable::from(bigint!(5));
        let mut memory = Memory::new();
        memory.data.push(Vec::new());
        memory.insert(&key_a, &val).unwrap();
        memory.insert(&key_b, &val).unwrap();
        assert_eq!(memory.get(&key_b).unwrap(), Some(&val));
    }

    #[test]
    fn insert_non_contiguous_element_memory_gaps_none() {
        let key_a = MaybeRelocatable::from((0, 0));
        let key_b = MaybeRelocatable::from((0, 5));
        let val = MaybeRelocatable::from(bigint!(5));
        let mut memory = Memory::new();
        memory.data.push(Vec::new());
        memory.insert(&key_a, &val).unwrap();
        memory.insert(&key_b, &val).unwrap();
        assert_eq!(memory.get(&key_b).unwrap(), Some(&val));
        assert_eq!(memory.get(&MaybeRelocatable::from((0, 1))).unwrap(), None);
        assert_eq!(memory.get(&MaybeRelocatable::from((0, 2))).unwrap(), None);
        assert_eq!(memory.get(&MaybeRelocatable::from((0, 3))).unwrap(), None);
        assert_eq!(memory.get(&MaybeRelocatable::from((0, 4))).unwrap(), None);
    }

    #[test]
    fn from_array_test() {
        let mem = Memory::from(
            vec![(
                MaybeRelocatable::from((1, 0)),
                MaybeRelocatable::from(bigint!(5)),
            )],
            2,
        )
        .unwrap();
        assert_eq!(
            matches!(mem.get(&MaybeRelocatable::from((1, 0))), _val_clone),
            true
        );
    }
}
