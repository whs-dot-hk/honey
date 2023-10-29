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
    /// let disko = Import::disko();
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
    pub fn disko() -> Self {
        Self {
            inherit: Some(Inherit::disko()),
            name: String::from("nixosModules.disko"),
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
