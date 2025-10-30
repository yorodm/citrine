#!/bin/bash

# Fix all instances of SyntaxKind.into() in parser/mod.rs
sed -i 's/SyntaxKind::\([a-zA-Z]*\)\.into()/CitrineLanguage::kind_to_raw(SyntaxKind::\1)/g' src/parser/mod.rs

# Fix the token method call
sed -i 's/self\.builder\.token(kind\.into(), token\.text\.as_str());/self.builder.token(CitrineLanguage::kind_to_raw(kind), token.text.as_str());/g' src/parser/mod.rs

