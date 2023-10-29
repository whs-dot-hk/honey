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
    pub fn nixpkgs() -> Self {
        Self {
            name: String::from("nixpkgs"),
            path: String::from("inputs"),
        }
    }
}

impl FormatInto<Nix> for Inherit {
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
    fn format_into(self, tokens: &mut Tokens<Nix>) {
        let inherit = nix::inherit(self.path, self.name);
        tokens.append(inherit);
    }
}
