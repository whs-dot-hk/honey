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
    ///         "{",
    ///         "    cell,",
    ///         "    inputs,",
    ///         "    ...",
    ///         "}:",
    ///         "",
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
                Import::bee(name).into(),
                Import::cell_disko_configurations(name).into(),
                Import::cell_hardware_profiles(name).into(),
                Import::cell_home_configurations(name).into(),
                Import::cell_nixos_modules(name).into(),
                Import::cell_nixos_profiles(name).into(),
                Import::disko_module().into(),
            ],
            name: String::from(name),
        }
    }

    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let configurations = Configurations::new_nixos_configurations1("machine1", Some("home-23-05"), "nixos-23-05", "x86_64-linux");
    ///
    /// let toks = quote!($configurations);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "{",
    ///         "    cell,",
    ///         "    inputs,",
    ///         "    ...",
    ///         "}:",
    ///         "",
    ///         "let",
    ///         "    inherit (inputs) disko;",
    ///         "    inherit (inputs) home-23-05;",
    ///         "    inherit (inputs) nixos-23-05;",
    ///         "    bee-machine1 = {",
    ///         "        bee = {",
    ///         "            home = home-23-05;",
    ///         "            pkgs = nixos-23-05.legacyPackages;",
    ///         "            system = \"x86_64-linux\";",
    ///         "        };",
    ///         "    };",
    ///         "in",
    ///         "",
    ///         "{",
    ///         "    imports = [",
    ///         "        bee-machine1",
    ///         "        cell.diskoConfigurations.machine1",
    ///         "        cell.hardwareProfiles.machine1",
    ///         "        cell.homeConfigurations.machine1",
    ///         "        cell.nixosModules.machine1",
    ///         "        cell.nixosProfiles.machine1",
    ///         "        disko.nixosModules.disko",
    ///         "    ];",
    ///         "}",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn new_nixos_configurations1(
        name: &str,
        home_manager: Option<&str>,
        nixpkgs: &str,
        system: &str,
    ) -> Self {
        let mut imports = Vec::new();
        imports.push(Import::bee1(name, home_manager, nixpkgs, system));
        imports.push(Import::cell_disko_configurations(name));
        imports.push(Import::cell_hardware_profiles(name));
        if let Some(_) = home_manager {
            imports.push(Import::cell_home_configurations(name));
        }
        imports.push(Import::cell_nixos_modules(name));
        imports.push(Import::cell_nixos_profiles(name));
        imports.push(Import::disko_module());
        Self {
            configurations: imports.into_iter().map(|import| import.into()).collect(),
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
    ///         "{",
    ///         "    cell,",
    ///         "    inputs,",
    ///         "    ...",
    ///         "}:",
    ///         "",
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

impl NixosConfigurations {
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let nixos_configurations = NixosConfigurations::new("machine1");
    ///
    /// let toks = quote!($nixos_configurations);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "{",
    ///         "    cell,",
    ///         "    inputs,",
    ///         "    ...",
    ///         "}:",
    ///         "",
    ///         "let",
    ///         "    inherit (inputs) disko;",
    ///         "    inherit (inputs) home-manager;",
    ///         "    bee-machine1 = {",
    ///         "        bee = {",
    ///         "            home = home-manager;",
    ///         "            pkgs = cell.pkgs.machine1;",
    ///         "            system = \"x86_64-linux\";",
    ///         "        };",
    ///         "    };",
    ///         "in",
    ///         "",
    ///         "{",
    ///         "    machine1 = {",
    ///         "        imports = [",
    ///         "            bee-machine1",
    ///         "            cell.diskoConfigurations.machine1",
    ///         "            cell.hardwareProfiles.machine1",
    ///         "            cell.homeConfigurations.machine1",
    ///         "            cell.nixosModules.machine1",
    ///         "            cell.nixosProfiles.machine1",
    ///         "            disko.nixosModules.disko",
    ///         "        ];",
    ///         "    };",
    ///         "}",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn new(name: &str) -> Self {
        Self(vec![Configurations::new_nixos_configurations(name)])
    }

    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let nixos_configurations = NixosConfigurations::new1("machine", 2, Some("home-23-05"), "nixos-23-05", "x86_64-linux");
    ///
    /// let toks = quote!($nixos_configurations);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "{",
    ///         "    cell,",
    ///         "    inputs,",
    ///         "    ...",
    ///         "}:",
    ///         "",
    ///         "let",
    ///         "    inherit (inputs) disko;",
    ///         "    inherit (inputs) home-23-05;",
    ///         "    inherit (inputs) nixos-23-05;",
    ///         "    bee-machine00 = {",
    ///         "        bee = {",
    ///         "            home = home-23-05;",
    ///         "            pkgs = nixos-23-05.legacyPackages;",
    ///         "            system = \"x86_64-linux\";",
    ///         "        };",
    ///         "    };",
    ///         "    bee-machine01 = {",
    ///         "        bee = {",
    ///         "            home = home-23-05;",
    ///         "            pkgs = nixos-23-05.legacyPackages;",
    ///         "            system = \"x86_64-linux\";",
    ///         "        };",
    ///         "    };",
    ///         "in",
    ///         "",
    ///         "{",
    ///         "    machine00 = {",
    ///         "        imports = [",
    ///         "            bee-machine00",
    ///         "            cell.diskoConfigurations.machine00",
    ///         "            cell.hardwareProfiles.machine00",
    ///         "            cell.homeConfigurations.machine00",
    ///         "            cell.nixosModules.machine00",
    ///         "            cell.nixosProfiles.machine00",
    ///         "            disko.nixosModules.disko",
    ///         "        ];",
    ///         "    };",
    ///         "    machine01 = {",
    ///         "        imports = [",
    ///         "            bee-machine01",
    ///         "            cell.diskoConfigurations.machine01",
    ///         "            cell.hardwareProfiles.machine01",
    ///         "            cell.homeConfigurations.machine01",
    ///         "            cell.nixosModules.machine01",
    ///         "            cell.nixosProfiles.machine01",
    ///         "            disko.nixosModules.disko",
    ///         "        ];",
    ///         "    };",
    ///         "}",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn new1(
        prefix: &str,
        number: u32,
        home_manager: Option<&str>,
        nixpkgs: &str,
        system: &str,
    ) -> Self {
        let mut configurations = Vec::new();
        for i in 0..number {
            let name = &format!("{}{:02}", prefix, i);
            configurations.push(Configurations::new_nixos_configurations1(
                name,
                home_manager,
                nixpkgs,
                system,
            ))
        }
        Self(configurations)
    }
}

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
    ///         "{",
    ///         "    cell,",
    ///         "    inputs,",
    ///         "    ...",
    ///         "}:",
    ///         "",
    ///         "let",
    ///         "    inherit (inputs) disko;",
    ///         "    inherit (inputs) home-manager;",
    ///         "    bee-machine1 = {",
    ///         "        bee = {",
    ///         "            home = home-manager;",
    ///         "            pkgs = cell.pkgs.machine1;",
    ///         "            system = \"x86_64-linux\";",
    ///         "        };",
    ///         "    };",
    ///         "    bee-machine2 = {",
    ///         "        bee = {",
    ///         "            home = home-manager;",
    ///         "            pkgs = cell.pkgs.machine2;",
    ///         "            system = \"x86_64-linux\";",
    ///         "        };",
    ///         "    };",
    ///         "in",
    ///         "",
    ///         "{",
    ///         "    machine1 = {",
    ///         "        imports = [",
    ///         "            bee-machine1",
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
    ///         "            bee-machine2",
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
