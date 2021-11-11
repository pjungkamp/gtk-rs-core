// Take a look at the license at the top of the repository in the LICENSE file.

//! `Variant` binding and helper traits.
//!
//! [`Variant`](struct.Variant.html) is an immutable dynamically-typed generic
//! container. Its type and value are defined at construction and never change.
//!
//! `Variant` types are described by [`VariantType`](../struct.VariantType.html)
//! "type strings".
//!
//! Although `GVariant` supports arbitrarily complex types, this binding is
//! currently limited to the basic ones: `bool`, `u8`, `i16`, `u16`, `i32`,
//! `u32`, `i64`, `u64`, `f64`, `&str`/`String`, and [`VariantDict`](../struct.VariantDict.html).
//!
//! # Examples
//!
//! ```
//! use glib::prelude::*; // or `use gtk::prelude::*;`
//! use glib::{Variant, FromVariant, ToVariant};
//! use std::collections::HashMap;
//!
//! // Using the `ToVariant` trait.
//! let num = 10.to_variant();
//!
//! // `is` tests the type of the value.
//! assert!(num.is::<i32>());
//!
//! // `get` tries to extract the value.
//! assert_eq!(num.get::<i32>(), Some(10));
//! assert_eq!(num.get::<u32>(), None);
//!
//! // `get_str` tries to borrow a string slice.
//! let hello = "Hello!".to_variant();
//! assert_eq!(hello.str(), Some("Hello!"));
//! assert_eq!(num.str(), None);
//!
//! // `fixed_array` tries to borrow a fixed size array,
//! // rather than creating a deep copy which would be expensive for
//! // nontrivially sized byte arrays.
//! // The test data here is the zstd compression header, which
//! // stands in for arbitrary binary data (e.g. not UTF-8).
//! let bufdata = b"\xFD\x2F\xB5\x28";
//! let bufv = bufdata.to_variant();
//! assert_eq!(bufv.fixed_array::<u8>().unwrap(), bufdata);
//! assert!(num.fixed_array::<u8>().is_err());
//!
//! // Variant carrying a Variant
//! let variant = Variant::from_variant(&hello);
//! let variant = variant.as_variant().unwrap();
//! assert_eq!(variant.str(), Some("Hello!"));
//!
//! // Variant carrying an array
//! let array = ["Hello", "there!"];
//! let variant = array.into_iter().collect::<Variant>();
//! assert_eq!(variant.n_children(), 2);
//! assert_eq!(variant.child_value(0).str(), Some("Hello"));
//! assert_eq!(variant.child_value(1).str(), Some("there!"));
//!
//! // You can also convert from and to a Vec
//! let variant = vec!["Hello", "there!"].to_variant();
//! assert_eq!(variant.n_children(), 2);
//! let vec = <Vec<String>>::from_variant(&variant).unwrap();
//! assert_eq!(vec[0], "Hello");
//!
//! // Conversion to and from HashMap and BTreeMap is also possible
//! let mut map: HashMap<u16, &str> = HashMap::new();
//! map.insert(1, "hi");
//! map.insert(2, "there");
//! let variant = map.to_variant();
//! assert_eq!(variant.n_children(), 2);
//! let map: HashMap<u16, String> = HashMap::from_variant(&variant).unwrap();
//! assert_eq!(map[&1], "hi");
//! assert_eq!(map[&2], "there");
//!
//! // And conversion to and from tuples.
//! let variant = ("hello", 42u16, vec![ "there", "you" ],).to_variant();
//! assert_eq!(variant.n_children(), 3);
//! assert_eq!(variant.type_().as_str(), "(sqas)");
//! let tuple = <(String, u16, Vec<String>)>::from_variant(&variant).unwrap();
//! assert_eq!(tuple.0, "hello");
//! assert_eq!(tuple.1, 42);
//! assert_eq!(tuple.2, &[ "there", "you"]);
//!
//! // `Option` is supported as well, through maybe types
//! let variant = Some("hello").to_variant();
//! assert_eq!(variant.n_children(), 1);
//! let mut s = <Option<String>>::from_variant(&variant).unwrap();
//! assert_eq!(s.unwrap(), "hello");
//! s = None;
//! let variant = s.to_variant();
//! assert_eq!(variant.n_children(), 0);
//! let s = <Option<String>>::from_variant(&variant).unwrap();
//! assert!(s.is_none());
//! ```

use crate::bytes::Bytes;
use crate::gstring::GString;
use crate::translate::*;
use crate::StaticType;
use crate::Type;
use crate::VariantTy;
use crate::VariantType;
use crate::{VariantIter, VariantStrIter};
use std::borrow::Cow;
use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt;
use std::hash::{BuildHasher, Hash, Hasher};
use std::mem;
use std::ptr;
use std::slice;
use std::str;

wrapper! {
    /// A generic immutable value capable of carrying various types.
    ///
    /// See the [module documentation](index.html) for more details.
    #[doc(alias = "GVariant")]
    pub struct Variant(Shared<ffi::GVariant>);

    match fn {
        ref => |ptr| ffi::g_variant_ref_sink(ptr),
        unref => |ptr| ffi::g_variant_unref(ptr),
    }
}

impl StaticType for Variant {
    fn static_type() -> Type {
        Type::VARIANT
    }
}

#[doc(hidden)]
impl crate::value::ValueType for Variant {
    type Type = Variant;
}

#[doc(hidden)]
unsafe impl<'a> crate::value::FromValue<'a> for Variant {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a crate::Value) -> Self {
        let ptr = gobject_ffi::g_value_dup_variant(value.to_glib_none().0);
        assert!(!ptr.is_null());
        from_glib_full(ptr)
    }
}

#[doc(hidden)]
impl crate::value::ToValue for Variant {
    fn to_value(&self) -> crate::Value {
        unsafe {
            let mut value = crate::Value::from_type(Variant::static_type());
            gobject_ffi::g_value_take_variant(
                value.to_glib_none_mut().0,
                self.to_glib_full() as *mut _,
            );
            value
        }
    }

    fn value_type(&self) -> crate::Type {
        Variant::static_type()
    }
}

#[doc(hidden)]
impl crate::value::ToValueOptional for Variant {
    fn to_value_optional(s: Option<&Self>) -> crate::Value {
        let mut value = crate::Value::for_value_type::<Self>();
        unsafe {
            gobject_ffi::g_value_take_variant(
                value.to_glib_none_mut().0,
                s.to_glib_full() as *mut _,
            );
        }

        value
    }
}

