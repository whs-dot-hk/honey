use genco::prelude::*;

pub struct Inherit {
    pub name: String,
    pub path: String,
}

impl Inherit {
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let nixpkgs = Inherit::new("inputs", "nixpkgs");
    ///
    /// let toks = quote!($nixpkgs);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
    ///         "    inherit (inputs) nixpkgs;",
    ///         "in",
    ///         "",
    ///         "nixpkgs",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn new(path: &str, name: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
        }
    }

    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let nixpkgs = Inherit::nixpkgs();
    ///
    /// let toks = quote!($nixpkgs);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
    ///         "    inherit (inputs) nixpkgs;",
    ///         "in",
    ///         "",
    ///         "nixpkgs",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn nixpkgs() -> Self {
        Self {
            name: String::from("nixpkgs"),
            path: String::from("inputs"),
        }
    }

    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let home_manager = Inherit::home_manager();
    ///
    /// let toks = quote!($home_manager);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
    ///         "    inherit (inputs) home-manager;",
    ///         "in",
    ///         "",
    ///         "home-manager",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn home_manager() -> Self {
        Self {
            name: String::from("home-manager"),
            path: String::from("inputs"),
        }
    }

    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let disko = Inherit::disko();
    ///
    /// let toks = quote!($disko);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
    ///         "    inherit (inputs) disko;",
    ///         "in",
    ///         "",
    ///         "disko",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    pub fn disko() -> Self {
        Self {
            name: String::from("disko"),
            path: String::from("inputs"),
        }
    }
}

impl Into<nix::Tokens> for Inherit {
    fn into(self) -> nix::Tokens {
        let mut tokens = nix::Tokens::new();
        tokens.append(self);
        tokens
    }
}

impl FormatInto<Nix> for Inherit {
    fn format_into(self, tokens: &mut Tokens<Nix>) {
        let inherit = nix::inherit(self.path, self.name);
        tokens.append(inherit);
    }
}
