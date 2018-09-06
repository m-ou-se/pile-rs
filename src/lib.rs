extern crate stable_deref_trait;

use stable_deref_trait::StableDeref;
use std::cell::Cell;
use std::mem::transmute;
use std::ops::Deref;

/// A pile of `T`.
///
/// You can throw more `T` on the pile, but never remove them: The pile will
/// only grow bigger. This means references to existing things on the pile will
/// stay valid. Therefore, you can add elements through a non-mutable reference
/// to a `Pile`.
///
/// (You can, of course, destroy the entire pile. But, like any other Rust data
/// structure, only when there are no references to any elements left.)
pub struct Pile<T> {
	elements: Cell<Vec<T>>,
}

impl<T> Pile<T> {
	pub fn new() -> Self {
		Pile {
			elements: Cell::new(Vec::new()),
		}
	}

	pub fn from_elements(elements: Vec<T>) -> Self {
		Pile {
			elements: Cell::new(elements),
		}
	}

	pub fn into_elements(self) -> Vec<T> {
		self.elements.into_inner()
	}

	pub fn mut_elements(&mut self) -> &mut Vec<T> {
		self.elements.get_mut()
	}
}

impl<T: StableDeref> Pile<T> {
	pub fn add<'s>(&'s self, element: T) -> &'s <T as Deref>::Target {
		unsafe {
			let reference =
				transmute::<&<T as Deref>::Target, &'s <T as Deref>::Target>(element.deref());
			(*self.elements.as_ptr()).push(element);
			reference
		}
	}
}

#[test]
fn test() {
	let pile = Pile::new();
	{
		let hello = pile.add("hello".to_string());
		let world = pile.add("world".to_string());
		assert_eq!(hello, "hello");
		assert_eq!(world, "world");
	}
	let mut pile = pile;
	pile.mut_elements()[1] = "world!".to_string();
	assert_eq!(pile.into_elements(), vec!["hello", "world!"]);
}