/// An error returned from the [`try_get`](struct.Variant.html#method.try_get) function
/// on a [`Variant`](struct.Variant.html) when the expected type does not match the actual type.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VariantTypeMismatchError {
    pub actual: VariantType,
    pub expected: VariantType,
}

impl VariantTypeMismatchError {
    pub fn new(actual: VariantType, expected: VariantType) -> Self {
        Self { actual, expected }
    }
}

impl fmt::Display for VariantTypeMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type mismatch: Expected '{}' got '{}'",
            self.expected, self.actual
        )
    }
}

impl std::error::Error for VariantTypeMismatchError {}

impl Variant {
    /// Returns the type of the value.
    #[doc(alias = "g_variant_get_type")]
    pub fn type_(&self) -> &VariantTy {
        unsafe { VariantTy::from_ptr(ffi::g_variant_get_type(self.to_glib_none().0)) }
    }

    /// Returns `true` if the type of the value corresponds to `T`.
    #[inline]
    #[doc(alias = "g_variant_is_of_type")]
    pub fn is<T: StaticVariantType>(&self) -> bool {
        unsafe {
            from_glib(ffi::g_variant_is_of_type(
                self.to_glib_none().0,
                T::static_variant_type().to_glib_none().0,
            ))
        }
    }

    /// Returns the classification of the variant.
    #[doc(alias = "g_variant_classify")]
    pub fn classify(&self) -> crate::VariantClass {
        unsafe { from_glib(ffi::g_variant_classify(self.to_glib_none().0)) }
    }

    /// Tries to extract a value of type `T`.
    ///
    /// Returns `Some` if `T` matches the variant's type.
    #[inline]
    pub fn get<T: FromVariant>(&self) -> Option<T> {
        T::from_variant(self)
    }

    /// Tries to extract a value of type `T`.
    pub fn try_get<T: FromVariant>(&self) -> Result<T, VariantTypeMismatchError> {
        self.get().ok_or_else(|| {
            VariantTypeMismatchError::new(
                self.type_().to_owned(),
                T::static_variant_type().into_owned(),
            )
        })
    }

    /// Boxes value.
    #[inline]
    pub fn from_variant(value: &Variant) -> Self {
        unsafe { from_glib_none(ffi::g_variant_new_variant(value.to_glib_none().0)) }
    }

    /// Unboxes self.
    ///
    /// Returns `Some` if self contains a `Variant`.
    #[inline]
    #[doc(alias = "get_variant")]
    pub fn as_variant(&self) -> Option<Variant> {
        unsafe { from_glib_full(ffi::g_variant_get_variant(self.to_glib_none().0)) }
    }

    /// Reads a child item out of a container `Variant` instance.
    ///
    /// # Panics
    ///
    /// * if `self` is not a container type.
    /// * if given `index` is larger than number of children.
    #[doc(alias = "get_child_value")]
    #[doc(alias = "g_variant_get_child_value")]
    pub fn child_value(&self, index: usize) -> Variant {
        assert!(self.is_container());
        assert!(index < self.n_children());

        unsafe { from_glib_full(ffi::g_variant_get_child_value(self.to_glib_none().0, index)) }
    }

    /// Try to read a child item out of a container `Variant` instance.
    ///
    /// It returns `None` if `self` is not a container type or if the given
    /// `index` is larger than number of children.
    pub fn try_child_value(&self, index: usize) -> Option<Variant> {
        if !(self.is_container() && index < self.n_children()) {
            return None;
        }

        let v =
            unsafe { from_glib_full(ffi::g_variant_get_child_value(self.to_glib_none().0, index)) };
        Some(v)
    }

    /// Try to read a child item out of a container `Variant` instance.
    ///
    /// It returns `Ok(None)` if `self` is not a container type or if the given
    /// `index` is larger than number of children.  An error is thrown if the
    /// type does not match.
    pub fn try_child_get<T: StaticVariantType + FromVariant>(
        &self,
        index: usize,
    ) -> Result<Option<T>, VariantTypeMismatchError> {
        // TODO: In the future optimize this by using g_variant_get_child()
        // directly to avoid allocating a GVariant.
        self.try_child_value(index).map(|v| v.try_get()).transpose()
    }

    /// Read a child item out of a container `Variant` instance.
    ///
    /// # Panics
    ///
    /// * if `self` is not a container type.
    /// * if given `index` is larger than number of children.
    /// * if the expected variant type does not match
    pub fn child_get<T: StaticVariantType + FromVariant>(&self, index: usize) -> T {
        // TODO: In the future optimize this by using g_variant_get_child()
        // directly to avoid allocating a GVariant.
        self.child_value(index).get().unwrap()
    }

    /// Tries to extract a `&str`.
    ///
    /// Returns `Some` if the variant has a string type (`s`, `o` or `g` type
    /// strings).
    #[doc(alias = "get_str")]
    #[doc(alias = "g_variant_get_string")]
    pub fn str(&self) -> Option<&str> {
        unsafe {
            match self.type_().as_str() {
                "s" | "o" | "g" => {
                    let mut len = 0;
                    let ptr = ffi::g_variant_get_string(self.to_glib_none().0, &mut len);
                    let ret = str::from_utf8_unchecked(slice::from_raw_parts(
                        ptr as *const u8,
                        len as usize,
                    ));
                    Some(ret)
                }
                _ => None,
            }
        }
    }

    /// Tries to extract a `&[T]` from a variant of array type with a suitable element type.
    ///
    /// Returns an error if the type is wrong.
    #[doc(alias = "g_variant_get_fixed_array")]
    pub fn fixed_array<T: FixedSizeVariantType>(&self) -> Result<&[T], VariantTypeMismatchError> {
        unsafe {
            let expected_ty = T::static_variant_type().as_array();
            if self.type_() != expected_ty {
                return Err(VariantTypeMismatchError {
                    actual: self.type_().to_owned(),
                    expected: expected_ty.into_owned(),
                });
            }

            let mut n_elements = mem::MaybeUninit::uninit();
            let ptr = ffi::g_variant_get_fixed_array(
                self.to_glib_none().0,
                n_elements.as_mut_ptr(),
                mem::size_of::<T>(),
            );
            assert!(!ptr.is_null());

            let n_elements = n_elements.assume_init();
            if n_elements == 0 {
                Ok(&[])
            } else {
                Ok(slice::from_raw_parts(ptr as *const T, n_elements))
            }
        }
    }

