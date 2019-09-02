pub struct KvStore;

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }

    pub fn set(self : &mut KvStore,
               key : String, value : String) {
        unimplemented!();
    }

    pub fn get(self : &KvStore,
               key : String) -> Option<String> {
        unimplemented!();
    }

    pub fn remove(self : &mut KvStore,
                  key : String) {
        unimplemented!();
    }
}
