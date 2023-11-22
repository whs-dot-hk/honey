use genco::fmt;
use genco::prelude::*;
use honey::hive::*;

fn template(prefix: &str, name: &str, number: u32) -> nix::Tokens {
    let group = &nix::inherit("inputs.cells", "group");

    let mut tokens = nix::Tokens::new();
    tokens.append("{");
    tokens.indent();
    for n in 0..number {
        let machine_name = &format!("{}{:02}", prefix, n);
        quote_in! { tokens =>
            $machine_name = {
                imports = [
                    $group.$name.myGroup
                ];
            };
        }
        tokens.push();
    }
    tokens.unindent();
    tokens.append("}");
    tokens
}

fn main() -> anyhow::Result<()> {
    let number = 2;
    let prefix = "machine";

    let nixos_configurations =
        NixosConfigurations::new1(prefix, number, None, "nixos-23-05", "aarch64-linux");

    let tokens = quote!($nixos_configurations);

    let disko_configurations = template(prefix, "diskoConfigurations", number);
    let hardware_profiles = template(prefix, "hardwareProfiles", number);
    let nixos_modules = template(prefix, "nixosModules", number);
    let nixos_profiles = template(prefix, "nixosProfiles", number);

    let tokens2 = quote!($disko_configurations);
    let tokens3 = quote!($hardware_profiles);
    let tokens4 = quote!($nixos_modules);
    let tokens5 = quote!($nixos_profiles);

    let stdout = std::io::stdout();
    let mut w = fmt::IoWriter::new(stdout.lock());

    let fmt = fmt::Config::from_lang::<Nix>();
    let config = nix::Config::new();

    tokens.format_file(&mut w.as_formatter(&fmt), &config)?;
    tokens2.format_file(&mut w.as_formatter(&fmt), &config)?;
    tokens3.format_file(&mut w.as_formatter(&fmt), &config)?;
    tokens4.format_file(&mut w.as_formatter(&fmt), &config)?;
    tokens5.format_file(&mut w.as_formatter(&fmt), &config)?;
    Ok(())
}