    /// Creates a new Variant array from children.
    ///
    /// # Panics
    ///
    /// This function panics if not all variants are of type `T`.
    #[doc(alias = "g_variant_new_array")]
    pub fn array_from_iter<T: StaticVariantType, I: IntoIterator<Item = Variant>>(
        children: I,
    ) -> Self {
        let type_ = T::static_variant_type();

        unsafe {
            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), type_.as_array().to_glib_none().0);
            let mut builder = builder.assume_init();
            for value in children.into_iter() {
                if ffi::g_variant_is_of_type(value.to_glib_none().0, type_.to_glib_none().0)
                    == ffi::GFALSE
                {
                    ffi::g_variant_builder_clear(&mut builder);
                    assert!(value.is::<T>());
                }

                ffi::g_variant_builder_add_value(&mut builder, value.to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }

    /// Creates a new Variant tuple from children.
    #[doc(alias = "g_variant_new_tuple")]
    pub fn tuple_from_iter(children: impl IntoIterator<Item = Variant>) -> Self {
        unsafe {
            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::TUPLE.to_glib_none().0);
            let mut builder = builder.assume_init();
            for value in children.into_iter() {
                ffi::g_variant_builder_add_value(&mut builder, value.to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }

    /// Creates a new maybe Variant.
    #[doc(alias = "g_variant_new_maybe")]
    pub fn from_maybe<T: StaticVariantType>(child: Option<&Variant>) -> Self {
        let type_ = T::static_variant_type();
        let ptr = match child {
            Some(child) => {
                assert_eq!(type_, child.type_());

                child.to_glib_none().0
            }
            None => std::ptr::null(),
        };
        unsafe {
            from_glib_none(ffi::g_variant_new_maybe(
                type_.as_ptr() as *const _,
                ptr as *mut ffi::GVariant,
            ))
        }
    }

    /// Constructs a new serialised-mode GVariant instance.
    #[doc(alias = "g_variant_new_from_bytes")]
    pub fn from_bytes<T: StaticVariantType>(bytes: &Bytes) -> Self {
        Variant::from_bytes_with_type(bytes, &T::static_variant_type())
    }

    /// Constructs a new serialised-mode GVariant instance.
    ///
    /// This is the same as `from_bytes`, except that checks on the passed
    /// data are skipped.
    ///
    /// You should not use this function on data from external sources.
    ///
    /// # Safety
    ///
    /// Since the data is not validated, this is potentially dangerous if called
    /// on bytes which are not guaranteed to have come from serialising another
    /// Variant.  The caller is responsible for ensuring bad data is not passed in.
    pub unsafe fn from_bytes_trusted<T: StaticVariantType>(bytes: &Bytes) -> Self {
        Variant::from_bytes_with_type_trusted(bytes, &T::static_variant_type())
    }

    /// Constructs a new serialised-mode GVariant instance.
    #[doc(alias = "g_variant_new_from_data")]
    pub fn from_data<T: StaticVariantType, A: AsRef<[u8]>>(data: A) -> Self {
        Variant::from_data_with_type(data, &T::static_variant_type())
    }

    /// Constructs a new serialised-mode GVariant instance.
    ///
    /// This is the same as `from_data`, except that checks on the passed
    /// data are skipped.
    ///
    /// You should not use this function on data from external sources.
    ///
    /// # Safety
    ///
    /// Since the data is not validated, this is potentially dangerous if called
    /// on bytes which are not guaranteed to have come from serialising another
    /// Variant.  The caller is responsible for ensuring bad data is not passed in.
    pub unsafe fn from_data_trusted<T: StaticVariantType, A: AsRef<[u8]>>(data: A) -> Self {
        Variant::from_data_with_type_trusted(data, &T::static_variant_type())
    }

    /// Constructs a new serialised-mode GVariant instance with a given type.
    #[doc(alias = "g_variant_new_from_bytes")]
    pub fn from_bytes_with_type(bytes: &Bytes, type_: &VariantTy) -> Self {
        unsafe {
            from_glib_none(ffi::g_variant_new_from_bytes(
                type_.as_ptr() as *const _,
                bytes.to_glib_none().0,
                false.into_glib(),
            ))
        }
    }

    /// Constructs a new serialised-mode GVariant instance with a given type.
    ///
    /// This is the same as `from_bytes`, except that checks on the passed
    /// data are skipped.
    ///
    /// You should not use this function on data from external sources.
    ///
    /// # Safety
    ///
    /// Since the data is not validated, this is potentially dangerous if called
    /// on bytes which are not guaranteed to have come from serialising another
    /// Variant.  The caller is responsible for ensuring bad data is not passed in.
    pub unsafe fn from_bytes_with_type_trusted(bytes: &Bytes, type_: &VariantTy) -> Self {
        from_glib_none(ffi::g_variant_new_from_bytes(
            type_.as_ptr() as *const _,
            bytes.to_glib_none().0,
            true.into_glib(),
        ))
    }

    /// Constructs a new serialised-mode GVariant instance with a given type.
    #[doc(alias = "g_variant_new_from_data")]
    pub fn from_data_with_type<A: AsRef<[u8]>>(data: A, type_: &VariantTy) -> Self {
        unsafe {
            let data = Box::new(data);
            let (data_ptr, len) = {
                let data = (&*data).as_ref();
                (data.as_ptr(), data.len())
            };

            unsafe extern "C" fn free_data<A: AsRef<[u8]>>(ptr: ffi::gpointer) {
                let _ = Box::from_raw(ptr as *mut A);
            }

            from_glib_none(ffi::g_variant_new_from_data(
                type_.as_ptr() as *const _,
                data_ptr as ffi::gconstpointer,
                len,
                false.into_glib(),
                Some(free_data::<A>),
                Box::into_raw(data) as ffi::gpointer,
            ))
        }
    }

    /// Constructs a new serialised-mode GVariant instance with a given type.
    ///
    /// This is the same as `from_data`, except that checks on the passed
    /// data are skipped.
    ///
    /// You should not use this function on data from external sources.
    ///
    /// # Safety
    ///
    /// Since the data is not validated, this is potentially dangerous if called
    /// on bytes which are not guaranteed to have come from serialising another
    /// Variant.  The caller is responsible for ensuring bad data is not passed in.
    pub unsafe fn from_data_with_type_trusted<A: AsRef<[u8]>>(data: A, type_: &VariantTy) -> Self {
        let data = Box::new(data);
        let (data_ptr, len) = {
            let data = (&*data).as_ref();
            (data.as_ptr(), data.len())
        };

        unsafe extern "C" fn free_data<A: AsRef<[u8]>>(ptr: ffi::gpointer) {
            let _ = Box::from_raw(ptr as *mut A);
        }

        from_glib_none(ffi::g_variant_new_from_data(
            type_.as_ptr() as *const _,
            data_ptr as ffi::gconstpointer,
            len,
            true.into_glib(),
            Some(free_data::<A>),
            Box::into_raw(data) as ffi::gpointer,
        ))
    }

    /// Returns the serialised form of a GVariant instance.
    #[doc(alias = "get_data_as_bytes")]
    #[doc(alias = "g_variant_get_data_as_bytes")]
    pub fn data_as_bytes(&self) -> Bytes {
        unsafe { from_glib_full(ffi::g_variant_get_data_as_bytes(self.to_glib_none().0)) }
    }

    /// Returns the serialised form of a GVariant instance.
    #[doc(alias = "g_variant_get_data")]
    pub fn data(&self) -> &[u8] {
        unsafe {
            let selfv = self.to_glib_none();
            let len = ffi::g_variant_get_size(selfv.0);
            let ptr = ffi::g_variant_get_data(selfv.0);
            slice::from_raw_parts(ptr as *const u8, len as usize)
        }
    }

    /// Returns the size of serialised form of a GVariant instance.
    #[doc(alias = "g_variant_get_size")]
    pub fn size(&self) -> usize {
        unsafe { ffi::g_variant_get_size(self.to_glib_none().0) }
    }

    /// Stores the serialised form of a GVariant instance into the given slice.
    ///
    /// The slice needs to be big enough.
    #[doc(alias = "g_variant_store")]
    pub fn store(&self, data: &mut [u8]) -> Result<usize, crate::BoolError> {
        unsafe {
            let size = ffi::g_variant_get_size(self.to_glib_none().0);
            if data.len() < size {
                return Err(bool_error!("Provided slice is too small"));
            }

            ffi::g_variant_store(self.to_glib_none().0, data.as_mut_ptr() as ffi::gpointer);

            Ok(size)
        }
    }

    /// Returns a copy of the variant in normal form.
    #[doc(alias = "g_variant_get_normal_form")]
    pub fn normal_form(&self) -> Self {
        unsafe { from_glib_full(ffi::g_variant_get_normal_form(self.to_glib_none().0)) }
    }

    /// Returns a copy of the variant in the opposite endianness.
    #[doc(alias = "g_variant_byteswap")]
    pub fn byteswap(&self) -> Self {
        unsafe { from_glib_full(ffi::g_variant_byteswap(self.to_glib_none().0)) }
    }

    /// Determines the number of children in a container GVariant instance.
    #[doc(alias = "g_variant_n_children")]
    pub fn n_children(&self) -> usize {
        assert!(self.is_container());

        unsafe { ffi::g_variant_n_children(self.to_glib_none().0) }
    }

    /// Create an iterator over items in the variant.
    ///
    /// Note that this heap allocates a variant for each element,
    /// which can be particularly expensive for large arrays.
    pub fn iter(&self) -> VariantIter {
        assert!(self.is_container());

        VariantIter::new(self.clone())
    }

    /// Create an iterator over borrowed strings from a GVariant of type `as` (array of string).
    ///
    /// This will fail if the variant is not an array of with
    /// the expected child type.
    ///
    /// A benefit of this API over [`Self::iter()`] is that it
    /// minimizes allocation, and provides strongly typed access.
    ///
    /// ```
    /// # use glib::prelude::*;
    /// let strs = &["foo", "bar"];
    /// let strs_variant: glib::Variant = strs.to_variant();
    /// for s in strs_variant.array_iter_str()? {
    ///     println!("{}", s);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn array_iter_str(&self) -> Result<VariantStrIter, VariantTypeMismatchError> {
        let child_ty = String::static_variant_type();
        let actual_ty = self.type_();
        let expected_ty = child_ty.as_array();
        if actual_ty != expected_ty {
            return Err(VariantTypeMismatchError {
                actual: actual_ty.to_owned(),
                expected: expected_ty.into_owned(),
            });
        }

        Ok(VariantStrIter::new(self))
    }

    /// Variant has a container type.
    #[doc(alias = "g_variant_is_container")]
    pub fn is_container(&self) -> bool {
        unsafe { ffi::g_variant_is_container(self.to_glib_none().0) != ffi::GFALSE }
    }
}

unsafe impl Send for Variant {}
unsafe impl Sync for Variant {}

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Variant")
            .field("ptr", &self.to_glib_none().0)
            .field("type", &self.type_())
            .field("value", &self.to_string())
            .finish()
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serialized: GString = unsafe {
            from_glib_full(ffi::g_variant_print(
                self.to_glib_none().0,
                false.into_glib(),
            ))
        };
        f.write_str(&serialized)
    }
}

