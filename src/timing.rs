#[macro_export]
macro_rules! measured {
    ($expr:expr) => {{
        let start = ::std::time::Instant::now();
        let value = $expr;
        let elapsed = start.elapsed();
        println!(" -> solution ran in {elapsed:?}");
        value
    }};
}
