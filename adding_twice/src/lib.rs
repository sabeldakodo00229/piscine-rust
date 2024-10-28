
pub fn add_curry(value: i32) -> impl Fn(i32) -> i32 {
    move |x| x + value
}



pub fn twice<F>(f: F) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
{
    move |x| f(f(x))
}
