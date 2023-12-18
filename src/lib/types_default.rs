// This source file was derived from the Rust ignore crate. The original source
// file was found at the following URL and repository state:
// https://github.com/BurntSushi/ripgrep/blob/3f2fe0afee0d1a1eeb3235904cfef4f35c4644dc/crates/ignore/src/default_types.rs
// It was commmitted to this source repository by Chris Simpkins on 2023-12-17
// under the MIT license. Changes made to the original source file on this path are
// documented in the commit history of this repository and licensed under the
// siz project Apache 2.0 license.

//The MIT License (MIT)

// Copyright (c) 2015 Andrew Gallant

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#[rustfmt::skip]
pub(crate) const DEFAULT_TYPES: &[(&[&str], &[&str])] = &[
    (&["ada"], &["*.adb", "*.ads"]),
    (&["agda"], &["*.agda", "*.lagda"]),
    (&["aidl"], &["*.aidl"]),
    (&["alire"], &["alire.toml"]),
    (&["amake"], &["*.mk", "*.bp"]),
    (&["apk"], &["*.apk", "*.apks", "*.aab", "*.xapk", "*.apkm", "*.akp"]),
    (&["appleimg"], &["*.dmg", "*.smi", "*.img"]),
    (&["asciidoc"], &["*.adoc", "*.asc", "*.asciidoc"]),
    (&["asm"], &["*.asm", "*.s", "*.S"]),
    (&["asp"], &[
        "*.aspx", "*.aspx.cs", "*.aspx.vb", "*.ascx", "*.ascx.cs",
        "*.ascx.vb", "*.asp"
    ]),
    (&["ats"], &["*.ats", "*.dats", "*.sats", "*.hats"]),
    (&["avro"], &["*.avdl", "*.avpr", "*.avsc"]),
    (&["awk"], &["*.awk"]),
    (&["bat", "batch"], &["*.bat"]),
    (&["bazel"], &[
        "*.bazel", "*.bzl", "*.BUILD", "*.bazelrc", "BUILD", "MODULE.bazel",
        "WORKSPACE", "WORKSPACE.bazel",
    ]),
    (&["bitbake"], &["*.bb", "*.bbappend", "*.bbclass", "*.conf", "*.inc"]),
    (&["bmp"], &["*.bmp", "*.dib"]),
    (&["brotli"], &["*.br"]),
    (&["buildstream"], &["*.bst"]),
    (&["bzip2"], &["*.bz2", "*.tbz2"]),
    (&["c"], &["*.[chH]", "*.[chH].in", "*.cats"]),
    (&["cabal"], &["*.cabal"]),
    (&["candid"], &["*.did"]),
    (&["carp"], &["*.carp"]),
    (&["cbor"], &["*.cbor"]),
    (&["ceylon"], &["*.ceylon"]),
    (&["clojure"], &["*.clj", "*.cljc", "*.cljs", "*.cljx"]),
    (&["cmake"], &["*.cmake", "CMakeLists.txt"]),
    (&["cmd"], &["*.bat", "*.cmd"]),
    (&["cml"], &["*.cml"]),
    (&["coffeescript"], &["*.coffee"]),
    (&["config"], &["*.cfg", "*.conf", "*.config", "*.ini"]),
    (&["coq"], &["*.v"]),
    (&["cpp"], &[
        "*.[ChH]", "*.cc", "*.[ch]pp", "*.[ch]xx", "*.hh",  "*.inl",
        "*.[ChH].in", "*.cc.in", "*.[ch]pp.in", "*.[ch]xx.in", "*.hh.in",
    ]),
    (&["creole"], &["*.creole"]),
    (&["crystal"], &["Projectfile", "*.cr", "*.ecr", "shard.yml"]),
    (&["cs"], &["*.cs"]),
    (&["csharp"], &["*.cs"]),
    (&["cshtml"], &["*.cshtml"]),
    (&["csproj"], &["*.csproj"]),
    (&["css"], &["*.css", "*.scss"]),
    (&["csv"], &["*.csv"]),
    (&["cuda"], &["*.cu", "*.cuh"]),
    (&["cython"], &["*.pyx", "*.pxi", "*.pxd"]),
    (&["d"], &["*.d"]),
    (&["dart"], &["*.dart"]),
    (&["deb"], &["*.deb", "*.udeb"]),
    (&["devicetree"], &["*.dts", "*.dtsi"]),
    (&["dhall"], &["*.dhall"]),
    (&["diff"], &["*.patch", "*.diff"]),
    (&["dita"], &["*.dita", "*.ditamap", "*.ditaval"]),
    (&["docker"], &["*Dockerfile*"]),
    (&["dockercompose"], &["docker-compose.yml", "docker-compose.*.yml"]),
    (&["dts"], &["*.dts", "*.dtsi"]),
    (&["dvc"], &["Dvcfile", "*.dvc"]),
    (&["ebuild"], &["*.ebuild", "*.eclass"]),
    (&["edn"], &["*.edn"]),
    (&["elisp"], &["*.el"]),
    (&["elixir"], &["*.ex", "*.eex", "*.exs", "*.heex", "*.leex", "*.livemd"]),
    (&["elm"], &["*.elm"]),
    (&["erb"], &["*.erb"]),
    (&["erlang"], &["*.erl", "*.hrl"]),
    (&["exe"], &["*.[Ee][Xx][Ee]"]),
    (&["fennel"], &["*.fnl"]),
    (&["fidl"], &["*.fidl"]),
    (&["fish"], &["*.fish"]),
    (&["flatbuffers"], &["*.fbs"]),
    (&["fortran"], &[
        "*.f", "*.F", "*.f77", "*.F77", "*.pfo",
        "*.f90", "*.F90", "*.f95", "*.F95",
    ]),
    (&["fsharp"], &["*.fs", "*.fsx", "*.fsi"]),
    (&["fut"], &["*.fut"]),
    (&["gap"], &["*.g", "*.gap", "*.gi", "*.gd", "*.tst"]),
    (&["gif"], &["*.gif"]),
    (&["gn"], &["*.gn", "*.gni"]),
    (&["go"], &["*.go"]),
    (&["gprbuild"], &["*.gpr"]),
    (&["gradle"], &[
        "*.gradle", "*.gradle.kts", "gradle.properties", "gradle-wrapper.*",
        "gradlew", "gradlew.bat",
    ]),
    (&["graphql"], &["*.graphql", "*.graphqls"]),
    (&["groovy"], &["*.groovy", "*.gradle"]),
    (&["gzip"], &["*.gz", "*.tgz"]),
    (&["h"], &["*.h", "*.hh", "*.hpp"]),
    (&["haml"], &["*.haml"]),
    (&["hare"], &["*.ha"]),
    (&["haskell"], &["*.hs", "*.lhs", "*.cpphs", "*.c2hs", "*.hsc"]),
    (&["hbs"], &["*.hbs"]),
    (&["hs"], &["*.hs", "*.lhs"]),
    (&["html"], &["*.htm", "*.html", "*.ejs"]),
    (&["hy"], &["*.hy"]),
    (&["idris"], &["*.idr", "*.lidr"]),
    (&["janet"], &["*.janet"]),
    (&["jar"], &["*.jar"]),
    (&["java"], &["*.java", "*.jsp", "*.jspx", "*.properties"]),
    (&["jinja"], &["*.j2", "*.jinja", "*.jinja2"]),
    (&["jl"], &["*.jl"]),
    (&["jpg"], &["*.jpg", "*.jpeg"]),
    (&["js"], &["*.js", "*.jsx", "*.vue", "*.cjs", "*.mjs"]),
    (&["json"], &["*.json", "composer.lock", "*.sarif"]),
    (&["jsonl"], &["*.jsonl"]),
    (&["julia"], &["*.jl"]),
    (&["jupyter"], &["*.ipynb", "*.jpynb"]),
    (&["k"], &["*.k"]),
    (&["kotlin"], &["*.kt", "*.kts"]),
    (&["lean"], &["*.lean"]),
    (&["less"], &["*.less"]),
    (&["license"], &[
        // General
        "COPYING", "COPYING[.-]*",
        "COPYRIGHT", "COPYRIGHT[.-]*",
        "EULA", "EULA[.-]*",
        "licen[cs]e", "licen[cs]e.*",
        "LICEN[CS]E", "LICEN[CS]E[.-]*", "*[.-]LICEN[CS]E*",
        "NOTICE", "NOTICE[.-]*",
        "PATENTS", "PATENTS[.-]*",
        "UNLICEN[CS]E", "UNLICEN[CS]E[.-]*",
        // GPL (gpl.txt, etc.)
        "agpl[.-]*",
        "gpl[.-]*",
        "lgpl[.-]*",
        // Other license-specific (APACHE-2.0.txt, etc.)
        "AGPL-*[0-9]*",
        "APACHE-*[0-9]*",
        "BSD-*[0-9]*",
        "CC-BY-*",
        "GFDL-*[0-9]*",
        "GNU-*[0-9]*",
        "GPL-*[0-9]*",
        "LGPL-*[0-9]*",
        "MIT-*[0-9]*",
        "MPL-*[0-9]*",
        "OFL-*[0-9]*",
    ]),
    (&["lilypond"], &["*.ly", "*.ily"]),
    (&["lisp"], &["*.el", "*.jl", "*.lisp", "*.lsp", "*.sc", "*.scm"]),
    (&["lock"], &["*.lock", "package-lock.json"]),
    (&["log"], &["*.log"]),
    (&["lua"], &["*.lua"]),
    (&["lz4"], &["*.lz4"]),
    (&["lzma"], &["*.lzma"]),
    (&["m4"], &["*.ac", "*.m4"]),
    (&["make"], &[
        "[Gg][Nn][Uu]makefile", "[Mm]akefile",
        "[Gg][Nn][Uu]makefile.am", "[Mm]akefile.am",
        "[Gg][Nn][Uu]makefile.in", "[Mm]akefile.in",
        "*.mk", "*.mak"
    ]),
    (&["mako"], &["*.mako", "*.mao"]),
    (&["man"], &["*.[0-9lnpx]", "*.[0-9][cEFMmpSx]"]),
    (&["markdown", "md"], &[
        "*.markdown",
        "*.md",
        "*.mdown",
        "*.mdwn",
        "*.mkd",
        "*.mkdn",
        "*.mdx",
    ]),
    (&["matlab"], &["*.m"]),
    (&["meson"], &["meson.build", "meson_options.txt", "meson.options"]),
    (&["minified"], &["*.min.html", "*.min.css", "*.min.js"]),
    (&["mint"], &["*.mint"]),
    (&["mk"], &["mkfile"]),
    (&["ml"], &["*.ml"]),
    (&["motoko"], &["*.mo"]),
    (&["msbuild"], &[
        "*.csproj", "*.fsproj", "*.vcxproj", "*.proj", "*.props", "*.targets",
        "*.sln",
    ]),
    (&["nim"], &["*.nim", "*.nimf", "*.nimble", "*.nims"]),
    (&["nix"], &["*.nix"]),
    (&["objc"], &["*.h", "*.m"]),
    (&["objcpp"], &["*.h", "*.mm"]),
    (&["ocaml"], &["*.ml", "*.mli", "*.mll", "*.mly"]),
    (&["org"], &["*.org", "*.org_archive"]),
    (&["pants"], &["BUILD"]),
    (&["pascal"], &["*.pas", "*.dpr", "*.lpr", "*.pp", "*.inc"]),
    (&["pdf"], &["*.pdf"]),
    (&["perl"], &["*.perl", "*.pl", "*.PL", "*.plh", "*.plx", "*.pm", "*.t"]),
    (&["php"], &[
        // note that PHP 6 doesn't exist
        // See: https://wiki.php.net/rfc/php6
        "*.php", "*.php3", "*.php4", "*.php5", "*.php7", "*.php8",
        "*.pht", "*.phtml"
    ]),
    (&["png"], &["*.png"]),
    (&["po"], &["*.po"]),
    (&["pod"], &["*.pod"]),
    (&["postscript"], &["*.eps", "*.ps"]),
    (&["prolog"], &["*.pl", "*.pro", "*.prolog", "*.P"]),
    (&["protobuf"], &["*.proto"]),
    (&["ps"], &["*.cdxml", "*.ps1", "*.ps1xml", "*.psd1", "*.psm1"]),
    (&["puppet"], &["*.epp", "*.erb", "*.pp", "*.rb"]),
    (&["purs"], &["*.purs"]),
    (&["py", "python"], &["*.py", "*.pyi"]),
    (&["qmake"], &["*.pro", "*.pri", "*.prf"]),
    (&["qml"], &["*.qml"]),
    (&["r"], &["*.R", "*.r", "*.Rmd", "*.Rnw"]),
    (&["racket"], &["*.rkt"]),
    (&["raku"], &[
        "*.raku", "*.rakumod", "*.rakudoc", "*.rakutest",
        "*.p6", "*.pl6", "*.pm6"
    ]),
    (&["rdoc"], &["*.rdoc"]),
    (&["readme"], &["README*", "*README"]),
    (&["reasonml"], &["*.re", "*.rei"]),
    (&["red"], &["*.r", "*.red", "*.reds"]),
    (&["rescript"], &["*.res", "*.resi"]),
    (&["robot"], &["*.robot"]),
    (&["rpm"], &["*.rpm"]),
    (&["rst"], &["*.rst"]),
    (&["ruby"], &[
        // Idiomatic files
        "config.ru", "Gemfile", ".irbrc", "Rakefile",
        // Extensions
        "*.gemspec", "*.rb", "*.rbw"
    ]),
    (&["rust"], &["*.rs"]),
    (&["sass"], &["*.sass", "*.scss"]),
    (&["scala"], &["*.scala", "*.sbt"]),
    (&["sh"], &[
        // Portable/misc. init files
        ".login", ".logout", ".profile", "profile",
        // bash-specific init files
        ".bash_login", "bash_login",
        ".bash_logout", "bash_logout",
        ".bash_profile", "bash_profile",
        ".bashrc", "bashrc", "*.bashrc",
        // csh-specific init files
        ".cshrc", "*.cshrc",
        // ksh-specific init files
        ".kshrc", "*.kshrc",
        // tcsh-specific init files
        ".tcshrc",
        // zsh-specific init files
        ".zshenv", "zshenv",
        ".zlogin", "zlogin",
        ".zlogout", "zlogout",
        ".zprofile", "zprofile",
        ".zshrc", "zshrc",
        // Extensions
        "*.bash", "*.csh", "*.ksh", "*.sh", "*.tcsh", "*.zsh",
    ]),
    (&["slim"], &["*.skim", "*.slim", "*.slime"]),
    (&["smarty"], &["*.tpl"]),
    (&["sml"], &["*.sml", "*.sig"]),
    (&["solidity"], &["*.sol"]),
    (&["soy"], &["*.soy"]),
    (&["spark"], &["*.spark"]),
    (&["spec"], &["*.spec"]),
    (&["sql"], &["*.sql", "*.psql"]),
    (&["stylus"], &["*.styl"]),
    (&["sv"], &["*.v", "*.vg", "*.sv", "*.svh", "*.h"]),
    (&["svg"], &["*.svg"]),
    (&["swift"], &["*.swift"]),
    (&["swig"], &["*.def", "*.i"]),
    (&["systemd"], &[
        "*.automount", "*.conf", "*.device", "*.link", "*.mount", "*.path",
        "*.scope", "*.service", "*.slice", "*.socket", "*.swap", "*.target",
        "*.timer",
    ]),
    (&["tar"], &["*.tar", "*.tar.*", "*.tgz", "*.tbz2", "*.txz"]),
    (&["taskpaper"], &["*.taskpaper"]),
    (&["tcl"], &["*.tcl"]),
    (&["tex"], &["*.tex", "*.ltx", "*.cls", "*.sty", "*.bib", "*.dtx", "*.ins"]),
    (&["texinfo"], &["*.texi"]),
    (&["textile"], &["*.textile"]),
    (&["tf"], &[
        "*.tf", "*.auto.tfvars", "terraform.tfvars", "*.tf.json",
        "*.auto.tfvars.json", "terraform.tfvars.json", "*.terraformrc",
        "terraform.rc", "*.tfrc", "*.terraform.lock.hcl",
    ]),
    (&["thrift"], &["*.thrift"]),
    (&["toml"], &["*.toml", "Cargo.lock"]),
    (&["ts", "typescript"], &["*.ts", "*.tsx", "*.cts", "*.mts"]),
    (&["twig"], &["*.twig"]),
    (&["txt"], &["*.txt"]),
    (&["typoscript"], &["*.typoscript", "*.ts"]),
    (&["usd"], &["*.usd", "*.usda", "*.usdc"]),
    (&["v"], &["*.v", "*.vsh"]),
    (&["vala"], &["*.vala"]),
    (&["vb"], &["*.vb"]),
    (&["vcl"], &["*.vcl"]),
    (&["verilog"], &["*.v", "*.vh", "*.sv", "*.svh"]),
    (&["vhdl"], &["*.vhd", "*.vhdl"]),
    (&["vim"], &[
        "*.vim", ".vimrc", ".gvimrc", "vimrc", "gvimrc", "_vimrc", "_gvimrc",
    ]),
    (&["vimscript"], &[
        "*.vim", ".vimrc", ".gvimrc", "vimrc", "gvimrc", "_vimrc", "_gvimrc",
    ]),
    (&["webidl"], &["*.idl", "*.webidl", "*.widl"]),
    (&["webp"], &["*.webp"]),
    (&["wiki"], &["*.mediawiki", "*.wiki"]),
    (&["xml"], &[
        "*.xml", "*.xml.dist", "*.dtd", "*.xsl", "*.xslt", "*.xsd", "*.xjb",
        "*.rng", "*.sch", "*.xhtml",
    ]),
    (&["xz"], &["*.xz", "*.txz"]),
    (&["yacc"], &["*.y"]),
    (&["yaml"], &["*.yaml", "*.yml"]),
    (&["yang"], &["*.yang"]),
    (&["z"], &["*.Z"]),
    (&["zig"], &["*.zig"]),
    (&["zip"], &["*.zip", "*.zipx", "*.z01", "*.zx01"]),
    (&["zsh"], &[
        ".zshenv", "zshenv",
        ".zlogin", "zlogin",
        ".zlogout", "zlogout",
        ".zprofile", "zprofile",
        ".zshrc", "zshrc",
        "*.zsh",
    ]),
    (&["zstd"], &["*.zst", "*.zstd"]),
];

#[cfg(test)]
mod tests {
    use super::DEFAULT_TYPES;

    #[test]
    fn default_types_are_sorted() {
        let mut names = DEFAULT_TYPES.iter().map(|(aliases, _)| aliases[0]);
        let Some(mut previous_name) = names.next() else {
            return;
        };
        for name in names {
            assert!(
                name > previous_name,
                r#""{}" should be sorted before "{}" in `DEFAULT_TYPES`"#,
                name,
                previous_name
            );
            previous_name = name;
        }
    }
}
