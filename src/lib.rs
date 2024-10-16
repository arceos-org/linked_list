//! Linked lists that supports arbitrary removal in constant time.
//!
//! It is based on the linked list implementation in [Rust-for-Linux][1].
//!
//! [1]: https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/linked_list.rs
//!
//! In more general use cases, shoud not use RawList directly,
//! suggest use smart pointers of nodes and move ownership of smart
//! pointers to List:
//!
//! ```
//! use linked_list::{GetLinks, Links, List};
//!
//! type InnerType = usize;
//!
//! pub struct ExampleNode {
//!     pub inner: InnerType,
//!     links: Links<Self>,
//! }
//!
//! impl GetLinks for ExampleNode {
//!     type EntryType = Self;
//!
//!     fn get_links(t: &Self) -> &Links<Self> {
//!         &t.links
//!     }
//! }
//!
//! impl ExampleNode {
//!     fn new(inner: InnerType) -> Self {
//!         Self {
//!             inner,
//!             links: Links::new()
//!         }
//!     }
//!
//!     fn inner(&self) -> &InnerType {
//!         &self.inner
//!     }
//! }
//!
//! let node1 = Box::new(ExampleNode::new(0));
//! let node2 = Box::new(ExampleNode::new(1));
//! let mut list =  List::<Box<ExampleNode>>::new();
//!
//! list.push_back(node1);
//! list.push_back(node2);
//!
//! //Support Iter
//! for (i,e) in list.iter().enumerate() {
//!     assert!(*e.inner() == i);
//! }
//!
//! // Pop drop
//! assert!(*list.pop_front().unwrap().inner() == 0);
//! assert!(*list.pop_front().unwrap().inner() == 1);
//!
//! ```
//!
//! use def_list_node macro
//! ```
//! use linked_list::{def_node, def_generic_node, List};
//!
//! def_node!(ExampleNode, usize);
//!
//! let node1 = Box::new(ExampleNode::new(0));
//! let node2 = Box::new(ExampleNode::new(1));
//! let mut list =  List::<Box<ExampleNode>>::new();
//!
//! list.push_back(node1);
//! list.push_back(node2);
//!
//! for (i,e) in list.iter().enumerate() {
//!     assert!(*e.inner() == i);
//! }
//!
//! def_generic_node!(GenericExampleNode);
//!
//! let node1 = Box::new(GenericExampleNode::new(0));
//! let node2 = Box::new(GenericExampleNode::new(1));
//! let mut list = List::<Box<GenericExampleNode<usize>>>::new();
//!
//! list.push_back(node1);
//! list.push_back(node2);
//!
//! //Support Iter
//! for (i,e) in list.iter().enumerate() {
//!     assert!(*e.inner() == i);
//! }
//! ```

#![cfg_attr(not(test), no_std)]

mod linked_list;
mod raw_list;
pub use linked_list::List;
pub use raw_list::{GetLinks, Links};

#[allow(missing_docs)]
#[macro_export]
macro_rules! def_node {
    ($struct_name:ident, $type:ty) => {
        #[doc = "A node wrapper for inner type "]
        pub struct $struct_name {
            inner: $type,
            links: $crate::Links<Self>,
        }

        impl $crate::GetLinks for $struct_name {
            type EntryType = Self;

            #[inline]
            fn get_links(t: &Self) -> &$crate::Links<Self> {
                &t.links
            }
        }

        impl $struct_name {
            #[doc = "Create a node"]
            pub const fn new(inner: $type) -> Self {
                Self {
                    inner,
                    links: $crate::Links::new(),
                }
            }

            #[inline]
            #[doc = "Get inner"]
            pub fn inner(&self) -> &$type {
                &self.inner
            }
        }

        impl core::ops::Deref for $struct_name {
            type Target = $type;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! def_generic_node {
    ($struct_name:ident) => {
        #[doc = "A node wrapper include a generic type"]
        pub struct $struct_name<T> {
            inner: T,
            links: $crate::Links<Self>,
        }

        impl<T> $crate::GetLinks for $struct_name<T> {
            type EntryType = Self;

            #[inline]
            fn get_links(t: &Self) -> &$crate::Links<Self> {
                &t.links
            }
        }

        impl<T> $struct_name<T> {
            #[doc = "Create a node"]
            pub const fn new(inner: T) -> Self {
                Self {
                    inner,
                    links: $crate::Links::new(),
                }
            }

            #[inline]
            #[doc = "Get inner"]
            pub fn inner(&self) -> &T {
                &self.inner
            }
        }

        impl<T> core::ops::Deref for $struct_name<T> {
            type Target = T;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
}
