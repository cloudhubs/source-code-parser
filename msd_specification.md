
### General Algorithm/Structures

- The user provides an MSD, a JSON which contains an array of trees
  - Each node contained immediately within the array is a "Root Parser". All other nodes are considered "Parsers".
    - Both describe a node within the AST we expect to find.
  - Root Parsers:
    - All Root Parsers are applied against all nodes in the AST seeking subtree matches.
    - Root Parsers are applied in the order of the provided array. This allows one Parser to build on work from another.
  - Parsers:
    - Not all Parsers must match perfectly. That is, nodes that exist in the AST can be "skipped" to simplify the parsers that must be defined.
    - A Parser can also be marked 'essential', indicating that it must be matched at some point in the tree for the subtree to be considered "matched".
      - If an essential Parser fails to match, then the match is aborted; no further user-defined action is taken.
      - Root Parsers are implicitly essential.
- Parsers are matched by verifying an internal type flag, and matching user-defined regex(es).
  - Upon a match, information is copied out of the regex match to the Parser Context. See [ParserContext](ParserContext).
  - Then, child nodes of the matched node are passed to subpatterns of the Parser in a node-specific manner.
- If a subtree has completed matching and has a defined `callback`, then the callback is executed. See [Callback](Callback).
- Once all nodes are checked against all parsers, the context is cleaned and returned. See [ParserContext](ParserContext).

### Pattern Matching

- Each Parser has a Pattern and an optional Auxiliary Pattern. These are applied against information in the tree (see [Parsers](Parsers)). For a Parser to begin matching, both the Pattern and Auxiliary Pattern (if provided) must match.
- The patterns are Regexes with the following added syntax:
  - `#{varname}`: Expands to `P<varname>.*`, but copies the captured string into the ParserContext as a variable named `varname` in the ParserContext.
  - `#&{object_name}`: Acts like `#{object_name}`, with the added constraint that `object_name`'s captured value must correspond to the name of an existing Object or Tag within the ParserContext.
  - After the close brace, the user may optionally specify an alternate regex the variable must follow in parenthesis, like `#{myvar}([a-z]+)`. This replaces `.*` in the expansion with `[a-z]+`.
- Matches are verified when a node matches, but their variables are not inserted into the context until the parse is completed and all essential children have matched successfully, so they are not available to callbacks in subnodes.

### ParserContext

- The ParserContext stores three types of information:
  - Objects: Non-volatile named entitities containing key-value data, like a hash map. Values on keys can be `None`, or null.
	  - Objects should not attempt having a key `???`, as this is used as an internal sentinel value. User usage may cause undesired behavior or crashes.
  - Tags: Non-volatile aliases for objects which may or may not yet exist. This allows bridging between different names for one object.
	  - Tags are transparently resolved to Objects, so be careful not to have naming collisions between them.
  - Variables: Temporary key/value pairs. These are scraped from the parser's matches, and used in the callback to create objects. These are cleared whenever a Root Parser finishes matching.
	  - Variable names are restricted to `a-z`, `A-Z`, `-`, and `_` at current. This set may be expanded, but will not shrink.
- When execution completes, only Objects are returned; tags and variables are cleared.

### Callbacks

- The callbacks define where the user can populate Objects using the information they've scraped.
- Each node can be defined with a callback. BE CAREFUL, however! Callbacks incur overhead, and they can cause logical errors if you call them at a point the match could still fail (e.g., essential nodes still remaining to be matched). Unless you really know what you're doing, just stick to one in each Root Parser node.
- Callbacks are in the Rune language, which allows for syntax similar to Rust. See [the Rune book](https://rune-rs.github.io/book/) for details.
- Available within Callbacks:
  - Full Rune default library. This includes NO file i/o or network connectivity, so there should be no danger running arbitrary Rune scripts.
  - `ctx`: The parser context
- `ctx` offers the following methods:
  - `make_object(name: &str)`: Defines an object with a given name in the context.
  - `make_attribute(name: &str, attr_name: &str, attr_value: Option<String>)`: Defines an attribute on an Object with a provided name in the context. Provides an optional value that the attribute will take on.
  - `make_tag(name: &str, resolves_to: &str)`: Defines a Tag in the context.
  - `get_object(name: &str) -> Option<HashMap<String, Option<String>>>`: Retrieves a copy of the object in the context with the given name. Automatically flattens `Tag`s, so it always returns a true Object, or `None` if the name was invalid.
  - `make_variable(name: &str, val: &str)`: Define a Variable in the context.
  - `get_variable(name: &str) -> Option<String>`: Get the value of a Variable from the context.
  - `clear_variables()`: Manually delete all variables from the context. Happens implicitly on a Root Parser completing, after its final Callback.

### Parsers

- ClassOrInterface: Describes a class definition.
  - Subpatterns: applied against all annotations, fields, and method definitions
  - Pattern: applied against class name
  - Auxiliary pattern: unused
- Method: Describes a method definition.
  - Subpatterns: applied against all annotations, submethods, and the method body. All MethodParam subpattern nodes are applied as a subsequence against method parameters.
  - Pattern: Applied against method name
  - Auxiliary Pattern: applied against the return type of the method
- MethodParam: Describes a parameter to a method.
  - Subpatterns: applied against all annotations
  - Pattern: Applied against parameter name
  - Auxiliary Pattern: applied against the parameter’s type
- Field: Describes a field in a class.
  - Subpatterns: applied against all annotations
  - Pattern: Applied against the field’s name
  - Auxiliary Pattern: applied against the field’s type
- Annotation: Describes an annotation on a class.
  - Subpattern: applied against all AnnotationValuePairs
  - Value: applied against the annotation’s name
  - Auxiliary Pattern: applied against the annotation’s unnamed value
- AnnotationValuePair: Applies to one key-value pair of an annotation
  - Subpattern: None
  - Pattern: Applied against the key
  - Auxiliary Pattern: applied against the value
- CallExpr: Describes a method call
  - Subpattern: applied against annotations. All subnodes that could be arguments to the call (CallExpr, VarDecl, Ident, Literal) are applied as a subsequence against method arguments.
  - Pattern: applied against the method name
  - Auxiliary Pattern: applied against the lefthand side, if specified. If omitted, all subpatterns are applied against the lefthand side node.
- VarDecl: Describes a single variable declaration
  - Subpattern: applied against all annotations
  - Pattern: applied against the variable’s name
  - Auxiliary Pattern: applied against the variable’s type
- DeclStmt: Describes an overarching variable declaration. This is the most complicated node, thanks to language-specific details.
  - Subpattern: Three cases for application:
    - If there is exactly one VarDecl child to this node, it is matched against some child in the variables list and the expressions list is ignored.
    - If there are an equal number of VarDecls and non-VarDecl nodes, then they are paired up in order of encounter (so first VarDecl with first non-VarDecl, second VarDecl with second non-VarDecl, etc.) and matched against all found declarations.
    - If there are more non-VarDecl nodes than VarDecl nodes, then the VarDecls are applied in-order against all found VarDecls (failing if there are too many/few declarations), and the non-VarDecls are applied against all expressions.
    - Having more VarDecl nodes than non-VarDecl nodes is currently considered invalid.
  - Pattern: unused
  - Subpattern: unused
- Ident: Describes a named identifier—like a method name, variable name, etc.
  - Subpattern: unused
  - Pattern: applied against the Ident’s name
  - Subpattern: unused
- Literal: Describes a value literal—like a string literal, numeric literal, etc.
  - Subpattern: unused
  - Pattern: applied against the Literal’s value
  - Subpattern: unused