impl PartialEq for Variant {
    #[doc(alias = "g_variant_equal")]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::g_variant_equal(
                self.to_glib_none().0 as *const _,
                other.to_glib_none().0 as *const _,
            ))
        }
    }
}

impl Eq for Variant {}

impl PartialOrd for Variant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe {
            if ffi::g_variant_classify(self.to_glib_none().0)
                != ffi::g_variant_classify(other.to_glib_none().0)
            {
                return None;
            }

            if self.is_container() {
                return None;
            }

            let res = ffi::g_variant_compare(
                self.to_glib_none().0 as *const _,
                other.to_glib_none().0 as *const _,
            );

            Some(res.cmp(&0))
        }
    }
}

impl Hash for Variant {
    #[doc(alias = "g_variant_hash")]
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { state.write_u32(ffi::g_variant_hash(self.to_glib_none().0 as *const _)) }
    }
}

/// Converts to `Variant`.
pub trait ToVariant {
    /// Returns a `Variant` clone of `self`.
    fn to_variant(&self) -> Variant;
}

/// Extracts a value.
pub trait FromVariant: Sized + StaticVariantType {
    /// Tries to extract a value.
    ///
    /// Returns `Some` if the variant's type matches `Self`.
    fn from_variant(variant: &Variant) -> Option<Self>;
}

