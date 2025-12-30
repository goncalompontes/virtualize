// A #[repr(C)] struct composed of two other #[repr(C)] structs
// Should be read as "`T` derived from `From`"
#[repr(C)]
struct Derive<T, From>(T, From);

// A #[repr(C)] struct composed of a #[repr(C)] struct and a virtually inherited struct
// This is a special case to deal with the diamond problem
// The solution MSVC uses is to use a *VirtualBase
#[repr(C)]
struct DeriveVirtual<T: Repr, From: Repr>(T, From);

//impl<T: VBase, U: VBase> VBase for Compose<T, U> {}

// Composed structs are also #[repr(C)] structs
impl <T: Repr, From: Repr> Repr for Derive<T, From> {}



// Marker trait for the #[repr(C)] struct
trait Repr {}

// Marker trait for the virtual inheritance struct
trait VirtualBaseTable {}

trait VirtualRepr: Repr {
    type VBT: VirtualBaseTable;
}


// The main trait for MSVC C++ compliant classes
trait Class {
    // The members of the class as a #[repr(C)] struct
    type Repr: Repr;
}