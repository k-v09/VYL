package main

import (
	"fmt"
)

/*
Parsing: the source text is converted to an abstract syntax tree (AST).
    Variable types must be preserved, as well as the location of each declaration in source code.
    The order of executable statements must be explicitly represented and well defined.
    Left and right components of binary operations must be stored and correctly identified.
    Identifiers and their assigned values must be stored for assignment statements.
Resolution of references to other modules (C postpones this step untill linking).
Semantic validation: weeding out syntactically correct statements that make no sense, e.g. unreachable code or duplicate declarations.
Equivalent transformations and high-level optimization: the AST is transformed to represent a more efficient computation with the same semantics. This includes e.g. early calculation of common subexpressions and constant expressions, eliminating excessive local assignments (see also SSA), etc.
Code generation: the AST is transformed into linear low-level code, with jumps, register allocation and the like. Some function calls can be inlined at this stage, some loops unrolled, etc.
Peephole optimization: the low-level code is scanned for simple local inefficiencies which are eliminated.

*/

func main() {
	fmt.Println("I work wahoo")
}
