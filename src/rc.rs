use std::ops::Deref;

pub struct RcInner<T> {
    value: T,
    refcount: usize
} 

pub struct Rc <T> {
    inner: *const RcInner<T>
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner { value: v, refcount: 1});
        Rc { inner: Box::into_raw(inner)}
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe{
            &*self.inner
        };
        inner.refcount += 1;
        Rc {
            inner: self.inner,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe{
            &*self.inner
        }.value
    }
}