/// Returns `VariantType` of `Self`.
pub trait StaticVariantType {
    /// Returns the `VariantType` corresponding to `Self`.
    fn static_variant_type() -> Cow<'static, VariantTy>;
}

impl StaticVariantType for Variant {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::VARIANT)
    }
}

impl<'a, T: ?Sized + ToVariant> ToVariant for &'a T {
    fn to_variant(&self) -> Variant {
        <T as ToVariant>::to_variant(self)
    }
}

impl<'a, T: ?Sized + StaticVariantType> StaticVariantType for &'a T {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <T as StaticVariantType>::static_variant_type()
    }
}

macro_rules! impl_numeric {
    ($name:ty, $typ:expr, $new_fn:ident, $get_fn:ident) => {
        impl StaticVariantType for $name {
            fn static_variant_type() -> Cow<'static, VariantTy> {
                Cow::Borrowed($typ)
            }
        }

        impl ToVariant for $name {
            fn to_variant(&self) -> Variant {
                unsafe { from_glib_none(ffi::$new_fn(*self)) }
            }
        }

        impl FromVariant for $name {
            fn from_variant(variant: &Variant) -> Option<Self> {
                unsafe {
                    if variant.is::<Self>() {
                        Some(ffi::$get_fn(variant.to_glib_none().0))
                    } else {
                        None
                    }
                }
            }
        }
    };
}

impl_numeric!(u8, VariantTy::BYTE, g_variant_new_byte, g_variant_get_byte);
impl_numeric!(
    i16,
    VariantTy::INT16,
    g_variant_new_int16,
    g_variant_get_int16
);
impl_numeric!(
    u16,
    VariantTy::UINT16,
    g_variant_new_uint16,
    g_variant_get_uint16
);
impl_numeric!(
    i32,
    VariantTy::INT32,
    g_variant_new_int32,
    g_variant_get_int32
);
impl_numeric!(
    u32,
    VariantTy::UINT32,
    g_variant_new_uint32,
    g_variant_get_uint32
);
impl_numeric!(
    i64,
    VariantTy::INT64,
    g_variant_new_int64,
    g_variant_get_int64
);
impl_numeric!(
    u64,
    VariantTy::UINT64,
    g_variant_new_uint64,
    g_variant_get_uint64
);
impl_numeric!(
    f64,
    VariantTy::DOUBLE,
    g_variant_new_double,
    g_variant_get_double
);

impl StaticVariantType for () {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::UNIT)
    }
}

impl ToVariant for () {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(ffi::g_variant_new_tuple(ptr::null(), 0)) }
    }
}

impl FromVariant for () {
    fn from_variant(variant: &Variant) -> Option<Self> {
        if variant.is::<Self>() {
            Some(())
        } else {
            None
        }
    }
}

impl StaticVariantType for bool {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::BOOLEAN)
    }
}

impl ToVariant for bool {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(ffi::g_variant_new_boolean(self.into_glib())) }
    }
}

impl FromVariant for bool {
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            if variant.is::<Self>() {
                Some(from_glib(ffi::g_variant_get_boolean(
                    variant.to_glib_none().0,
                )))
            } else {
                None
            }
        }
    }
}

impl StaticVariantType for String {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::STRING)
    }
}

impl ToVariant for String {
    fn to_variant(&self) -> Variant {
        self[..].to_variant()
    }
}

impl FromVariant for String {
    fn from_variant(variant: &Variant) -> Option<Self> {
        variant.str().map(String::from)
    }
}

impl StaticVariantType for str {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        String::static_variant_type()
    }
}

impl ToVariant for str {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(ffi::g_variant_new_take_string(self.to_glib_full())) }
    }
}

impl<T: StaticVariantType> StaticVariantType for Option<T> {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe {
            let ptr = ffi::g_variant_type_new_maybe(T::static_variant_type().to_glib_none().0);
            Cow::Owned(from_glib_full(ptr))
        }
    }
}

impl<T: StaticVariantType + ToVariant> ToVariant for Option<T> {
    fn to_variant(&self) -> Variant {
        Variant::from_maybe::<T>(self.as_ref().map(|m| m.to_variant()).as_ref())
    }
}

impl<T: StaticVariantType + FromVariant> FromVariant for Option<T> {
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            if variant.is::<Self>() {
                let c_child = ffi::g_variant_get_maybe(variant.to_glib_none().0);
                if !c_child.is_null() {
                    let child: Variant = from_glib_full(c_child);

                    Some(T::from_variant(&child))
                } else {
                    Some(None)
                }
            } else {
                None
            }
        }
    }
}

impl<T: StaticVariantType> StaticVariantType for [T] {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        T::static_variant_type().as_array()
    }
}

