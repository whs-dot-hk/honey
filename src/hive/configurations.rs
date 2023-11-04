use crate::hive::*;
use genco::prelude::*;

pub enum ConfigurationType {
    Import(Import),
    Dummy,
}

impl From<Import> for ConfigurationType {
    fn from(import: Import) -> Self {
        Self::Import(import)
    }
}

pub struct Configurations {
    pub configurations: Vec<ConfigurationType>,
    /// Name do **not** included in quote.
    pub name: String,
}

impl Configurations {
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let configurations = Configurations::new("dummy", vec![
    ///     Import::cell_disko_configurations("my-disko-configurations"),
    ///     Import::disko_module(),
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
    pub fn new<T>(name: &str, imports: T) -> Self
    where
        T: IntoIterator<Item = Import>,
    {
        let mut configurations = Vec::new();
        for import in imports {
            configurations.push(ConfigurationType::from(import))
        }
        Self {
            configurations: configurations,
            name: String::from(name),
        }
    }

    pub fn new_nixos_configurations(name: &str) -> Self {
        Self {
            configurations: vec![
                Import::cell_disko_configurations(name).into(),
                Import::cell_hardware_profiles(name).into(),
                Import::cell_home_configurations(name).into(),
                Import::cell_nixos_modules(name).into(),
                Import::cell_nixos_profiles(name).into(),
                Import::disko_module().into(),
                Import::bee().into(),
            ],
            name: String::from(name),
        }
    }
}

impl IntoIterator for Configurations {
    type Item = ConfigurationType;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.configurations.into_iter()
    }
}

impl From<Configurations> for Imports {
    fn from(configurations: Configurations) -> Self {
        let mut imports = Vec::new();
        for configuration in configurations {
            if let ConfigurationType::Import(import) = configuration {
                imports.push(import)
            }
        }
        Self(imports)
    }
}

impl FormatInto<Nix> for Configurations {
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let configurations = Configurations {
    ///     configurations: vec![
    ///         ConfigurationType::Import(Import::cell_disko_configurations("my-disko-configurations")),
    ///         ConfigurationType::Import(Import::disko_module()),
    ///     ],
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
        let imports = Imports::from(self);
        quote_in! { *tokens =>
            {
                imports = $(imports);
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
    ///         "    inherit (inputs) home-manager;",
    ///         "    inherit (inputs) nixpkgs;",
    ///         "    bee = {",
    ///         "        home = home-manager;",
    ///         "        pkgs = nixpkgs;",
    ///         "        system = \"x86_64-linux\";",
    ///         "    };",
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
    ///         "            bee",
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
    ///         "            bee",
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
