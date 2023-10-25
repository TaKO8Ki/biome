use crate::{
    inner_string_text, JsIdentifierAssignment, JsLiteralExportName, JsReferenceIdentifier,
    JsSyntaxToken, JsxReferenceIdentifier,
};
use biome_rowan::{declare_node_union, SyntaxResult, TokenText};

declare_node_union! {
    pub AnyJsIdentifierUsage = JsReferenceIdentifier | JsIdentifierAssignment | JsxReferenceIdentifier
}

impl AnyJsIdentifierUsage {
    pub fn value_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsIdentifierUsage::JsReferenceIdentifier(node) => node.value_token(),
            AnyJsIdentifierUsage::JsIdentifierAssignment(node) => node.name_token(),
            AnyJsIdentifierUsage::JsxReferenceIdentifier(node) => node.value_token(),
        }
    }
}

impl JsLiteralExportName {
    /// get the exported name, stripping the quotes if it is a string.
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    /// let export_name = make::js_literal_export_name(make::js_string_literal("foo")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]));
    /// assert_eq!(export_name.inner_string_text().unwrap().text(), "foo");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value()?))
    }

    /// Returns `true` if the export name is `default`.
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    /// let export_name = make::js_literal_export_name(make::js_string_literal("default"));
    /// assert!(export_name.is_default());
    ///
    /// let export_name = make::js_literal_export_name(make::js_string_literal("foo"));
    /// assert!(!export_name.is_default());
    /// ```
    pub fn is_default(&self) -> bool {
        self.inner_string_text()
            .map(|x| x.text() == "default")
            .unwrap_or(false)
    }
}
