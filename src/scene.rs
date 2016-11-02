use std::path::Path;

use cache::{Cache, Get};
use resource::Load;

/// The scene type.
///
/// This type gathers all the required objects a scene needs to correctly handle and render all
/// visual effects.
pub struct Scene<'a> {
  /// Cache.
  cache: Cache<'a>
}

impl<'a> Scene<'a> {
  pub fn new<P>(root: P) -> Self where P: AsRef<Path>{
    Scene {
      cache: Cache::new(root),
    }
  }

  pub fn get_id<T>(&mut self, name: &str) -> Option<<Cache<'a> as Get<T>>::Id> where Cache<'a>: Get<T>, T: Load {
    self.cache.get_id(name)
  }

  pub fn get_by_id<T>(&mut self, id: &<Cache<'a> as Get<T>>::Id) -> Option<&T> where Cache<'a>: Get<T>, T: Load {
    self.cache.get_by_id(id)
  }

  pub fn get<T>(&mut self, name: &str) -> Option<&T> where Cache<'a>: Get<T>, T: Load {
    self.cache.get(name)
  }
}
