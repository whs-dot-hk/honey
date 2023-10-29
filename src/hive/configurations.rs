use crate::hive::*;
use genco::prelude::*;

pub enum ConfigurationType {
    Import(Import),
}

pub struct Configurations {
    pub imports: Imports,
}

impl Configurations {
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let configurations = Configurations::new(vec![
    ///     ConfigurationType::Import(Import::disko()),
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
    pub fn new(configurations: Vec<ConfigurationType>) -> Self {
        let mut imports = Vec::new();
        for c in configurations {
            match c {
                ConfigurationType::Import(import) => imports.push(import),
            }
        }
        Self {
            imports: Imports(imports),
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
    ///         Import::disko(),
    ///         Import::disko_configurations("my-disko-configurations"),
    ///     ]),
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
