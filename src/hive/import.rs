use crate::hive::*;
use genco::prelude::*;

pub struct Import {
    pub inherit: Option<Inherit>,
    pub name: nix::Tokens,
}

impl Import {
    /// Create a new `Import` with inherit.
    ///
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let disko = Import::new("inputs", "disko", "nixosModules.disko");
    ///
    /// let toks = quote!($disko);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
    ///         "    inherit (inputs) disko;",
    ///         "in",
    ///         "",
    ///         "disko.nixosModules.disko",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn new(path: &str, var: &str, name: &str) -> Self {
        Self {
            inherit: Some(Inherit {
                name: var.to_string(),
                path: path.to_string(),
            }),
            name: quote!($name),
        }
    }

    /// Create a new `Import` **without** inherit.
    ///
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let my_home_configurations = Import::new1("cell.homeConfigurations.my-home-configurations");
    ///
    /// let toks = quote!($my_home_configurations);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "cell.homeConfigurations.my-home-configurations",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn new1(name: &str) -> Self {
        Self {
            inherit: None,
            name: quote!($name),
        }
    }

    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let disko_module = Import::disko_module();
    ///
    /// let toks = quote!($disko_module);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
    ///         "    inherit (inputs) disko;",
    ///         "in",
    ///         "",
    ///         "disko.nixosModules.disko",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn disko_module() -> Self {
        Self {
            inherit: Some(Inherit::disko()),
            name: quote!(nixosModules.disko),
        }
    }

    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let my_hardware_profile = Import::cell_hardware_profiles("my-hardware-profile");
    ///
    /// let toks = quote!($my_hardware_profile);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "cell.hardwareProfiles.my-hardware-profile",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn cell_hardware_profiles(name: &str) -> Self {
        Self {
            inherit: None,
            name: quote!(cell.hardwareProfiles.$name),
        }
    }

    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let my_nixos_profile = Import::cell_nixos_profiles("my-nixos-profile");
    ///
    /// let toks = quote!($my_nixos_profile);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "cell.nixosProfiles.my-nixos-profile",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn cell_nixos_profiles(name: &str) -> Self {
        Self {
            inherit: None,
            name: quote!(cell.nixosProfiles.$name),
        }
    }

    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let my_disko_configurations = Import::cell_disko_configurations("my-disko-configurations");
    ///
    /// let toks = quote!($my_disko_configurations);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "cell.diskoConfigurations.my-disko-configurations",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn cell_disko_configurations(name: &str) -> Self {
        Self {
            inherit: None,
            name: quote!(cell.diskoConfigurations.$name),
        }
    }

    pub fn cell_home_configurations(name: &str) -> Self {
        Self {
            inherit: None,
            name: quote!(cell.homeConfigurations.$name),
        }
    }

    pub fn cell_nixos_modules(name: &str) -> Self {
        Self {
            inherit: None,
            name: quote!(cell.nixosModules.$name),
        }
    }

    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let bee = Import::bee("machine1");
    ///
    /// let toks = quote!($bee);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
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
    ///         "bee-machine1"
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn bee(name: &str) -> Self {
        let home_manager = Some(Inherit::home_manager());
        let nixpkgs = quote!(cell.pkgs.$name);
        let bee = Variable::bee(
            &format!("bee-{}", name),
            home_manager,
            nixpkgs,
            "x86_64-linux",
        );
        Self {
            inherit: None,
            name: quote!($bee),
        }
    }

    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let bee = Import::bee2(Some("home-23-05"), "nixos-23-05", "x86_64-linux");
    ///
    /// let toks = quote!($bee);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
    ///         "    inherit (inputs) home-23-05;",
    ///         "    inherit (inputs) nixos-23-05;",
    ///         "    bee = {",
    ///         "        bee = {",
    ///         "            home = home-23-05;",
    ///         "            pkgs = nixos-23-05;",
    ///         "            system = \"x86_64-linux\";",
    ///         "        };",
    ///         "    };",
    ///         "in",
    ///         "",
    ///         "bee"
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn bee2(home_manager: Option<&str>, nixpkgs: &str, system: &str) -> Self {
        let home_manager = if let Some(home_manager) = home_manager {
            Some(Inherit::new("inputs", home_manager))
        } else {
            None
        };
        let nixpkgs = Inherit::new("inputs", nixpkgs);
        let bee = Variable::bee("bee", home_manager, nixpkgs, system);
        Self {
            inherit: None,
            name: quote!($bee),
        }
    }

    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let home_manager = Some(Inherit::home_manager());
    /// let nixpkgs = Inherit::nixpkgs();
    ///
    /// let bee = Import::bee3(home_manager, nixpkgs, "x86_64-linux");
    ///
    /// let toks = quote!($bee);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
    ///         "    inherit (inputs) home-manager;",
    ///         "    inherit (inputs) nixpkgs;",
    ///         "    bee = {",
    ///         "        bee = {",
    ///         "            home = home-manager;",
    ///         "            pkgs = nixpkgs;",
    ///         "            system = \"x86_64-linux\";",
    ///         "        };",
    ///         "    };",
    ///         "in",
    ///         "",
    ///         "bee"
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn bee3<M, N>(home_manager: Option<M>, nixpkgs: N, system: &str) -> Self
    where
        M: Into<nix::Tokens>,
        N: Into<nix::Tokens>,
    {
        let bee = Variable::bee("bee", home_manager, nixpkgs, system);
        Self {
            inherit: None,
            name: quote!($bee),
        }
    }
}

impl FormatInto<Nix> for Import {
    fn format_into(self, tokens: &mut Tokens<Nix>) {
        if let Some(inherit) = self.inherit {
            quote_in!(*tokens => $inherit.$(self.name))
        } else {
            quote_in!(*tokens => $(self.name))
        }
    }
}

pub struct Imports(pub Vec<Import>);

impl IntoIterator for Imports {
    type Item = Import;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FormatInto<Nix> for Imports {
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let imports = Imports(vec![
    ///     Import::cell_hardware_profiles("my-hardware-profile"),
    ///     Import::cell_nixos_profiles("my-nix-profile"),
    /// ]);
    ///
    /// let toks = quote!($imports);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "[",
    ///         "    cell.hardwareProfiles.my-hardware-profile",
    ///         "    cell.nixosProfiles.my-nix-profile",
    ///         "]"
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    fn format_into(self, tokens: &mut Tokens<Nix>) {
        tokens.append("[");
        tokens.indent();
        for import in self {
            tokens.append(import);
            tokens.push();
        }
        tokens.unindent();
        tokens.append("]");
    }
}
