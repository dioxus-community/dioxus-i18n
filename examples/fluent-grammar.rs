//! This example demonstrates many of the Fluent grammar constructs, and how they are
//! used in dioxus-i18n.
//! This performs a lookup only, no additional translation files are provided

use dioxus::prelude::*;
use dioxus_i18n::{tid, use_init_i18n, I18nConfig};
use unic_langid::langid;

use std::path::PathBuf;

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
#[component]
fn Body() -> Element {
    rsx! {
        table {
            tbody {
                tr {
                    td { "Simple message" }
                    td { {tid!("simple-message")} }
                }
                tr {
                    td { "Non-existing message: id provided by default when using tid! macro" }
                    td { {tid!("non-existing-message")} }
                }
                tr {
                    td { "Message with a variable" }
                    td { {tid!("message-with-variable", name: "Value 1")} }
                }
                tr {
                    td { }
                    td { {tid!("message-with-variable", name: "Value 2")} }
                }
                tr {
                    td { "Reference to a term" }
                    td { {tid!("message-referencing-a-term")} }
                }
                tr {
                    td { "Use of special characters." }
                    td { {tid!("message-with-special-character")} }
                }
                tr {
                    td { "Message with blanks." }
                    td { "'" {tid!("blank-is-removed")} "'" }
                }
                tr {
                    td { }
                    td { "'" {tid!("blank-is-preserved")} "'" }
                }
                tr {
                    td { "Message with attributes: root" }
                    td { {tid!("message-with-attributes")} }
                }
                tr {
                    td { "Message with attributes: attribute" }
                    td { {tid!("message-with-attributes.placeholder")} }
                }
                tr {
                    td { }
                    td { {tid!("message-with-attributes.aria-label")} }
                }
                tr {
                    td { }
                    td { {tid!("message-with-attributes.title")} }
                }
                tr {
                    td { "Message with attributes: not existing" }
                    td { {tid!("message-with-attributes.not-existing")} }
                }
                tr {
                    td { "Message with attributes: invalid" }
                    td { {tid!("message-with-attributes.placeholder.invalid")} }
                }
                tr {
                    td { "Message with quotes: cryptic" }
                    td { {tid!("literal-quote-cryptic")} }
                }
                tr {
                    td { "Message with quotes: preferred" }
                    td { {tid!("literal-quote-preferred")} }
                }
                tr {
                    td { "Message with Unicode characters: cryptic" }
                    td { {tid!("unicode-cryptic")} }
                }
                tr {
                    td { "Message with Unicode characters: preferred" }
                    td { {tid!("unicode-preferred")} }
                }
                tr {
                    td { "Message with a placeable: single-line" }
                    td { {tid!("line-single")} }
                }
                tr {
                    td { "Message with a placeable: single-line" }
                    td { {tid!("single-line")} }
                }
                tr {
                    td { "Message with a placeable: multi-line (1)" }
                    td { {tid!("multi-line")} }
                }
                tr {
                    td { "Message with a placeable: multi-line (2)" }
                    td { pre { {tid!("multi-line")} } }
                }
                tr {
                    td { "Message with a placeable: block-line (1)" }
                    td { {tid!("block-line")} }
                }
                tr {
                    td { "Message with a placeable: block-line (2)" }
                    td { pre { {tid!("block-line")} } }
                }
                tr {
                    td { "Message using functions: no function" }
                    td { pre { {tid!("time-elapsed-no-function", duration: 23.7114812589)} } }
                }
                tr {
                    td { "Message using functions: function" }
                    td { pre { {tid!("time-elapsed-function", duration: 23.7114812589)} } }
                }
                tr {
                    td { "Reference to a message" }
                    td { {tid!("message-referencing-another-message")} }
                }
                tr {
                    td { "Message selection: plurals" }
                    td { {tid!("message-selection-plurals", value: 1)} }
                }
                tr {
                    td { }
                    td { {tid!("message-selection-plurals", value: 2)} }
                }
                tr {
                    td { "Message selection: plurals (default: an 'empty' value must be provided...)" }
                    td { {tid!("message-selection-plurals", value: "")} }
                }
                tr {
                    td { "Message selection: plurals (default: ... otherwise an error is raised)" }
                    td { {tid!("message-selection-plurals")} }
                }
                tr {
                    td { "Message selection: numeric" }
                    td { {tid!("message-selection-numeric", value: 0.0)} }
                }
                tr {
                    td { }
                    td { {tid!("message-selection-numeric", value: 0.5)} }
                }
                tr {
                    td { }
                    td { {tid!("message-selection-numeric", value: 42.0)} }
                }
                tr {
                    td { "Message selection: numeric (default)" }
                    td { {tid!("message-selection-numeric", value: "")} }
                }
                tr {
                    td { "Message selection: number" }
                    td { {tid!("message-selection-number", pos: 1)} }
                }
                tr {
                    td { "" }
                    td { {tid!("message-selection-number", pos: 2)} }
                }
                tr {
                    td { "" }
                    td { {tid!("message-selection-number", pos: 3)} }
                }
                tr {
                    td { "" }
                    td { {tid!("message-selection-number", pos: 4)} }
                }
                tr {
                    td { "Variables in references (1)" }
                    td { {tid!("message-using-term-with-variable")} }
                }
                tr {
                    td { "Variables in references (2)" }
                    td { {tid!("message-using-term-with-variable-2-1")} }
                }
                tr {
                    td { }
                    td { {tid!("message-using-term-with-variable-2-2")} }
                }
                tr {
                    td { }
                    td { {tid!("message-using-term-with-variable-2-default")} }
                }
                tr {
                    td { }
                    td { {tid!("message-using-term-with-variable-2-not-provided")} }
                }
                tr {
                    td { "Literals: string" }
                    td { {tid!("string-literal")} }
                }
                tr {
                    td { }
                    td { {tid!("number-literal-1")} }
                }
                tr {
                    td { }
                    td { {tid!("number-literal-2")} }
                }
                tr {
                    td { }
                    td { {tid!("number-literal-3")} }
                }
                tr {
                    td { }
                    td { {tid!("inline-expression-placeable-1")} }
                }
                tr {
                    td { }
                    td { {tid!("inline-expression-placeable-2")} }
                }
            }
        }
    }
}

fn app() -> Element {
    use_init_i18n(|| {
        // Only one example in this path, which contains the complete Fluent grammar.
        I18nConfig::new(langid!("en")).with_auto_locales(PathBuf::from("./examples/data/fluent/"))
    });

    rsx!(Body {})
}
