use bash_builtins::{Args, Builtin, Error, Result, builtin_metadata};

builtin_metadata!(name = "gecho", create = RGeckoBuiltin::new);

struct RGeckoBuiltin;

impl RGeckoBuiltin {
    fn new() -> RGeckoBuiltin {
        RGeckoBuiltin {}
    }
}

impl Builtin for RGeckoBuiltin {
    fn call(&mut self, args: &mut Args) -> Result<()> {
        let mut stdout = std::io::stdout();
        let argv: Vec<&str> = match args.string_arguments().collect() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("rgecko: invalid UTF-8 argument: {e}");
                return Err(Error::ExitCode(1));
            }
        };

        rgecko::markup_write(&argv, &mut stdout);

        args.finished()?;

        Ok(())
    }
}
