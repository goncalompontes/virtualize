
#[repr(C)]
struct Compose<T, U>(T, U);

impl<T: Repr, U: Repr> Repr for Compose<T, U> {}

#[repr(C)]
struct DerivedVirtual<T: Repr, U: Repr>(Compose<T::VBT, U::VBT>, Compose<T, U>);
impl<T: Repr, U: Repr> Repr for DerivedVirtual<T, U> {
    type VBT = Compose<T::VBT, U::VBT>;
}

struct BaseOffset<T: Repr> {
    offset: usize,
    _marker: std::marker::PhantomData<T>
}
impl <T: Repr> VirtualBaseTable for BaseOffset<T> {}

impl <T: VirtualBaseTable, U: VirtualBaseTable> VirtualBaseTable for Compose<T, U> {}

// Marker trait for the virtual base table
trait VirtualBaseTable {}

// Members of a class
// Its exterior and *seeming* representation
trait Repr: Sized {
    type VBT: VirtualBaseTable = BaseOffset<Self>;
}