#[macro_export]
macro_rules! impl_generational_list {
    ($GenerationalList:ident<$GenerationalKey:ident, $Item:ident>) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $GenerationalKey {
            index: usize,
            generation: usize,
        }

        impl $GenerationalKey {
            fn new(index: usize, generation: usize) -> Self {
                Self {
                    index,
                    generation,
                }
            }
        }

        pub struct $GenerationalList {
            items: Vec<Option<$Item>>,
            generations: Vec<usize>,
            empty: Vec<usize>,
        }

        impl $GenerationalList {
            pub fn new() -> Self {
                Self {
                    items: Vec::new(),
                    generations: Vec::new(),
                    empty: Vec::new(),
                }
            }

            pub fn with_capacity(capacity: usize) -> Self {
                Self {
                    items: Vec::with_capacity(capacity),
                    generations: Vec::with_capacity(capacity),
                    empty: Vec::new(),
                }
            }

            pub fn insert(&mut self, item: $Item) -> $GenerationalKey {
                if let Some(index) = self.empty.pop() {
                    let new_generation = self.generations[index] + 1;

                    self.items[index] = Some(item);
                    self.generations[index] = new_generation;

                    $GenerationalKey::new(index, new_generation)
                } else {
                    let index = self.items.len();
                    let new_generation = 1;

                    self.items.push(Some(item));
                    self.generations.push(new_generation);

                    $GenerationalKey::new(index, new_generation)
                }
            }

            pub fn get(&self, key: &$GenerationalKey) -> Option<&$Item> {
                let index = key.index;
                let generation = key.generation;

                if index >= self.items.len() {
                    return None;
                }

                if generation != self.generations[index] {
                    return None;
                }

                match self.items[index] {
                    Some(ref item) => Some(item),
                    None => None,
                }
            }

            pub fn get_mut(&mut self, key: &$GenerationalKey) -> Option<&mut $Item> {
                let index = key.index;
                let generation = key.generation;

                if index >= self.items.len() {
                    return None;
                }

                if generation != self.generations[index] {
                    return None;
                }

                match self.items[index] {
                    Some(ref mut item) => Some(item),
                    None => None,
                }
            }

            pub fn remove(&mut self, key: &$GenerationalKey) -> Option<$Item> {
                let index = key.index;
                let generation = key.generation;

                if index >= self.items.len() {
                    return None;
                }

                if generation != self.generations[index] {
                    return None;
                }

                let previous = self.items[index].take();
                self.generations[index] += 1;
                self.empty.push(index);

                previous
            }
        }
    }
}

#[macro_export]
macro_rules! impl_secondary_list {
    ($SecondaryList:ident<$GenerationalKey:ident, $Item:ident>) => {
        pub struct $SecondaryList {
            items: Vec<Option<$Item>>,
            generations: Vec<usize>,
        }

        impl $SecondaryList {
            pub fn new() -> Self {
                Self {
                    items: Vec::new(),
                    generations: Vec::new(),
                }
            }

            pub fn with_capacity(capacity: usize) -> Self {
                Self {
                    items: Vec::with_capacity(capacity),
                    generations: Vec::with_capacity(capacity),
                }
            }

            pub fn set(&mut self, key: &$GenerationalKey, item: $Item) -> Option<$Item> {
                while key.index + 1 < self.items.len() {
                    self.items.push(None);
                }

                if self.generations[key.index] > key.generation {
                    return None;
                }

                self.items[key.index].replace(item)
            }

            pub fn get(&self, key: &$GenerationalKey) -> Option<&$Item> {
                let index = key.index;
                let generation = key.generation;

                if index >= self.items.len() {
                    return None;
                }

                if generation != self.generations[index] {
                    return None;
                }

                match self.items[index] {
                    Some(ref item) => Some(item),
                    None => None,
                }
            }

            pub fn get_mut(&mut self, key: &$GenerationalKey) -> Option<&mut $Item> {
                let index = key.index;
                let generation = key.generation;

                if index >= self.items.len() {
                    return None;
                }

                if generation != self.generations[index] {
                    return None;
                }

                match self.items[index] {
                    Some(ref mut item) => Some(item),
                    None => None,
                }
            }

            pub fn remove(&mut self, key: &$GenerationalKey) -> Option<$Item> {
                let index = key.index;
                let generation = key.generation;

                if index >= self.items.len() {
                    return None;
                }

                if generation != self.generations[index] {
                    return None;
                }

                let previous = self.items[index].take();
                previous
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        impl_generational_list,
        impl_secondary_list,
    };

    impl_generational_list!(TestList<TestKey, TestValue>);
    impl_secondary_list!(TestDataList<TestKey, TestData>);

    #[derive(Debug, Eq, PartialEq)]
    pub struct TestValue(usize);
    #[derive(Debug, Eq, PartialEq)]
    pub struct TestData(usize);

    #[test]
    fn it_works() {
        let mut list = TestList::new();

        let key0 = list.insert(TestValue(0));
        let key1 = list.insert(TestValue(1));
        let key2 = list.insert(TestValue(2));

        assert_eq!(list.get(&key1), Some(&TestValue(1)));
        assert_eq!(list.get_mut(&key1), Some(&mut TestValue(1)));

        assert_eq!(list.remove(&key0), Some(TestValue(0)));
        assert_eq!(list.remove(&key2), Some(TestValue(2)));

        assert_eq!(list.get(&key0), None);
    }

    fn it_works_with_secondary_lists() {
        let mut list = TestList::new();
        let mut data = TestDataList::new();

        let key = list.insert(TestValue(0));

        data.set(&key, TestData(0));

        assert_eq!(data.get(&key), Some(&TestData(0)));
        assert_eq!(data.get_mut(&key), Some(&mut TestData(0)));
        
        assert_eq!(data.remove(&key), Some(TestData(0)));
    }
}
