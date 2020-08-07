/// a copy of `vec!` from the standard library
// macro_export is basically `pub`, but for macros
#[macro_export]
macro_rules! avec {
    // double {{}} because we return a _block_, so therefore we are returning the contents of the
    // inner block, the outer block is macro scope
    ($($element:expr),*) => {{
        // proof that count is known at COMPILETIME
        // (only things known at compile-time can be assigned to a const)
        const ELEM_COUNT: usize = $crate::count![$($element),*];
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(ELEM_COUNT);
        // rust figures out how many times to repeat the following snippet based on the variables
        // used inside the snippet itself
        $(vs.push($element);)*
        vs
    }};
    // this rule just allows for trailing commas as well
    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let count = $count; // only evaulate the user code once, or bad things could happen
        let mut vs = Vec::with_capacity(count);
        vs.resize(count, $element);
        vs
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    // (counting things in a macro is a surprisingly tricky task)
    // there are a **few ways** to pull off this 'counting trick', but this is the reccommended one
    // because it will work for arrays of any length, and won't risk crashing the compiler
    // (e.g. a crazy one that substitutes each element to a '1' then results in a massive
    // expression that adds them all up)
    //
    // the COUNT helper down here will give an expression that returns the length of the input,
    // by subsitiuting each element with Unit ()
    //
    // this will result in an expression that can be resolved at compile-time, so it should be very
    // efficient and, even in such a case where this could not be resolved at compile-time, Unit
    // takes up 0 size on the stack, so this would have almost no memory overhead
    ($($element:expr),*) => {
        // $element won't be called multiple times, because it gets subsituted to unit
        //
        // "call the implementation of len() for slices of unit for the given array we have
        // generated"
        // len is const for slices, so this will compile down to a pre-computed constant value, so
        // there is no runtime overhead, it will all occur at compile-time, the result is just a
        // number.
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    // @SUBST is our own custom syntax, we've just written it like this to show that it is clearly
    // for internal use, and you should not try to to call this from outside the macro
    //
    // substitute an element with Unit
    // the great thing about Unit is that it takes 0 space on the stack!
    (@SUBST; $_element:expr) => { () };
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn trail_comma() {
    let x: Vec<u32> = avec![1, 2, 3,];
    assert_eq!(x.len(), 3);
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert_eq!(x.len(), 1);
}

#[test]
fn repeated() {
    let x: Vec<u32> = avec![42; 100];
    assert_eq!(x.len(), 100);
    assert_eq!(x[30], 42);
}

// not sure why this is showing an error on import?
use std::collections::HashMap;

/// used to create `HashMap` in very little code
///
/// if a key is defined more than once, the last used key will be the one used in the dictionary,
/// the other will be overridden
#[macro_export]
macro_rules! dict {
    ($($key:expr => $val:expr),*) => {{
        const ELEM_COUNT: usize = $crate::count![$($key),*];
        #[allow(unused_mut)]
        let mut hm = HashMap::with_capacity(ELEM_COUNT);
        $(hm.insert($key, $val);)*
        hm
    }};
    ($($key:expr => $val:expr,)*) => {{
        $crate::dict![$($key => $val),*]
    }};
}

#[test]
fn single_hashmap() {
    // macros can use square brackets, curly brackets or normal brackets - it literally does not
    // matter.
    let hm = dict! {
        "John" => "Sailor"
    };
    assert_eq!(hm.len(), 1);
    assert_eq!(hm.get("John").unwrap(), &"Sailor");
}

#[test]
fn empty_hashmap() {
    let hm: HashMap<u32, u32> = dict! {};
    assert_eq!(hm.len(), 0);
}

#[test]
fn many_hashmap() {
    let hm = dict! {
        "John" => "Sailor",
        "Peter" => "Baker",
        "Sally" => "Royal Chef",
        "Ben" => "Programmer",
    };
    assert_eq!(hm.len(), 4);
    assert_eq!(hm.get("John").unwrap(), &"Sailor");
    assert_eq!(hm.get("Peter").unwrap(), &"Baker");
}