impl<T: StaticVariantType + ToVariant> ToVariant for [T] {
    fn to_variant(&self) -> Variant {
        unsafe {
            if self.is_empty() {
                return from_glib_none(ffi::g_variant_new_array(
                    T::static_variant_type().to_glib_none().0,
                    ptr::null(),
                    0,
                ));
            }

            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::ARRAY.to_glib_none().0);
            let mut builder = builder.assume_init();
            for value in self {
                let value = value.to_variant();
                ffi::g_variant_builder_add_value(&mut builder, value.to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }
}

impl<T: FromVariant> FromVariant for Vec<T> {
    fn from_variant(variant: &Variant) -> Option<Self> {
        if !variant.is_container() {
            return None;
        }

        let mut vec = Vec::with_capacity(variant.n_children());

        for i in 0..variant.n_children() {
            match variant.child_value(i).get() {
                Some(child) => vec.push(child),
                None => return None,
            }
        }

        Some(vec)
    }
}

impl<T: StaticVariantType + ToVariant> ToVariant for Vec<T> {
    fn to_variant(&self) -> Variant {
        self.as_slice().to_variant()
    }
}

impl<T: StaticVariantType> StaticVariantType for Vec<T> {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <[T]>::static_variant_type()
    }
}

#[test]
fn test_regression_from_variant_panics() {
    let variant = "text".to_variant();
    let hashmap: Option<HashMap<u64, u64>> = FromVariant::from_variant(&variant);
    assert!(hashmap.is_none());

    let variant = HashMap::<u64, u64>::new().to_variant();
    let hashmap: Option<HashMap<u64, u64>> = FromVariant::from_variant(&variant);
    assert!(hashmap.is_some());
}

impl<K, V, H> FromVariant for HashMap<K, V, H>
where
    K: FromVariant + Eq + Hash,
    V: FromVariant,
    H: BuildHasher + Default,
{
    fn from_variant(variant: &Variant) -> Option<Self> {
        if !variant.is_container() {
            return None;
        }

        let mut map = HashMap::default();

        for i in 0..variant.n_children() {
            let entry = variant.child_value(i);
            let key = match entry.child_value(0).get() {
                Some(key) => key,
                None => return None,
            };
            let val = match entry.child_value(1).get() {
                Some(val) => val,
                None => return None,
            };

            map.insert(key, val);
        }

        Some(map)
    }
}

impl<K, V> FromVariant for BTreeMap<K, V>
where
    K: FromVariant + Eq + Ord,
    V: FromVariant,
{
    fn from_variant(variant: &Variant) -> Option<Self> {
        if !variant.is_container() {
            return None;
        }

        let mut map = BTreeMap::default();

        for i in 0..variant.n_children() {
            let entry = variant.child_value(i);
            let key = match entry.child_value(0).get() {
                Some(key) => key,
                None => return None,
            };
            let val = match entry.child_value(1).get() {
                Some(val) => val,
                None => return None,
            };

            map.insert(key, val);
        }

        Some(map)
    }
}

impl<K, V> ToVariant for HashMap<K, V>
where
    K: StaticVariantType + ToVariant + Eq + Hash,
    V: StaticVariantType + ToVariant,
{
    fn to_variant(&self) -> Variant {
        unsafe {
            if self.is_empty() {
                return from_glib_none(ffi::g_variant_new_array(
                    DictEntry::<K, V>::static_variant_type().to_glib_none().0,
                    ptr::null(),
                    0,
                ));
            }

            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::ARRAY.to_glib_none().0);
            let mut builder = builder.assume_init();
            for (key, value) in self {
                let entry = DictEntry::new(key, value).to_variant();
                ffi::g_variant_builder_add_value(&mut builder, entry.to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }
}

impl<K, V> ToVariant for BTreeMap<K, V>
where
    K: StaticVariantType + ToVariant + Eq + Hash,
    V: StaticVariantType + ToVariant,
{
    fn to_variant(&self) -> Variant {
        unsafe {
            if self.is_empty() {
                return from_glib_none(ffi::g_variant_new_array(
                    DictEntry::<K, V>::static_variant_type().to_glib_none().0,
                    ptr::null(),
                    0,
                ));
            }

            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::ARRAY.to_glib_none().0);
            let mut builder = builder.assume_init();
            for (key, value) in self {
                let entry = DictEntry::new(key, value).to_variant();
                ffi::g_variant_builder_add_value(&mut builder, entry.to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }
}

/// A Dictionary entry.
///
/// While GVariant format allows a dictionary entry to be an independent type, typically you'll need
/// to use this in a dictionary, which is simply an array of dictionary entries. The following code
/// creates a dictionary:
///
/// ```
///# use glib::prelude::*; // or `use gtk::prelude::*;`
/// use glib::{Variant, FromVariant, ToVariant};
/// use glib::variant::DictEntry;
///
/// let entries = [
///     DictEntry::new("uuid", 1000u32),
///     DictEntry::new("guid", 1001u32),
/// ];
/// let dict = entries.into_iter().collect::<Variant>();
/// assert_eq!(dict.n_children(), 2);
/// assert_eq!(dict.type_().as_str(), "a{su}");
/// ```
pub struct DictEntry<K, V> {
    key: K,
    value: V,
}

impl<K, V> DictEntry<K, V>
where
    K: StaticVariantType + ToVariant,
    V: StaticVariantType + ToVariant,
{
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }
}

impl<K, V> FromVariant for DictEntry<K, V>
where
    K: FromVariant,
    V: FromVariant,
{
    fn from_variant(variant: &Variant) -> Option<Self> {
        if !variant.type_().is_subtype_of(VariantTy::DICT_ENTRY) {
            return None;
        }

        let key = match variant.child_value(0).get() {
            Some(key) => key,
            None => return None,
        };
        let value = match variant.child_value(1).get() {
            Some(value) => value,
            None => return None,
        };

        Some(Self { key, value })
    }
}

impl<K, V> ToVariant for DictEntry<K, V>
where
    K: StaticVariantType + ToVariant,
    V: StaticVariantType + ToVariant,
{
    fn to_variant(&self) -> Variant {
        unsafe {
            from_glib_none(ffi::g_variant_new_dict_entry(
                self.key.to_variant().to_glib_none().0,
                self.value.to_variant().to_glib_none().0,
            ))
        }
    }
}

impl ToVariant for Variant {
    fn to_variant(&self) -> Variant {
        Variant::from_variant(self)
    }
}

impl FromVariant for Variant {
    fn from_variant(variant: &Variant) -> Option<Self> {
        variant.as_variant()
    }
}

impl<K: StaticVariantType, V: StaticVariantType> StaticVariantType for DictEntry<K, V> {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe {
            let ptr = ffi::g_variant_type_new_dict_entry(
                K::static_variant_type().to_glib_none().0,
                V::static_variant_type().to_glib_none().0,
            );
            Cow::Owned(from_glib_full(ptr))
        }
    }
}

fn static_variant_mapping<K, V>() -> Cow<'static, VariantTy>
where
    K: StaticVariantType,
    V: StaticVariantType,
{
    let key_type = K::static_variant_type();
    let value_type = V::static_variant_type();

    if key_type == VariantTy::STRING && value_type == VariantTy::VARIANT {
        return Cow::Borrowed(VariantTy::VARDICT);
    }

    unsafe {
        let ptr = ffi::g_string_sized_new(16);
        ffi::g_string_append_len(ptr, b"a{".as_ptr() as *const _, 2);
        ffi::g_string_append_len(
            ptr,
            key_type.as_str().as_ptr() as *const _,
            key_type.as_str().len() as isize,
        );
        ffi::g_string_append_len(
            ptr,
            value_type.as_str().as_ptr() as *const _,
            value_type.as_str().len() as isize,
        );
        ffi::g_string_append_c(ptr, b'}' as _);

        Cow::Owned(from_glib_full(
            ffi::g_string_free(ptr, ffi::GFALSE) as *mut ffi::GVariantType
        ))
    }
}

impl<K, V, H> StaticVariantType for HashMap<K, V, H>
where
    K: StaticVariantType,
    V: StaticVariantType,
    H: BuildHasher + Default,
{
    fn static_variant_type() -> Cow<'static, VariantTy> {
        static_variant_mapping::<K, V>()
    }
}

impl<K, V> StaticVariantType for BTreeMap<K, V>
where
    K: StaticVariantType,
    V: StaticVariantType,
{
    fn static_variant_type() -> Cow<'static, VariantTy> {
        static_variant_mapping::<K, V>()
    }
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> StaticVariantType for ($($name,)+)
            where
                $($name: StaticVariantType,)+
            {
                fn static_variant_type() -> Cow<'static, VariantTy> {
                    unsafe {
                        let ptr = ffi::g_string_sized_new(255);
                        ffi::g_string_append_c(ptr, b'(' as _);
                        $(
                            let t = $name::static_variant_type();
                            ffi::g_string_append_len(ptr, t.as_str().as_ptr() as *const _, t.as_str().len() as isize);
                        )+
                        ffi::g_string_append_c(ptr, b')' as _);

                        Cow::Owned(from_glib_full(ffi::g_string_free(ptr, ffi::GFALSE) as *mut ffi::GVariantType))
                    }
                }
            }

