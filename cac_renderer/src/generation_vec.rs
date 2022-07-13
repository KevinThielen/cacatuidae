use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Resource handle that is returned by the [Renderer] whenever a graphics resource, like a mesh,
/// shader or texture is created. It is similar to a normal Vec, with the difference that it
/// carries the generation data, in case a resource is released and another take the spot.
pub struct Handle<T: Copy> {
    pub(crate) index: usize,
    pub(crate) generation: usize,
    phantom: PhantomData<T>,
}

#[derive(Debug)]
struct Resource<R> {
    value: Option<R>,
    generation: usize,
}

#[derive(Debug)]
pub struct GenerationVec<K: Copy, V> {
    values: Vec<Resource<V>>,
    free: Vec<usize>,
    phantom: PhantomData<K>,
}

impl<K: Copy, V> Default for GenerationVec<K, V> {
    fn default() -> Self {
        Self {
            values: Vec::with_capacity(10),
            free: Vec::with_capacity(10),
            phantom: PhantomData,
        }
    }
}

impl<K: Copy, V> GenerationVec<K, V> {
    /// Constructor
    pub fn new() -> Self {
        Self::default()
    }

    /// reserves the capacity to avoid reallocation until it is reached
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            free: Vec::with_capacity(capacity),
            phantom: PhantomData,
        }
    }

    /// Removes the resource from the GenerationVec and pushes its index into the free list.
    /// The freelist will take the last entry as the index for a new value.
    pub fn remove(&mut self, handle: Handle<K>) {
        if let Some(resource) = self.values.get_mut(handle.index) {
            if resource.generation == handle.generation {
                resource.value = None;
                self.free.push(handle.index);
            }
        }
    }

    /// Returns an immutable reference to the value associated with the handle, or None if there is
    /// none.
    pub fn get(&self, handle: Handle<K>) -> Option<&V> {
        self.values
            .get(handle.index)
            .filter(|r| r.generation == handle.generation)
            .and_then(|r| r.value.as_ref())
    }

    /// Returns a mutable reference to the value associated with the handle, or None if there is
    /// none.
    pub fn get_mut(&mut self, handle: Handle<K>) -> Option<&mut V> {
        self.values
            .get_mut(handle.index)
            .filter(|r| r.generation == handle.generation)
            .and_then(|r| r.value.as_mut())
    }

    /// Updates the value the handle is refering to, without invalidating existing handles to it.
    /// It shouldn't be used to create entire different values, but rather change the existing one
    /// while keeping the same meaning.
    pub fn update(&mut self, handle: Handle<K>) -> Option<&mut V> {
        self.values.get_mut(handle.index).and_then(|resource| {
            if resource.generation == handle.generation {
                resource.value.as_mut()
            } else {
                None
            }
        })
    }

    /// Pushes a value into the GenerationVec and returns a handle to it.
    pub fn push(&mut self, value: V) -> Handle<K> {
        let index = self.free.pop().unwrap_or(self.values.len());

        if let Some(resource) = self.values.get_mut(index) {
            resource.value = Some(value);
            resource.generation += 1;

            Handle::<K> {
                index,
                generation: resource.generation,
                phantom: PhantomData,
            }
        } else {
            let resource = Resource::<V> {
                value: Some(value),
                generation: 0,
            };

            self.values.insert(index, resource);

            Handle::<K> {
                index,
                generation: 0,
                phantom: PhantomData,
            }
        }
    }

    pub fn clear(&mut self) {
        self.free.clear();
        self.values.iter_mut().enumerate().for_each(|(index, v)| {
            v.value = None;
            self.free.push(index);
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clear() {
        let mut gen_vec: GenerationVec<usize, i32> = GenerationVec::with_capacity(2);
        gen_vec.push(1);
        gen_vec.push(2);
        let to_be_removed = gen_vec.push(3);
        gen_vec.push(4);
        gen_vec.push(5);

        gen_vec.remove(to_be_removed);
        gen_vec.clear();

        assert_eq!(gen_vec.free.len(), 5);
        // they still hold None
        assert_eq!(gen_vec.values.len(), 5);

        let none_count = gen_vec.values.iter().filter(|v| v.value == None).count();
        assert_eq!(none_count, 5);
        let next_handle = gen_vec.push(5);
        assert_eq!(next_handle.generation, 1);
        assert_eq!(next_handle.index, 4);

        assert_eq!(*gen_vec.get(next_handle).unwrap(), 5);
    }

    #[test]
    fn insert() {
        let mut gen_vec: GenerationVec<usize, &str> = GenerationVec::with_capacity(2);
        let some_resource = "farty";
        let handle = gen_vec.push(some_resource);

        assert!(handle.index == 0);
        assert!(handle.generation == 0);

        let resource = gen_vec.get(handle);
        assert_eq!(resource.unwrap(), &"farty");

        let other_resource = "twart";
        let other_handle = gen_vec.push(other_resource);
        assert!(other_handle.index == 1);

        let resource = gen_vec.get(other_handle).unwrap();
        assert_eq!(resource, &"twart");
    }

    #[test]
    fn recycle_index() {
        let mut gen_vec: GenerationVec<usize, &str> = GenerationVec::with_capacity(2);
        let some_resource = "farty";
        let handle = gen_vec.push(some_resource);

        gen_vec.remove(handle);
        assert_eq!(gen_vec.free.len(), 1);

        let new_handle = gen_vec.push(some_resource);

        let no_handle = gen_vec.get_mut(handle);
        assert_eq!(no_handle, None);
        let no_handle = gen_vec.get(handle);
        assert_eq!(no_handle, None);

        assert_eq!(new_handle.generation, 1);
        assert_eq!(new_handle.index, 0);
    }

    #[test]
    fn update_value() {
        let mut gen_vec: GenerationVec<usize, &str> = GenerationVec::with_capacity(2);
        let some_resource = "farty";
        let handle = gen_vec.push(some_resource);

        let resource = gen_vec.get_mut(handle).unwrap();
        assert_eq!(resource, &"farty");

        *resource = "party";

        let resource = gen_vec.get(handle).unwrap();
        assert_eq!(resource, &"party");
    }
}
