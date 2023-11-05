use genco::prelude::*;

pub struct Variable {
    pub name: String,
    pub value: nix::Tokens,
}

impl Variable {
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let home_manager = Some(Inherit::home_manager());
    /// let nixpkgs = Inherit::nixpkgs();
    ///
    /// let bee = Variable::bee("bee", home_manager, nixpkgs, "x86_64-linux");
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
    pub fn bee<M, N>(name: &str, home_manager: Option<M>, nixpkgs: N, system: &str) -> Self
    where
        M: Into<nix::Tokens>,
        N: Into<nix::Tokens>,
    {
        Self {
            name: name.to_string(),
            value: quote! {
                {
                    bee = {
                        $(if let Some(home_manager) = home_manager {
                            home = $(home_manager.into());
                        })
                        pkgs = $(nixpkgs.into());
                        system = $(quoted(system));
                    };
                }
            },
        }
    }
}

impl FormatInto<Nix> for Variable {
    fn format_into(self, tokens: &mut Tokens<Nix>) {
        let variable = nix::variable(self.name, self.value);
        tokens.append(variable);
    }
}
