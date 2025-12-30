use std::marker;

#[repr(C)]
struct Joined<T, U>(T, U);

#[repr(C)]
struct VBOffset<T>(usize, std::marker::PhantomData<T>);

// The main Class trait
trait IsDyn: Class {
    type VT;
}

trait Class: Sized {
    type VBT = VBOffset<Self>;
    type NonVirtualPart;
    type VirtualPart;
}

trait Derive {
    type Derived: Class;
}

struct Derived<T, U>(marker::PhantomData<(T, U)>);
impl<T: Class, U: Class> Class for Derived<T, U> {
    type NonVirtualPart = Joined<U::NonVirtualPart, T::NonVirtualPart>;
    type VirtualPart = Joined<U::VirtualPart, T::VirtualPart>;
}

/*
struct DerivedClass<T: Class, From: Class>(<Self as Class>::NonVirtualPart, <Self as Class>::VirtualPart);
struct VirtuallyDerivedClass<T: Class, From: Class>(<Self as Class>::NonVirtualPart, <Self as Class>::VirtualPart);

struct DynDerivedClass<T: Class, From: IsDyn>(<Self as Class>::NonVirtualPart, <Self as Class>::VirtualPart);
struct DynVirtuallyDerivedClass<T: Class, From: IsDyn>(<Self as Class>::NonVirtualPart, <Self as Class>::VirtualPart);

impl <T: Class, U: IsDyn> Class for DynDerivedClass<T, U> {
    type VBT = Joined<T::VBT, U::VBT>;
    type NonVirtualPart = Joined<<Self as IsDyn>::VT, Joined<T::NonVirtualPart, U::NonVirtualPart>>;
    type VirtualPart = Joined<T::VirtualPart, U::VirtualPart>;
}

impl <T: Class, U: IsDyn> IsDyn for DynDerivedClass<T, U> {
    type VT = U::VT;
}


impl <T: Class, U: IsDyn> Class for DynVirtuallyDerivedClass<T, U> {
    type VBT = Joined<T::VBT, U::VBT>;
    type NonVirtualPart = Joined<<Self as IsDyn>::VT, Joined<Self::VBT, Joined<T::NonVirtualPart, U::NonVirtualPart>>>;
    type VirtualPart = Joined<T::VirtualPart, U::VirtualPart>;
}

impl <T: Class, U: IsDyn> IsDyn for DynVirtuallyDerivedClass<T, U> {
    type VT = U::VT;
}

impl <T: Class, U: Class> Class for DerivedClass<T, U> {
    type VBT = Joined<T::VBT, U::VBT>;
    type NonVirtualPart = Joined<T::NonVirtualPart, U::NonVirtualPart>;
    type VirtualPart = Joined<T::VirtualPart, U::VirtualPart>;
}


impl <T: Class, U: Class> Class for VirtuallyDerivedClass<T, U> {
    type VBT = Joined<T::VBT, U::VBT>;
    type NonVirtualPart = Joined<Self::VBT, Joined<T::NonVirtualPart, U::NonVirtualPart>>;
    type VirtualPart = Joined<T::VirtualPart, U::VirtualPart>;
}

 */