            impl<$($name),+> FromVariant for ($($name,)+)
            where
                $($name: FromVariant,)+
            {
                fn from_variant(variant: &Variant) -> Option<Self> {
                    if !variant.type_().is_subtype_of(VariantTy::TUPLE) {
                        return None;
                    }

                    Some((
                        $(
                            match variant.try_child_get::<$name>($n) {
                                Ok(Some(field)) => field,
                                _ => return None,
                            },
                        )+
                    ))
                }
            }

            impl<$($name),+> ToVariant for ($($name,)+)
            where
                $($name: ToVariant,)+
            {
                fn to_variant(&self) -> Variant {
                    unsafe {
                        let mut builder = mem::MaybeUninit::uninit();
                        ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::TUPLE.to_glib_none().0);
                        let mut builder = builder.assume_init();

                        $(
                            let field = self.$n.to_variant();
                            ffi::g_variant_builder_add_value(&mut builder, field.to_glib_none().0);
                        )+

                        from_glib_none(ffi::g_variant_builder_end(&mut builder))
                    }
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

impl<T: ToVariant + StaticVariantType> FromIterator<T> for Variant {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Variant::array_from_iter::<T, _>(iter.into_iter().map(|v| v.to_variant()))
    }
}

pub unsafe trait FixedSizeVariantType: StaticVariantType + Sized {}
unsafe impl FixedSizeVariantType for u8 {}
unsafe impl FixedSizeVariantType for i16 {}
unsafe impl FixedSizeVariantType for u16 {}
unsafe impl FixedSizeVariantType for i32 {}
unsafe impl FixedSizeVariantType for u32 {}
unsafe impl FixedSizeVariantType for i64 {}
unsafe impl FixedSizeVariantType for u64 {}
unsafe impl FixedSizeVariantType for f64 {}
unsafe impl FixedSizeVariantType for bool {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    macro_rules! unsigned {
        ($name:ident, $ty:ident) => {
            #[test]
            fn $name() {
                let mut n = $ty::max_value();
                while n > 0 {
                    let v = n.to_variant();
                    assert_eq!(v.get(), Some(n));
                    n /= 2;
                }
            }
        };
    }

    macro_rules! signed {
        ($name:ident, $ty:ident) => {
            #[test]
            fn $name() {
                let mut n = $ty::max_value();
                while n > 0 {
                    let v = n.to_variant();
                    assert_eq!(v.get(), Some(n));
                    let v = (-n).to_variant();
                    assert_eq!(v.get(), Some(-n));
                    n /= 2;
                }
            }
        };
    }

    unsigned!(test_u8, u8);
    unsigned!(test_u16, u16);
    unsigned!(test_u32, u32);
    unsigned!(test_u64, u64);
    signed!(test_i16, i16);
    signed!(test_i32, i32);
    signed!(test_i64, i64);

    #[test]
    fn test_str() {
        let s = "this is a test";
        let v = s.to_variant();
        assert_eq!(v.str(), Some(s));
        assert_eq!(42u32.to_variant().str(), None);
    }

    #[test]
    fn test_fixed_array() {
        let b = b"this is a test";
        let v = b.to_variant();
        assert_eq!(v.fixed_array::<u8>().unwrap(), b);
        assert!(42u32.to_variant().fixed_array::<u8>().is_err());

        let b = [1u32, 10u32, 100u32];
        let v = b.to_variant();
        assert_eq!(v.fixed_array::<u32>().unwrap(), b);
        assert!(v.fixed_array::<u8>().is_err());

        let b = [true, false, true];
        let v = b.to_variant();
        assert_eq!(v.fixed_array::<bool>().unwrap(), b);
        assert!(v.fixed_array::<u8>().is_err());

        let b = [1.0f64, 2.0f64, 3.0f64];
        let v = b.to_variant();
        assert_eq!(v.fixed_array::<f64>().unwrap(), b);
        assert!(v.fixed_array::<u64>().is_err());
    }

    #[test]
    fn test_string() {
        let s = String::from("this is a test");
        let v = s.to_variant();
        assert_eq!(v.get(), Some(s));
        assert_eq!(v.normal_form(), v);
    }

    #[test]
    fn test_eq() {
        let v1 = "this is a test".to_variant();
        let v2 = "this is a test".to_variant();
        let v3 = "test".to_variant();
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }

    #[test]
    fn test_hash() {
        let v1 = "this is a test".to_variant();
        let v2 = "this is a test".to_variant();
        let v3 = "test".to_variant();
        let mut set = HashSet::new();
        set.insert(v1);
        assert!(set.contains(&v2));
        assert!(!set.contains(&v3));

        assert_eq!(
            <HashMap<&str, (&str, u8, u32)>>::static_variant_type().as_str(),
            "a{s(syu)}"
        );
    }

