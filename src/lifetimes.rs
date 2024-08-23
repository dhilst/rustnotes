// 
#![allow(unused_labels)]

// Lifetimes induces an ordering (poset?) relation when used as types
// <'t: 'o> means 't lives more than 'o. This induces an ordering
// And this how the lifetimes are passed and compared and used.
fn foo<'a, 'b: 'a, 'c: 'b>(a: &'a i32, b: &'b i32, c: &'c i32) {
    //  'c : 'b : 'a
    //  'c >= 'b >= 'a
    if *a > 0 {
        foo(&c, &b, &a);
    }
}

fn foo2<'a, 'b: 'a>(a: &'a i32, b: &'b i32) {
    if *a > 0 {
        foo2::<'a, 'a>(b, a); // 1) okay! they use the smaller lifetime
        foo2::<'b, 'a>(b, a); // 2) lifetime may not live long enough consider adding the following bound: `'a: 'b`
        foo2::<'b, 'b>(b, a); // 3) okay!? does this makes a dangling?
        foo2(b, a);           // 4) okay, but what is happening?
    }
    // https://stackoverflow.com/a/69982401/652528
}


// There is a notion of subtyping too
fn bar<'a, 'b: 'a, 'c: 'a>() {
    //  'c : 'a  & 'b : 'a
    let _b: &'b i32 = &0; 
    let _c: &'c i32 = &0; // _b fails here, life-times 'b and 'c are not related 
    let _a: &'a i32 = &_b; // accepts _b and _c since 'a < 'b & 'a < c
}

// Lifetimes are bottom limits, they enforce that 
// variables are assigned to values that live at last
// as much as their lifetime. The value may live more
// (like 'static for example).

fn tar<'a, 'b>(_x: &'a i32, _y: &'b i32) 
    // where 'b: 'a
{
    let _b: &'static i32 = &0; 
    let _a: &'a i32 = _y; // uncomment the where to fix
    // ^ lifetime may not live long enough consider adding the following bound: `'b: 'a`

    // Adding 'b: 'a bound communicates to the
    // borrow checker that 'b lives more than 'a
    // and then the assignment will be okay
}
