use crate::hive::*;
use genco::prelude::*;

pub struct Import {
    pub inherit: Option<Inherit>,
    pub name: String,
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
            name: name.to_string(),
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
            name: name.to_string(),
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
            name: String::from("nixosModules.disko"),
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
            name: format!("cell.hardwareProfiles.{}", name),
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
            name: format!("cell.nixosProfiles.{}", name),
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
            name: format!("cell.diskoConfigurations.{}", name),
        }
    }

    pub fn cell_home_configurations(name: &str) -> Self {
        Self {
            inherit: None,
            name: format!("cell.homeConfigurations.{}", name),
        }
    }

    pub fn cell_nixos_modules(name: &str) -> Self {
        Self {
            inherit: None,
            name: format!("cell.nixosModules.{}", name),
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
        for import in self.0 {
            tokens.append(import);
            tokens.push();
        }
        tokens.unindent();
        tokens.append("]");
    }
}
