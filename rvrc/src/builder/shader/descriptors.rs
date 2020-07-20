use std::collections::HashMap;
use thiserror::Error;
use rvr_assets::shader::{ShaderStageFlags, DescriptorType};
use crate::Error;

#[derive(Debug, Error)]
pub enum DescriptorError {
    #[error("Duplicate binding in descriptor set")]
    DuplicateBinding,

    #[error("Cannot merge two descriptor bindings")]
    CannotMerge,
}

pub struct DescriptorBinding {
    binding: u32,
    descriptor_type: DescriptorType,
    count: u32,
    stage_flags: ShaderStageFlags
}

impl DescriptorBinding {
    pub fn new(binding: u32, descriptor_type: DescriptorType, count: u32, stage_flags: ShaderStageFlags) -> Self {
        Self {
            binding,
            descriptor_type,
            count,
            stage_flags,
        }
    }

    pub fn merge(&self, other: &Self) -> Result<Self, Error> {
        if other.binding != self.binding {
            return Err(DescriptorError::CannotMerge.into());
        }

        if other.descriptor_type != self.descriptor_type {
            return Err(DescriptorError::CannotMerge.into());
        }

        if other.count != self.count {
            return Err(DescriptorError::CannotMerge.into());
        }

        Ok(Self {
            binding: self.binding,
            descriptor_type: self.descriptor_type,
            count: self.count,
            stage_flags: self.stage_flags | other.stage_flags,
        })
    }
}

pub struct DescriptorSet {
    set: u32,
    bindings: HashMap<u32, DescriptorBinding>,
}

impl DescriptorSet {
    pub fn new(set: u32) -> Self {
        Self {
            set,
            bindings: HashMap::new(),
        }
    }

    pub fn insert_binding(&mut self, binding: DescriptorBinding) -> Result<(), Error> {
        let id = binding.binding;
        if let Some(existing) = self.bindings.get(&id) {
            let new = existing.merge(&binding)?;
            self.bindings.insert(id, new);
        } else {
            self.bindings.insert(id, binding);
        }

        Ok(())
    }
}

pub struct DescriptorSets {
    sets: HashMap<u32, DescriptorSet>,
}

impl DescriptorSets {
    pub fn new() -> Self {
        Self {
            sets: HashMap::new(),
        }
    }

    pub fn insert_set(&mut self, descriptor_set: DescriptorSet) -> Result<(), Error> {
        let id = descriptor_set.set;
        if let Some(existing) = self.sets.get_mut(&id) {
            for (id, binding) in descriptor_set.bindings {
                existing.insert_binding(binding)?;
            }
        } else {
            self.sets.insert(id, descriptor_set);
        }

        Ok(())
    }

    pub fn build_layouts(&self) -> Vec<rvr_assets::shader::DescriptorSetLayout> {
        use rvr_assets::shader::*;

        let mut sets = Vec::new();
        for set in self.sets.values() {
            let mut bindings = Vec::new();

            for binding in set.bindings.values() {
                bindings.push(DescriptorSetLayoutBinding::new(
                    binding.binding,
                    binding.descriptor_type,
                    binding.count,
                    binding.stage_flags,
                ));
            }

            sets.push(DescriptorSetLayout::new(
                set.set,
                bindings,
            ));
        }

        sets
    }
}
