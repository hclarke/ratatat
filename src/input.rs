use std::rc::Rc;
use core::ops::Deref;

#[derive(Clone)]
pub struct Shared<T:?Sized> {
	src: Rc<Vec<u8>>,
	ptr: *const T,
}

impl<T:?Sized> Shared<T> {
	pub fn map<V:?Sized, F:FnOnce(&T) -> &V>(self, f:F) -> Shared<V> {
		let t = unsafe { &*self.ptr };
		let v = f(t);
		let v = v as *const V;
		Shared {
			src: self.src,
			ptr: v,
		}
	}

	pub fn src(&self) -> &Rc<Vec<u8>> {
		&self.src
	}
}

impl Shared<[u8]> {
	pub fn new(src: Rc<Vec<u8>>) -> Self {
		let ptr = &src[..] as *const [u8];
		Shared {
			src,
			ptr,
		}
	}
}

impl<T:?Sized> Deref for Shared<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		unsafe { &*self.ptr }
	}
}

impl<T:?Sized> PartialEq for Shared<T> {
	fn eq(&self, rhs: &Self) -> bool {
		self.ptr == rhs.ptr && self.src.as_ptr() == rhs.src.as_ptr()
	}
}