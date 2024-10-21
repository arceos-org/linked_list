#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

mod linked_list;
mod raw_list;
pub use linked_list::List;
pub use raw_list::{GetLinks, Links};

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __def_node_internal {
    ($(#[$meta:meta])* ($($vis:tt)*) struct $name:ident($type:ty);) => {
        $(#[$meta])*
        $($vis)* struct $name {
            inner: $type,
            links: $crate::Links<Self>,
        }

        impl $crate::GetLinks for $name {
            type EntryType = Self;

            #[inline]
            fn get_links(t: &Self) -> &$crate::Links<Self> {
                &t.links
            }
        }

        impl $name {
            #[doc = "Create a node"]
            $($vis)* const fn new(inner: $type) -> Self {
                Self {
                    inner,
                    links: $crate::Links::new(),
                }
            }

            #[inline]
            #[doc = "Get inner"]
            $($vis)* const fn inner(&self) -> &$type {
                &self.inner
            }
        }

        impl core::ops::Deref for $name {
            type Target = $type;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };

    ($(#[$meta:meta])* ($($vis:tt)*) struct $name:ident<$gen:ident>($type:ty); $($t:tt)*) => {
        $(#[$meta])*
        $($vis)* struct $name<$gen> {
            inner: $type,
            links: $crate::Links<Self>,
        }

        impl<$gen> $crate::GetLinks for $name<$gen> {
            type EntryType = Self;

            #[inline]
            fn get_links(t: &Self) -> &$crate::Links<Self> {
                &t.links
            }
        }

        impl<$gen> $name<$gen> {
            #[doc = "Create a node"]
            $($vis)* const fn new(inner: $type) -> Self {
                Self {
                    inner,
                    links: $crate::Links::new(),
                }
            }

            #[inline]
            #[doc = "Get inner"]
            $($vis)* const fn inner(&self) -> &$type {
                &self.inner
            }
        }

        impl<$gen> core::ops::Deref for $name<$gen> {
            type Target = $type;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
}

/// A macro for create a node type that can be used in List.
///
/// # Syntax
///
/// ```ignore
/// def_node! {
/// /// A node with usize value.
/// [pub] struct UsizedNode(usize);
/// /// A node with generic inner type.
/// [pub] struct WrapperNode<T>(T);
/// }
/// ```
///
/// # Example
///
/// ```rust
/// use linked_list::{def_node, List};
///
/// def_node!(
///     /// An example Node with usize
///     struct ExampleNode(usize);
///     /// An example Node with generic Inner type
///     struct GenericNode<T>(T);
/// );
///
/// let node1 = Box::new(ExampleNode::new(0));
/// let node2 = Box::new(ExampleNode::new(1));
/// let mut list =  List::<Box<ExampleNode>>::new();
///
/// list.push_back(node1);
/// list.push_back(node2);
///
/// for (i,e) in list.iter().enumerate() {
///     assert!(*e.inner() == i);
/// }
///
/// let node1 = Box::new(GenericNode::new(0));
/// let node2 = Box::new(GenericNode::new(1));
/// let mut list =  List::<Box<GenericNode<usize>>>::new();
///
/// list.push_back(node1);
/// list.push_back(node2);
///
/// for (i,e) in list.iter().enumerate() {
///     assert!(*e.inner() == i);
/// }
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! def_node {
    ($(#[$meta:meta])* struct $name:ident($type:ty); $($t:tt)*) => {
        __def_node_internal!($(#[$meta])* () struct $name($type););
        def_node!($($t)*);

    };
    ($(#[$meta:meta])* pub struct $name:ident($type:ty); $($t:tt)*) => {
        __def_node_internal!($(#[$meta])* (pub) struct $name($type);$($t)*);
        def_node!($($t)*);

    };
    ($(#[$meta:meta])* pub ($($vis:tt)+) struct $name:ident($type:ty); $($t:tt)*) => {
        __def_node_internal!($(#[$meta])* (pub ($($vis)+)) struct $name($type);$($t)*);
        def_node!($($t)*);

    };

    ($(#[$meta:meta])* struct $name:ident<$gen:ident>($type:ty); $($t:tt)*) => {
        __def_node_internal!($(#[$meta])* () struct $name<$gen>($type); $($t)*);
        def_node!($($t)*);

    };
    ($(#[$meta:meta])* pub struct $name:ident<$gen:ident>($type:ty); $($t:tt)*) => {
        __def_node_internal!($(#[$meta])* (pub) struct $name<$gen>($type);$($t)*);
        def_node!($($t)*);

    };
    ($(#[$meta:meta])* pub ($($vis:tt)+) struct $name:ident<$gen:ident>($type:ty); $($t:tt)*) => {
        __def_node_internal!($(#[$meta])* (pub ($($vis)+)) struct $name<$gen>($type);$($t)*);
        def_node!($($t)*);

    };
    () => ()
}
