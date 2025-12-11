use syn::Meta;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrapper {
    /// Inner box
    ///
    /// repeated => not allowed / nop
    /// optional => Option<Box<T>>
    /// oneof variant => Enum::Var(Box<T>)
    /// map => ?
    Box,
    /// Inner Arc
    ///
    /// repeated => Vec<Arc<T>>
    /// optional => Option<Arc<T>>
    /// oneof variant => Enum::Var(Arc<T>)
    /// map => HashMap<Key, Arc<T>>
    Arc,
    // TODO ?
    // Outer Arc
    //
    // repeated => Arc<Vec<T>>
    // optional => Arc<Option<T>>
    // oneof variant => not allowed
    // oneof => Option<Arc<Enum>> / Arc<Option<Enum>> ?
    // map => Arc<HashMap<Key, T>>
}

pub fn wrapper_attr(attr: &Meta) -> Option<Wrapper> {
    if let Meta::Path(path) = attr {
        if path.is_ident("boxed") || path.is_ident("box") {
            Some(Wrapper::Box)
        } else if path.is_ident("arc") {
            Some(Wrapper::Arc)
        } else {
            None
        }
    } else {
        None
    }
}
