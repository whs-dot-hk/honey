use genco::fmt;
use genco::prelude::*;
use honey::hive::*;

fn template(name: &str) -> nix::Tokens {
    let group = &nix::inherit("inputs.cells", "group");

    let mut tokens = nix::Tokens::new();
    tokens.append("{");
    tokens.indent();
    for n in 0..2 {
        let machine_name = &format!("machine{:02}", n);
        quote_in! { tokens =>
            $machine_name = {
                imports = [
                    $group.$name.$machine_name
                ];
            };
        }
        tokens.push();
    }
    tokens
}

fn main() -> anyhow::Result<()> {
    let nixos_configurations =
        NixosConfigurations::new1("machine", 2, None, "nixos-23-05", "aarch64-linux");

    let tokens = quote!($nixos_configurations);

    let disko_configurations = template("diskoConfigurations");
    let hardware_profiles = template("hardwareProfiles");
    let nixos_modules = template("nixosModules");
    let nixos_profiles = template("nixosProfiles");

    let tokens2 = quote!($disko_configurations);
    let tokens3 = quote!($hardware_profiles);
    let tokens4 = quote!($nixos_modules);
    let tokens5 = quote!($nixos_profiles);

    let stdout = std::io::stdout();
    let mut w = fmt::IoWriter::new(stdout.lock());

    let fmt = fmt::Config::from_lang::<Nix>();
    let config = nix::Config::default();

    tokens.format_file(&mut w.as_formatter(&fmt), &config)?;
    tokens2.format_file(&mut w.as_formatter(&fmt), &config)?;
    tokens3.format_file(&mut w.as_formatter(&fmt), &config)?;
    tokens4.format_file(&mut w.as_formatter(&fmt), &config)?;
    tokens5.format_file(&mut w.as_formatter(&fmt), &config)?;
    Ok(())
}
