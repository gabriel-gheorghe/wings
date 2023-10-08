#[allow(unused_macros)]

#[macro_export]
macro_rules! val {
    ($x:literal px) => { Val::Px($x) };
    ($x:literal %) => { Val::Percent($x) }
}