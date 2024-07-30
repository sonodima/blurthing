use anyhow::Result;
use pico_args::Arguments;

pub struct BundleArgs {
    pub target: Option<String>,
}

impl BundleArgs {
    pub fn parse(args: &mut Arguments) -> Result<Self> {
        let instance = Self {
            target: args.opt_value_from_str("--target")?,
        };

        Ok(instance)
    }
}