    #[test]
    fn test_array() {
        assert_eq!(<Vec<&str>>::static_variant_type().as_str(), "as");
        assert_eq!(
            <Vec<(&str, u8, u32)>>::static_variant_type().as_str(),
            "a(syu)"
        );
        let a = ["foo", "bar", "baz"].to_variant();
        assert_eq!(a.normal_form(), a);
        assert_eq!(a.array_iter_str().unwrap().len(), 3);
        let o = 0u32.to_variant();
        assert!(o.array_iter_str().is_err());
    }

    #[test]
    fn test_array_from_iter() {
        let a = Variant::array_from_iter::<String, _>(
            ["foo", "bar", "baz"].into_iter().map(|s| s.to_variant()),
        );
        assert_eq!(a.type_().as_str(), "as");
        assert_eq!(a.n_children(), 3);

        assert_eq!(a.try_child_get::<String>(0), Ok(Some(String::from("foo"))));
        assert_eq!(a.try_child_get::<String>(1), Ok(Some(String::from("bar"))));
        assert_eq!(a.try_child_get::<String>(2), Ok(Some(String::from("baz"))));
    }

    #[test]
    fn test_array_collect() {
        let a = ["foo", "bar", "baz"].into_iter().collect::<Variant>();
        assert_eq!(a.type_().as_str(), "as");
        assert_eq!(a.n_children(), 3);

        assert_eq!(a.try_child_get::<String>(0), Ok(Some(String::from("foo"))));
        assert_eq!(a.try_child_get::<String>(1), Ok(Some(String::from("bar"))));
        assert_eq!(a.try_child_get::<String>(2), Ok(Some(String::from("baz"))));
    }

    #[test]
    fn test_tuple() {
        assert_eq!(<(&str, u32)>::static_variant_type().as_str(), "(su)");
        assert_eq!(<(&str, u8, u32)>::static_variant_type().as_str(), "(syu)");
        let a = ("test", 1u8, 2u32).to_variant();
        assert_eq!(a.normal_form(), a);
        assert_eq!(a.try_child_get::<String>(0), Ok(Some(String::from("test"))));
        assert_eq!(a.try_child_get::<u8>(1), Ok(Some(1u8)));
        assert_eq!(a.try_child_get::<u32>(2), Ok(Some(2u32)));
        assert_eq!(
            a.try_get::<(String, u8, u32)>(),
            Ok((String::from("test"), 1u8, 2u32))
        );
    }

    #[test]
    fn test_tuple_from_iter() {
        let a = Variant::tuple_from_iter(["foo".to_variant(), 1u8.to_variant(), 2i32.to_variant()]);
        assert_eq!(a.type_().as_str(), "(syi)");
        assert_eq!(a.n_children(), 3);

        assert_eq!(a.try_child_get::<String>(0), Ok(Some(String::from("foo"))));
        assert_eq!(a.try_child_get::<u8>(1), Ok(Some(1u8)));
        assert_eq!(a.try_child_get::<i32>(2), Ok(Some(2i32)));
    }

    #[test]
    fn test_empty() {
        assert_eq!(<()>::static_variant_type().as_str(), "()");
        let a = ().to_variant();
        assert_eq!(a.type_().as_str(), "()");
        assert_eq!(a.get::<()>(), Some(()));
    }

    #[test]
    fn test_btreemap() {
        assert_eq!(
            <BTreeMap<String, u32>>::static_variant_type().as_str(),
            "a{su}"
        );
        // Validate that BTreeMap adds entries to dict in sorted order
        let mut m = BTreeMap::new();
        let total = 20;
        for n in 0..total {
            let k = format!("v{:04}", n);
            m.insert(k, n as u32);
        }
        let v = m.to_variant();
        let n = v.n_children();
        assert_eq!(total, n);
        for n in 0..total {
            let child = v
                .try_child_get::<DictEntry<String, u32>>(n)
                .unwrap()
                .unwrap();
            assert_eq!(*child.value(), n as u32);
        }

        assert_eq!(BTreeMap::from_variant(&v).unwrap(), m);
    }

    #[test]
    fn test_get() -> Result<(), Box<dyn std::error::Error>> {
        let u = 42u32.to_variant();
        assert!(u.get::<i32>().is_none());
        assert_eq!(u.get::<u32>().unwrap(), 42);
        assert!(u.try_get::<i32>().is_err());
        // Test ? conversion
        assert_eq!(u.try_get::<u32>()?, 42);
        Ok(())
    }

    #[test]
    fn test_byteswap() {
        let u = 42u32.to_variant();
        assert_eq!(u.byteswap().get::<u32>().unwrap(), 704643072u32);
        assert_eq!(u.byteswap().byteswap().get::<u32>().unwrap(), 42u32);
    }

    #[test]
    fn test_try_child() {
        let a = ["foo"].to_variant();
        assert!(a.try_child_value(0).is_some());
        assert_eq!(a.try_child_get::<String>(0).unwrap().unwrap(), "foo");
        assert_eq!(a.child_get::<String>(0), "foo");
        assert!(a.try_child_get::<u32>(0).is_err());
        assert!(a.try_child_value(1).is_none());
        assert!(a.try_child_get::<String>(1).unwrap().is_none());
        let u = 42u32.to_variant();
        assert!(u.try_child_value(0).is_none());
        assert!(u.try_child_get::<String>(0).unwrap().is_none());
    }

    #[test]
    fn test_serialize() {
        let a = ("test", 1u8, 2u32).to_variant();

        let bytes = a.data_as_bytes();
        let data = a.data();
        let len = a.size();
        assert_eq!(bytes.len(), len);
        assert_eq!(data.len(), len);

        let mut store_data = vec![0u8; len];
        assert_eq!(a.store(&mut store_data).unwrap(), len);

        assert_eq!(&bytes, data);
        assert_eq!(&store_data, data);

        let b = Variant::from_data::<(String, u8, u32), _>(store_data);
        assert_eq!(a, b);

        let c = Variant::from_bytes::<(String, u8, u32)>(&bytes);
        assert_eq!(a, c);
    }
}
