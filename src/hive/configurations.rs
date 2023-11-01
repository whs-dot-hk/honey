use crate::hive::*;
use genco::prelude::*;

pub enum ConfigurationType {
    Import(Import),
}

pub struct Configurations {
    pub imports: Imports,
    /// Name do **not** included in quote.
    pub name: String,
}

impl Configurations {
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let configurations = Configurations::new("dummy", vec![
    ///     ConfigurationType::Import(Import::cell_disko_configurations("my-disko-configurations")),
    ///     ConfigurationType::Import(Import::disko_module()),
    /// ]);
    ///
    /// let toks = quote!($configurations);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
    ///         "    inherit (inputs) disko;",
    ///         "in",
    ///         "",
    ///         "{",
    ///         "    imports = [",
    ///         "        cell.diskoConfigurations.my-disko-configurations",
    ///         "        disko.nixosModules.disko",
    ///         "    ];",
    ///         "}"
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn new(name: &str, configurations: Vec<ConfigurationType>) -> Self {
        let mut imports = Vec::new();
        for c in configurations {
            match c {
                ConfigurationType::Import(import) => imports.push(import),
            }
        }
        Self {
            imports: Imports(imports),
            name: String::from(name),
        }
    }

    pub fn new_nixos_configurations(name: &str) -> Self {
        Self {
            imports: Imports(vec![
                Import::cell_disko_configurations(name),
                Import::cell_hardware_profiles(name),
                Import::cell_home_configurations(name),
                Import::cell_nixos_modules(name),
                Import::cell_nixos_profiles(name),
                Import::disko_module(),
            ]),
            name: String::from(name),
        }
    }
}

impl FormatInto<Nix> for Configurations {
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let configurations = Configurations {
    ///     imports: Imports(vec![
    ///         Import::cell_disko_configurations("my-disko-configurations"),
    ///         Import::disko_module(),
    ///     ]),
    ///     name: String::from("dummy"),
    /// };
    ///
    /// let toks = quote!($configurations);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
    ///         "    inherit (inputs) disko;",
    ///         "in",
    ///         "",
    ///         "{",
    ///         "    imports = [",
    ///         "        cell.diskoConfigurations.my-disko-configurations",
    ///         "        disko.nixosModules.disko",
    ///         "    ];",
    ///         "}"
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    fn format_into(self, tokens: &mut Tokens<Nix>) {
        quote_in! { *tokens =>
            {
                imports = $(self.imports);
            }
        }
    }
}

pub struct NixosConfigurations(pub Vec<Configurations>);

impl FormatInto<Nix> for NixosConfigurations {
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let nixos_configurations = NixosConfigurations(vec![
    ///     Configurations::new_nixos_configurations("machine1"),
    ///     Configurations::new_nixos_configurations("machine2"),
    /// ]);
    ///
    /// let toks = quote!($nixos_configurations);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
    ///         "    inherit (inputs) disko;",
    ///         "in",
    ///         "",
    ///         "{",
    ///         "    machine1 = {",
    ///         "        imports = [",
    ///         "            cell.diskoConfigurations.machine1",
    ///         "            cell.hardwareProfiles.machine1",
    ///         "            cell.homeConfigurations.machine1",
    ///         "            cell.nixosModules.machine1",
    ///         "            cell.nixosProfiles.machine1",
    ///         "            disko.nixosModules.disko",
    ///         "        ];",
    ///         "    };",
    ///         "    machine2 = {",
    ///         "        imports = [",
    ///         "            cell.diskoConfigurations.machine2",
    ///         "            cell.hardwareProfiles.machine2",
    ///         "            cell.homeConfigurations.machine2",
    ///         "            cell.nixosModules.machine2",
    ///         "            cell.nixosProfiles.machine2",
    ///         "            disko.nixosModules.disko",
    ///         "        ];",
    ///         "    };",
    ///         "}",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    fn format_into(self, tokens: &mut Tokens<Nix>) {
        tokens.append("{");
        tokens.indent();
        for configurations in self.0 {
            quote_in!(*tokens => $(configurations.name.clone()) = $configurations;);
            tokens.push();
        }
        tokens.unindent();
        tokens.append("}");
    }
}
