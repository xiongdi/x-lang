requires "domains.k"

module XLANG-SYNTAX
  imports DOMAINS
  imports BOOL
  imports INT
  imports FLOAT
  imports STRING
  imports LIST
  imports MAP

  // ====================
  // Lexical Syntax
  // ====================

  // Identifiers
  syntax Id ::= r"[a-zA-Z_][a-zA-Z0-9_-]*"  [token]
  syntax TypeName ::= Id  [token]

  // Keywords (aligned with latest spec: function, mutable, match, await, implement, etc.)
  syntax Keyword ::= "function" | "async" | "class" | "extends" | "trait" | "type"
                   | "let" | "mutable"
                   | "for" | "in" | "if" | "else" | "while" | "do"
                   | "weak"
                   | "needs" | "given" | "await" | "together" | "race"
                   | "match" | "is" | "where"
                   | "atomic" | "retry" | "with" | "or"
                   | "return" | "break" | "continue"
                   | "true" | "false" | "None" | "Some" | "Ok" | "Err"
                   | "public" | "private" | "protected" | "module"
                   | "static" | "abstract" | "final"
                   | "new" | "this" | "super"
                   | "get" | "set"
                   | "extension"
                   | "operator"
                   | "import" | "export"
                   | "as"
                   | "and" | "or" | "not"
                   | "implement"

  // ====================
  // Types
  // ====================

  syntax Type ::= BasicType
                | FunctionType
                | RecordType
                | UnionType
                | ListType
                | MapType
                | OptionType
                | ResultType
                | AsyncType
                | TupleType
                | TypeName
                | "(" Type ")"  [bracket]

  // Basic types use full English names in lowercase
  syntax BasicType ::= "integer" | "float" | "boolean" | "string" | "character" | "unit" | "never"

  syntax FunctionType ::= "(" TypeList ")" "->" Type
  syntax TypeList ::= List{Type, ","}

  syntax RecordType ::= "{" FieldDeclList "}"
  syntax FieldDeclList ::= List{FieldDecl, ","}
  syntax FieldDecl ::= Id ":" Type

  syntax UnionType ::= UnionType "|" Variant
                    | Variant
  syntax Variant ::= TypeName
                   | TypeName "{" FieldDeclList "}"

  syntax ListType ::= "[" Type "]"
  syntax MapType ::= "{" Type ":" Type "}"
  syntax OptionType ::= "Option" "<" Type ">"
  syntax ResultType ::= "Result" "<" Type "," Type ">"
  syntax AsyncType ::= "async" "<" Type ">"
  syntax TupleType ::= "(" Type "," TypeList ")"

  // ====================
  // Expressions
  // ====================

  syntax Exp ::= Literal
                | Id
                | Exp "." Id
                | Exp "(" ExpList ")"
                | "(" Exp ")"  [bracket]
                | UnOp Exp
                | Exp BinOp Exp
                | Exp "is" Type
                | Exp "as" Type
                | IfExp
                | MatchExp
                | LambdaExp
                | ListExp
                | RecordExp
                | TupleExp
                | OptionExp
                | ResultExp
                | PipeExp
                | "await" Exp
                | "await" "together" "{" ExpList "}"
                | "atomic" "{" Block "}"
                | "retry"
                | Exp "with" "{" FieldUpdateList "}"
                | Exp "or" Exp
                | Exp "?"  [strict(1)]
                | "[" Exp "|" ListComp "}"
                | "{" Exp ":" Exp "|" MapComp "}"

  syntax Literal ::= IntLit | FloatLit | BoolLit | StringLit | CharLit | UnitLit
  syntax IntLit ::= r"[0-9][0-9_]*"  [token]
                   | r"0[xX][0-9a-fA-F][0-9a-fA-F_]*"  [token]
                   | r"0[oO][0-7][0-7_]*"  [token]
                   | r"0[bB][01][01_]*"  [token]
  syntax FloatLit ::= r"[0-9][0-9_]*\.[0-9][0-9_]*([eE][+-]?[0-9][0-9_]*)?"  [token]
  syntax BoolLit ::= "true" | "false"
  syntax StringLit ::= r'"([^"\\]|\\.)*"'  [token]
                    | "\"\"\"" StringContent "\"\"\""  [token]
  syntax StringContent ::= r"([^\"\\]|\\.)*"  [token]
  syntax CharLit ::= r"'([^'\\]|\\.)'"  [token]
  syntax UnitLit ::= "()"

  syntax ExpList ::= List{Exp, ","}

  syntax UnOp ::= "-" | "not" | "!"

  syntax BinOp ::= "+" | "-" | "*" | "/" | "%" | "^"
                 | "==" | "!=" | "<" | ">" | "<=" | ">="
                 | "and" | "or" | "&&" | "||"
                 | "::"
                 | ":="
                 | "|>" | "|>>"

  syntax IfExp ::= "if" Exp "then" Exp "else" Exp

  // match 表达式（取代 when/is）
  syntax MatchExp ::= "match" Exp "{" MatchArmList "}"
  syntax MatchArmList ::= MatchArmList MatchArm
                        | MatchArm
  syntax MatchArm ::= Pattern "=>" Exp
                    | Pattern "if" Exp "=>" Exp
                    | "_" "=>" Exp

  syntax Pattern ::= Id
                   | Literal
                   | TypeName PatternArgs?
                   | "(" Pattern ")"
                   | Pattern "::" Pattern
                   | "[" PatternList "]"
                   | "{" FieldPatternList "}"
                   | "_"
  syntax PatternArgs ::= "(" PatternList ")"
  syntax PatternList ::= List{Pattern, ","}
  syntax FieldPatternList ::= List{FieldPattern, ","}
  syntax FieldPattern ::= Id
                          | Id "=" Pattern

  syntax LambdaExp ::= "(" ParamList ")" "->" Exp
                     | "(" ParamList ")" "->" "{" Block "}"
                     | Id "->" Exp

  syntax ListExp ::= "[" ExpList "]"
                   | "[" Exp ".." Exp "]"
                   | "[" Exp "..." Exp "]"

  syntax RecordExp ::= "{" FieldInitList "}"
  syntax FieldInitList ::= List{FieldInit, ","}
  syntax FieldInit ::= Id ":" Exp
                       | Id

  syntax TupleExp ::= "(" Exp "," ExpList ")"

  syntax OptionExp ::= "None" | "Some" "(" Exp ")"
  syntax ResultExp ::= "Ok" "(" Exp ")" | "Err" "(" Exp ")"

  syntax PipeExp ::= Exp "|>" Exp
                   | Exp "|>>" Exp

  syntax FieldUpdateList ::= List{FieldUpdate, ","}
  syntax FieldUpdate ::= Id ":" Exp

  syntax ListComp ::= ListCompClause ListComp
                    | ListCompClause
  syntax ListCompClause ::= Id "in" Exp
                           | Exp
  syntax MapComp ::= MapCompClause MapComp
                   | MapCompClause
  syntax MapCompClause ::= Id "in" Exp
                          | Exp

  // ====================
  // Statements
  // ====================

  syntax Stmt ::= ExpStmt
                | DeclStmt
                | IfStmt
                | WhileStmt
                | DoWhileStmt
                | ForStmt
                | UseStmt
                | ReturnStmt
                | BreakStmt
                | ContinueStmt
                | "{" Block "}"
                | ModuleStmt
                | ImportStmt
                | ExportStmt

  syntax ExpStmt ::= Exp ";"?

  syntax DeclStmt ::= LetDecl
                     | FunDecl
                     | ClassDecl
                     | TraitDecl
                     | TypeDecl
                     | ExtensionDecl

  syntax LetDecl ::= "let" LetBindingList
                    | "let" "mutable" LetBindingList
  syntax LetBindingList ::= List{LetBinding, ","}
  syntax LetBinding ::= Id ":" Type "=" Exp
                       | Id "=" Exp
                       | Pattern "=" Exp

  syntax IfStmt ::= "if" Exp "{" Block "}" ElseClause?
  syntax ElseClause ::= "else" "{" Block "}"
                      | "else" IfStmt

  syntax WhileStmt ::= "while" Exp "{" Block "}"
  syntax DoWhileStmt ::= "do" "{" Block "}" "while" Exp ";"?
  syntax ForStmt ::= "for" Id "in" Exp "{" Block "}"

  syntax UseStmt ::= Exp "use" Id "{" Block "}"

  syntax ReturnStmt ::= "return" Exp? ";"?
  syntax BreakStmt ::= "break" ";"?
  syntax ContinueStmt ::= "continue" ";"?
  syntax ModuleStmt ::= "module" Id "."? IdList ";"?
  syntax ImportStmt ::= "import" ImportItemList ";"?
  syntax ExportStmt ::= "export" ExportItemList ";"?
  syntax IdList ::= List{Id, "."}
  syntax ImportItemList ::= List{ImportItem, ","}
  syntax ImportItem ::= Id "."? IdList
                        | Id "."? IdList "as" Id
                        | Id "."? IdList ".*"
  syntax ExportItemList ::= List{ExportItem, ","}
  syntax ExportItem ::= Id "."? IdList
                        | Id "."? IdList "as" Id

  // ====================
  // Declarations
  // ====================

  syntax Decl ::= FunDecl
                | ClassDecl
                | TraitDecl
                | TypeDecl
                | ExtensionDecl

  syntax FunDecl ::= Modifier? "function" Id "(" ParamList ")" ReturnType? EffectDecl? "=" Exp
                   | Modifier? "function" Id "(" ParamList ")" ReturnType? EffectDecl? "{" Block "}"
                   | Modifier? "async" "function" Id "(" ParamList ")" ReturnType? EffectDecl? "=" Exp
                   | Modifier? "async" "function" Id "(" ParamList ")" ReturnType? EffectDecl? "{" Block "}"
                   | Modifier? FunDefClause+  // Piecewise function definition
  syntax FunDefClause ::= Id "(" PatternList ")" "=" Exp
  syntax ParamList ::= List{Param, ","}
  syntax Param ::= Id ":" Type
                 | Id ":" Type "=" Exp  // Default parameter
                 | "..." Id ":" Type    // Variadic parameter
                 | Id
  syntax ReturnType ::= ":" Type
  syntax EffectDecl ::= "needs" EffectList
  syntax EffectList ::= List{TypeName, ","}
  syntax Modifier ::= "public" | "private" | "protected" | "module"
                    | "static" | "abstract" | "final" | "overloadable"

  syntax ClassDecl ::= Modifier? "class" Id SuperClass? ImplementsList? "{" ClassBody "}"
  syntax SuperClass ::= "extends" TypeName
  syntax ImplementsList ::= "implements" TypeNameList
  syntax TypeNameList ::= List{TypeName, ","}
  syntax ClassBody ::= List{ClassMember, ""}
  syntax ClassMember ::= FieldDecl
                        | MethodDecl
                        | PropertyDecl
                        | ConstructorDecl
                        | InitializerDecl

  syntax MethodDecl ::= FunDecl
  syntax PropertyDecl ::= Modifier? "get" Id ":" Type "=" Exp
                         | Modifier? "get" Id ":" Type "{" Block "}"
                         | Modifier? "set" Id "(" Id ":" Type ")" "{" Block "}"
                         | Modifier? "get" Id ":" Type "=" Exp "set" Id "(" Id ":" Type ")" "{" Block "}"

  syntax ConstructorDecl ::= "new" "(" ParamList ")" "{" Block "}"

  syntax InitializerDecl ::= "init" "{" Block "}"

  syntax TraitDecl ::= Modifier? "trait" Id "{" TraitBody "}"
  syntax TraitBody ::= List{TraitMember, ""}
  syntax TraitMember ::= MethodSig
                        | FunDecl

  syntax MethodSig ::= Modifier? "function" Id "(" ParamList ")" ReturnType? EffectDecl? ";"?

  syntax TypeDecl ::= "type" TypeName "=" Type ";"?
                    | "type" TypeName "=" UnionType ";"?

  syntax ExtensionDecl ::= "extension" TypeName "{" ExtensionBody "}"
  syntax ExtensionBody ::= List{ExtensionMember, ""}
  syntax ExtensionMember ::= FunDecl
                            | PropertyDecl

  // ====================
  // Blocks and Programs
  // ====================

  syntax Block ::= List{Stmt, ""}

  syntax Program ::= List{TopLevelDecl, ""}
  syntax TopLevelDecl ::= Decl
                         | Stmt
                         | ModuleStmt
                         | ImportStmt
                         | ExportStmt

  // ====================
  // Operator Precedence
  // ====================

  // Unary operators
  syntax Exp ::= "-" Exp         [prec 35]
  syntax Exp ::= "not" Exp       [prec 35]
  syntax Exp ::= "!" Exp         [prec 35]

  // Exponentiation
  syntax Exp ::= Exp "^" Exp     [prec 30, right]

  // Multiplicative
  syntax Exp ::= Exp "*" Exp     [prec 25, left]
  syntax Exp ::= Exp "/" Exp     [prec 25, left]
  syntax Exp ::= Exp "%" Exp     [prec 25, left]
  syntax Exp ::= Exp "mod" Exp   [prec 25, left]

  // Additive
  syntax Exp ::= Exp "+" Exp     [prec 20, left]
  syntax Exp ::= Exp "-" Exp     [prec 20, left]

  // Cons
  syntax Exp ::= Exp "::" Exp    [prec 15, right]

  // Comparison
  syntax Exp ::= Exp "<" Exp     [prec 10, non-assoc]
  syntax Exp ::= Exp ">" Exp     [prec 10, non-assoc]
  syntax Exp ::= Exp "<=" Exp    [prec 10, non-assoc]
  syntax Exp ::= Exp ">=" Exp    [prec 10, non-assoc]

  // Equality
  syntax Exp ::= Exp "==" Exp    [prec 8, non-assoc]
  syntax Exp ::= Exp "!=" Exp    [prec 8, non-assoc]

  // Type test/cast
  syntax Exp ::= Exp "is" Type   [prec 7, left]
  syntax Exp ::= Exp "as" Type   [prec 7, left]

  // Logical AND
  syntax Exp ::= Exp "and" Exp   [prec 5, left]
  syntax Exp ::= Exp "&&" Exp    [prec 5, left]

  // Logical OR
  syntax Exp ::= Exp "or" Exp    [prec 4, left]
  syntax Exp ::= Exp "||" Exp    [prec 4, left]

  // Pipe
  syntax Exp ::= Exp "|>" Exp    [prec 3, left]
  syntax Exp ::= Exp "|>>" Exp   [prec 3, left]

  // Error propagation
  syntax Exp ::= Exp "?"         [prec 2, left]

  // With
  syntax Exp ::= Exp "with" "{" FieldUpdateList "}"  [prec 1]

  // Or (default value)
  syntax Exp ::= Exp "or" Exp    [prec 0, right]

endmodule
