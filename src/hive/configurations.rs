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
    ///     ConfigurationType::Import(Import::disko_module()),
    ///     ConfigurationType::Import(Import::disko_configurations("my-disko-configurations")),
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
    ///         "        disko.nixosModules.disko",
    ///         "        cell.diskoConfigurations.my-disko-configurations",
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
                Import::disko_module(),
                Import::disko_configurations(name),
                Import::hardware_profiles(name),
                Import::home_configurations(name),
                Import::nixos_profiles(name),
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
    ///         Import::disko_module(),
    ///         Import::disko_configurations("my-disko-configurations"),
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
    ///         "        disko.nixosModules.disko",
    ///         "        cell.diskoConfigurations.my-disko-configurations",
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
    ///         "            disko.nixosModules.disko",
    ///         "            cell.diskoConfigurations.machine1",
    ///         "            cell.hardwareProfiles.machine1",
    ///         "            cell.homeConfigurations.machine1",
    ///         "            cell.nixosProfiles.machine1",
    ///         "        ];",
    ///         "    }",
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
            quote_in!(*tokens => $(configurations.name.clone()) = $configurations)
        }
        tokens.unindent();
        tokens.append("}");
    }
}
