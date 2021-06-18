pub fn setup() {
    std::panic::set_hook(Box::new(move |info| {
        let backtrace = backtrace::Backtrace::new();
        use std::io::Write as _;

        let msg = info
            .payload()
            .downcast_ref::<&'static str>()
            .copied()
            .or_else(|| info.payload().downcast_ref().map(|s: &String| s.as_str()))
            .unwrap_or("Box<Any>");

        let mut fi = std::fs::OpenOptions::new()
            .create(true)
            .append(false)
            .truncate(true)
            .write(true)
            .open("panics.log")
            .expect("open panics.log for writing");

        match info.location() {
            Some(loc) => {
                writeln!(&mut fi, "{}:{}: {}", loc.file(), loc.line(), msg,)
            }
            None => {
                writeln!(&mut fi, "{}", msg)
            }
        }
        .unwrap();

        writeln!(&mut fi).unwrap();
        writeln!(&mut fi, "{:?}", backtrace).unwrap();
    }));
}
