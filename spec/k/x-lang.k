requires "x-lang-syntax.k"
requires "domains.k"

module XLANG
  imports XLANG-SYNTAX
  imports DOMAINS
  imports MAP
  imports LIST

  // ====================
  // Configuration
  // ====================

  configuration
    <T>
      <k> $PGM:Program </k>
      <env> .Map </env>
      <store> .Map </store>
      <stack> .List </stack>
      <effects> .Map </effects>
      <output> .List </output>
    </T>

  // ====================
  // Auxiliary Functions
  // ====================

  // Fresh variable generation
  syntax Int ::= fresh()  [function, klabel(fresh)]
  rule <k> fresh() => I ...</k>
       <store> ... I |-> _ ...</Store>
       requires I ==Int maxStoreKey(Store) +Int 1
  rule fresh() => 1
       requires maxStoreKey(.Map) ==Int 0

  syntax Int ::= maxStoreKey(Map)  [function]
  rule maxStoreKey(.Map) => 0
  rule maxStoreKey(M:Map I |-> _) => max(I, maxStoreKey(M))
       when notBool (I in keys(M))

  // Environment lookup
  syntax KItem ::= lookupEnv(Id)  [function, klabel(lookupEnv)]
  rule <k> lookupEnv(X) => V ...</k>
       <env> ... X |-> V ...</env>

  // Store operations
  syntax KItem ::= load(KItem)  [function, klabel(load)]
  rule <k> load(Loc) => V ...</k>
       <store> ... Loc |-> V ...</store>

  syntax KItem ::= store(KItem, KItem)  [function, klabel(store)]
  rule <k> store(Loc, V) => Loc ...</k>
       <store> S => S[Loc <- V] </store>

  syntax KItem ::= alloc(KItem)  [function, klabel(alloc)]
  rule <k> alloc(V) => Loc ...</k>
       <store> S => S[Loc <- V] </store>
       <env> _ </env>
       when Loc ==Int fresh()

  // ====================
  // Literals
  // ====================

  rule <k> I:IntLit => I:Int ...</k>
  rule <k> F:FloatLit => F:Float ...</k>
  rule <k> true => true ...</k>
  rule <k> false => false ...</k>
  rule <k> S:StringLit => S:String ...</k>
  rule <k> () => unit ...</k>

  // ====================
  // Variables and Environment
  // ====================

  // Variable lookup
  rule <k> X:Id => lookupEnv(X) ...</k>
       <env> Env:Map </env>
       when X in keys(Env)

  // Let declaration (immutable)
  rule <k> let X:Id = E:Exp ; => . ...</k>
       <env> Env => Env[X <- V] </env>
       <k> V ...</k>
       [structural]

  rule <k> let X:Id : T:Type = E:Exp ; => . ...</k>
       <env> Env => Env[X <- V] </env>
       <k> V ...</k>
       [structural]

  // Let declaration (mutable)
  rule <k> let mutable X:Id = E:Exp ; => . ...</k>
       <env> Env => Env[X <- Loc] </env>
       <store> Store => Store[Loc <- V] </store>
       <k> V ...</k>
       when Loc ==Int fresh()
       [structural]

  rule <k> let mutable X:Id : T:Type = E:Exp ; => . ...</k>
       <env> Env => Env[X <- Loc] </env>
       <store> Store => Store[Loc <- V] </store>
       <k> V ...</k>
       when Loc ==Int fresh()
       [structural]

  // Assignment to mutable variable
  rule <k> X:Id = E:Exp ; => . ...</k>
       <env> ... X |-> Loc ...</env>
       <store> Store => Store[Loc <- V] </store>
       <k> V ...</k>
       [structural]

  // ====================
  // Arithmetic Operations
  // ====================

  rule <k> I1:Int + I2:Int => I1 +Int I2 ...</k>
  rule <k> F1:Float + F2:Float => F1 +Float F2 ...</k>
  rule <k> I1:Int + F2:Float => I1:Float +Float F2 ...</k>
  rule <k> F1:Float + I2:Int => F1 +Float I2:Float ...</k>

  rule <k> I1:Int - I2:Int => I1 -Int I2 ...</k>
  rule <k> F1:Float - F2:Float => F1 -Float F2 ...</k>

  rule <k> I1:Int * I2:Int => I1 *Int I2 ...</k>
  rule <k> F1:Float * F2:Float => F1 *Float F2 ...</k>

  rule <k> I1:Int / I2:Int => I1 /Int I2 ...</k>
  rule <k> F1:Float / F2:Float => F1 /Float F2 ...</k>

  rule <k> I1:Int % I2:Int => I1 modInt I2 ...</k>
  rule <k> I1:Int mod I2:Int => I1 modInt I2 ...</k>

  rule <k> I1:Int ^ I2:Int => powInt(I1, I2) ...</k>
  rule <k> F1:Float ^ F2:Float => F1 ^Float F2 ...</k>

  rule <k> - I:Int => 0 -Int I ...</k>
  rule <k> - F:Float => 0.0 -Float F ...</k>

  // ====================
  // Boolean Operations
  // ====================

  rule <k> true and true => true ...</k>
  rule <k> true and false => false ...</k>
  rule <k> false and _ => false ...</k>
  rule <k> true && true => true ...</k>
  rule <k> true && false => false ...</k>
  rule <k> false && _ => false ...</k>

  rule <k> true or _ => true ...</k>
  rule <k> false or true => true ...</k>
  rule <k> false or false => false ...</k>
  rule <k> true || _ => true ...</k>
  rule <k> false || true => true ...</k>
  rule <k> false || false => false ...</k>

  rule <k> not true => false ...</k>
  rule <k> not false => true ...</k>
  rule <k> ! true => false ...</k>
  rule <k> ! false => true ...</k>

  // ====================
  // Comparison Operations
  // ====================

  rule <k> I1:Int == I2:Int => I1 ==Int I2 ...</k>
  rule <k> F1:Float == F2:Float => F1 ==Float F2 ...</k>
  rule <k> S1:String == S2:String => S1 ==String S2 ...</k>
  rule <k> true == true => true ...</k>
  rule <k> true == false => false ...</k>
  rule <k> false == true => false ...</k>
  rule <k> false == false => true ...</k>

  rule <k> V1:KItem != V2:KItem => not (V1 == V2) ...</k>

  rule <k> I1:Int < I2:Int => I1 <Int I2 ...</k>
  rule <k> F1:Float < F2:Float => F1 <Float F2 ...</k>
  rule <k> I1:Int > I2:Int => I1 >Int I2 ...</k>
  rule <k> F1:Float > F2:Float => F1 >Float F2 ...</k>
  rule <k> I1:Int <= I2:Int => I1 <=Int I2 ...</k>
  rule <k> F1:Float <= F2:Float => F1 <=Float F2 ...</k>
  rule <k> I1:Int >= I2:Int => I1 >=Int I2 ...</k>
  rule <k> F1:Float >= F2:Float => F1 >=Float F2 ...</k>

  // ====================
  // If Expressions
  // ====================

  rule <k> if true then E1:Exp else _:Exp => E1 ...</k>
  rule <k> if false then _:Exp else E2:Exp => E2 ...</k>

  // ====================
  // If Statements
  // ====================

  rule <k> if true { S:Block } _ => S ...</k>
  rule <k> if false { _:Block } => . ...</k>
  rule <k> if false { _:Block } else { S:Block } => S ...</k>

  // ====================
  // Blocks and Sequencing
  // ====================

  rule <k> { . } => . ...</k>
  rule <k> { S1:Stmt S2:Block } => S1 ~> { S2 } ...</k>

  // ====================
  // Functions
  // ====================

  // Function declaration
  rule <k> function F:Id ( P:ParamList ) = E:Exp ; => . ...</k>
       <env> Env => Env[F <- closure(F, P, E, Env)] </env>
       [structural]

  rule <k> function F:Id ( P:ParamList ) { B:Block } ; => . ...</k>
       <env> Env => Env[F <- closure(F, P, B, Env)] </env>
       [structural]

  // Function application
  rule <k> F:Id ( A:ExpList ) =>
            substClosure(Closure, A) ...</k>
       <env> ... F |-> Closure:Closure ...</env>

  syntax KItem ::= substClosure(KItem, ExpList)  [function]
  rule <k> substClosure(closure(F, X, E, Env), A) =>
            pushFrame(F, Env) ~> bindParams(X, A) ~> E ...</k>
       <stack> Stk => Stk ListItem(currentFrame) </stack>
       <env> _ </env>
       <store> _ </store>

  syntax KItem ::= bindParams(ParamList, ExpList)  [function]
  rule <k> bindParams(.ParamList, .ExpList) => . ...</k>
  rule <k> bindParams((X:Id, Xs:ParamList), (V:KItem, Vs:ExpList)) =>
            bindParam(X, V) ~> bindParams(Xs, Vs) ...</k>

  syntax KItem ::= bindParam(Id, KItem)  [function]
  rule <k> bindParam(X:Id, V:KItem) => . ...</k>
       <env> Env => Env[X <- V] </env>

  // Call stack
  syntax KItem ::= pushFrame(Id, Map)  [function]
  syntax KItem ::= popFrame()  [function]
  syntax Frame ::= frame(Id, Map)  [klabel(frame)]
  syntax KItem ::= currentFrame  [function]
  rule <k> popFrame() => . ...</k>
       <stack> (ListItem(frame(F, Env)) => .List) </stack>
       <env> _ => Env </env>

  // Return from function
  rule <k> return V:KItem ; =>
            V ~> popFrame() ...</k>

  // ====================
  // Lists
  // ====================

  rule <k> [ .ExpList ] => .List ...</k>
  rule <k> [ V:KItem, Vs:ExpList ] => ListItem(V) [ Vs ] ...</k>

  // List operations
  rule <k> ListItem(V:LItem) :: L:List => ListItem(V) L ...</k>

  // ====================
  // Records
  // ====================

  rule <k> { .FieldInitList } => .Map ...</k>
  rule <k> { X:Id : V:KItem, Fs:FieldInitList } =>
            (X |-> V) { Fs } ...</k>
  rule <k> { X:Id, Fs:FieldInitList } =>
            (X |-> lookupEnv(X)) { Fs } ...</k>
       <env> Env:Map </env>
       when X in keys(Env)

  // Record access
  rule <k> R:KItem . X:Id => lookupMap(X, R) ...</k>
       when isMap(R)

  syntax KItem ::= lookupMap(KItem, KItem)  [function]
  rule lookupMap(X:Id, (X |-> V) _) => V
  rule lookupMap(X:Id, (_ M:Map)) => lookupMap(X, M)
       when notBool (X in keys((X |-> V) M))

  // ====================
  // Option Type
  // ====================

  rule <k> None => none ...</k>
  rule <k> Some(V:KItem) => some(V) ...</k>

  // ====================
  // Result Type
  // ====================

  rule <k> Ok(V:KItem) => ok(V) ...</k>
  rule <k> Err(E:KItem) => err(E) ...</k>

  // ====================
  // Error Propagation (Option/Result, no exceptions)
  // ====================

  rule <k> ok(V:KItem) ? => V ...</k>
  rule <k> err(E:KItem) ? => err(E) ...</k>

  rule <k> some(V:KItem) ? => V ...</k>
  rule <k> none ? => none ...</k>

  // ====================
  // Default Value
  // ====================

  rule <k> some(V:KItem) or _:Exp => V ...</k>
  rule <k> none or D:Exp => D ...</k>

  rule <k> ok(V:KItem) or _:Exp => V ...</k>
  rule <k> err(_:KItem) or D:Exp => D ...</k>

  // ====================
  // Copy-With
  // ====================

  rule <k> R:Map with { .FieldUpdateList } => R ...</k>
  rule <k> R:Map with { X:Id : V:KItem, Fs:FieldUpdateList } =>
            (R[X <- V]) with { Fs } ...</k>

  // ====================
  // Pipe Operator
  // ====================

  rule <k> V:KItem |> F:Exp => F(V) ...</k>
  rule <k> V:KItem |>> F:Exp => F(V) ...</k>

  // ====================
  // While Loops
  // ====================

  rule <k> while C:Exp { B:Block } =>
            if C { B while C { B } } else { . } ...</k>

  // ====================
  // For Loops
  // ====================

  rule <k> for X:Id in .List { _:Block } => . ...</k>
  rule <k> for X:Id in ListItem(V:KItem) L:List { B:Block } =>
            let X = V ; { B } for X in L { B } ...</k>

  // ====================
  // Pattern Matching (match)
  // ====================

  // NOTE: Surface语法已从 when/is 切换为 match，
  // 这里仍然复用 Pattern / matches / matchPattern 这套内部机制。

  syntax Bool ::= matches(KItem, Pattern)  [function]
  rule matches(V:KItem, _:Id) => true
  rule matches(V:KItem, V:KItem) => true
  rule matches(some(V:KItem), Some(P:Pattern)) => matches(V, P)
  rule matches(none, None) => true
  rule matches(ok(V:KItem), Ok(P:Pattern)) => matches(V, P)
  rule matches(err(E:KItem), Err(P:Pattern)) => matches(E, P)
  rule matches(ListItem(V1:KItem) L1:List, P2::Ps:Pattern) =>
        matches(V1, P2) andBool matches(L1, Ps)
  rule matches(.List, []) => true
  rule matches(M:Map, { .FieldPatternList }) => true
  rule matches(M:Map, { X:Id = P:Pattern, Fs:FieldPatternList }) =>
        matches(lookupMap(X, M), P) andBool matches(M, { Fs })
       when X in keys(M)
  rule matches(_:KItem, _:Pattern) => false  [owise]

  syntax KItem ::= matchPattern(KItem, Pattern)  [function]
  rule matchPattern(V:KItem, X:Id) => bindParam(X, V)
  rule matchPattern(V:KItem, _:KItem) => .
  rule matchPattern(some(V:KItem), Some(P:Pattern)) => matchPattern(V, P)
  rule matchPattern(ListItem(V1:KItem) L1:List, P2::Ps:Pattern) =>
        matchPattern(V1, P2) ~> matchPattern(L1, Ps)
  rule matchPattern(.List, []) => .
  rule matchPattern(M:Map, { X:Id = P:Pattern, Fs:FieldPatternList }) =>
        matchPattern(lookupMap(X, M), P) ~> matchPattern(M, { Fs })
       when X in keys(M)

  // ====================
  // Print / Output
  // ====================

  rule <k> print(V:KItem) ; => . ...</k>
       <output> Out => Out ListItem(V) </output>

  // ====================
  // Effects (needs/given)
  // ====================

  rule <k> function F:Id ( P:ParamList ) needs Es:EffectList = E:Exp ; => . ...</k>
       <env> Env => Env[F <- closure(F, P, E, Env, Es)] </env>
       [structural]

  rule <k> E:Exp given { . } => E ...</k>
  rule <k> E:Exp given { X:Id <- V:KItem, Gs } =>
            withEffect(X, V) E given { Gs } ...</k>

  syntax KItem ::= withEffect(Id, KItem)  [function]
  rule <k> withEffect(X:Id, V:KItem) => . ...</k>
       <effects> Eff => Eff[X <- V] </effects>

  // ====================
  // Async (await)
  // ====================

  rule <k> await E:Exp => await(E) ...</k>

  rule <k> await together { .ExpList } => .List ...</k>
  rule <k> await together { E:Exp, Es:ExpList } =>
            await(E) :: await together { Es } ...</k>

  // ====================
  // Atomic (STM)
  // ====================

  rule <k> atomic { B:Block } =>
            startAtomic() ~> { B } ~> commitAtomic() ...</k>

  syntax KItem ::= startAtomic()  [function]
  syntax KItem ::= commitAtomic()  [function]
  syntax KItem ::= retry  [function]

  // ====================
  // Strictness Attributes
  // ====================

  // Function application is strict in arguments
  syntax Exp ::= Exp "(" ExpList ")"  [strict(2)]

  // Binary operations are strict
  syntax Exp ::= Exp BinOp Exp  [strict(1, 2)]

  // Unary operations are strict
  syntax Exp ::= UnOp Exp  [strict(2)]

  // If is strict in condition
  syntax Exp ::= "if" Exp "then" Exp "else" Exp  [strict(1)]
  syntax Stmt ::= "if" Exp "{" Block "}"  [strict(1)]

endmodule
