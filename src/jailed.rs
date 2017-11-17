/*!
A version of StableVec where the `make_compact` methods are
safe. Anything that produces an Index must be called on a
JailedStableVec, and Indexes are only valid for the lifetime
of the JailedStableVec.

```
# fn main() {
use stable_vec::StableVec;

let mut sv = StableVec::new();

for _ in 0..2 {
    {
        let mut jailed = sv.jail();
        let idx1 = jailed.push(1);
        let idx2 = jailed.push(2);
        let sum = jailed[idx1] + jailed[idx2];
        jailed.remove(idx1);
        assert_eq!(3, sum);
    }

    sv.make_compact();
}

assert_eq!(&[2, 2], &*sv.into_vec());
# }
```

The following will not compile, because the first borrow of `wrapper` by the call to `jail` continues as long as `idx` is in scope, and conflicts with the call to `make_compact`:

```compile_fail
# fn main() {
# use stable_vec::StableVec;
let mut sv = StableVec::<i32>::new();
let idx = sv.jail().push(5);
sv.make_compact();
// error[E0499]: cannot borrow `wrapper` as mutable more than once at a time
# }
```
*/

use super::StableVec;
use ::std;

use std::marker::PhantomData;

impl<T> StableVec<T> {
    pub fn jail<'a>(&'a mut self) -> JailedStableVec<'a, T> {
        JailedStableVec(self)
    }
}

pub struct JailedStableVec<'a, T: 'a>(&'a mut StableVec<T>);

impl<'a, T> JailedStableVec<'a, T> {
    pub fn push(&mut self, value: T) -> Index<'a> {
        let idx = self.0.push(value);
        self.index(idx)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn remove(&mut self, idx: Index<'a>) -> Option<T> {
        self.0.remove(idx.idx)
    }

    pub fn is_compact(&self) -> bool {
        self.0.is_compact()
    }

    pub fn num_elements(&self) -> usize {
        self.0.num_elements()
    }

    fn index(&self, idx: usize) -> Index<'a> {
        Index {
            idx,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> std::ops::Index<Index<'a>> for JailedStableVec<'a, T> {
    type Output = T;

    fn index(&self, index: Index<'a>) -> &T {
        &self.0[index.idx]
    }
}

impl<'a, T> std::ops::IndexMut<Index<'a>> for JailedStableVec<'a, T> {

    fn index_mut(&mut self, index: Index<'a>) -> &mut T {
        &mut self.0[index.idx]
    }
}

#[derive(Clone, Copy)]
pub struct Index<'a> {
    idx: usize,
    _marker: PhantomData<&'a ()>,
}
