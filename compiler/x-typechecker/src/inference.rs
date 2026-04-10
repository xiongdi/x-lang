use std::collections::HashMap;
use x_parser::ast::Type;

pub type TypeVar = String;

#[derive(Debug, Clone, Default)]
pub struct Substitution {
    pub mapping: HashMap<TypeVar, Type>,
}

impl Substitution {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    pub fn singleton(var: TypeVar, ty: Type) -> Self {
        let mut s = Self::new();
        s.mapping.insert(var, ty);
        s
    }

    pub fn apply(&self, ty: &Type) -> Type {
        match ty {
            Type::Var(name) => {
                if let Some(replacement) = self.mapping.get(name) {
                    self.apply(replacement)
                } else {
                    ty.clone()
                }
            }
            Type::TypeParam(name) => {
                if let Some(replacement) = self.mapping.get(name) {
                    self.apply(replacement)
                } else {
                    ty.clone()
                }
            }
            Type::Array(elem) => Type::Array(Box::new(self.apply(elem))),
            Type::Tuple(elems) => Type::Tuple(elems.iter().map(|e| self.apply(e)).collect()),
            Type::Function(params, ret) => Type::Function(
                params.iter().map(|p| Box::new(self.apply(p))).collect(),
                Box::new(self.apply(ret)),
            ),
            Type::Dictionary(k, v) => {
                Type::Dictionary(Box::new(self.apply(k)), Box::new(self.apply(v)))
            }
            Type::Async(inner) => Type::Async(Box::new(self.apply(inner))),
            Type::Reference(inner) => Type::Reference(Box::new(self.apply(inner))),
            Type::MutableReference(inner) => Type::MutableReference(Box::new(self.apply(inner))),
            Type::Pointer(inner) => Type::Pointer(Box::new(self.apply(inner))),
            Type::ConstPointer(inner) => Type::ConstPointer(Box::new(self.apply(inner))),
            Type::TypeConstructor(name, args) => {
                Type::TypeConstructor(name.clone(), args.iter().map(|a| self.apply(a)).collect())
            }
            _ => ty.clone(),
        }
    }

    pub fn compose(&self, other: &Substitution) -> Substitution {
        let mut result = Substitution::new();

        for (var, ty) in &self.mapping {
            result.mapping.insert(var.clone(), other.apply(ty));
        }

        for (var, ty) in &other.mapping {
            result.mapping.insert(var.clone(), ty.clone());
        }

        result
    }

    pub fn free_vars(&self, ty: &Type) -> Vec<TypeVar> {
        match ty {
            Type::Var(name) | Type::TypeParam(name) => vec![name.clone()],
            Type::Array(elem) => self.free_vars(elem),
            Type::Tuple(elems) => elems.iter().flat_map(|e| self.free_vars(e)).collect(),
            Type::Function(params, ret) => {
                let mut vars: Vec<TypeVar> =
                    params.iter().flat_map(|p| self.free_vars(p)).collect();
                vars.extend(self.free_vars(ret));
                vars
            }
            Type::Dictionary(k, v) => {
                let mut vars = self.free_vars(k);
                vars.extend(self.free_vars(v));
                vars
            }
            Type::Async(inner) => self.free_vars(inner),
            Type::Reference(inner) | Type::MutableReference(inner) => self.free_vars(inner),
            Type::Pointer(inner) | Type::ConstPointer(inner) => self.free_vars(inner),
            Type::TypeConstructor(_, args) => args.iter().flat_map(|a| self.free_vars(a)).collect(),
            _ => vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InferenceError {
    TypeMismatch(Type, Type),
    InfiniteType(TypeVar, Type),
    UndefinedVariable(String),
    UndefinedFunction(String),
    OccursCheckFailed(TypeVar, Type),
}

#[derive(Debug, Clone)]
pub struct TypeScheme {
    pub bound_vars: Vec<TypeVar>,
    pub ty: Type,
}

impl TypeScheme {
    pub fn new(bound_vars: Vec<TypeVar>, ty: Type) -> Self {
        Self { bound_vars, ty }
    }

    pub fn mono(ty: Type) -> Self {
        Self {
            bound_vars: vec![],
            ty,
        }
    }
}

pub fn occurs_check(var: &TypeVar, ty: &Type) -> bool {
    match ty {
        Type::Var(name) | Type::TypeParam(name) => var == name,
        Type::Array(elem) => occurs_check(var, elem),
        Type::Tuple(elems) => elems.iter().any(|e| occurs_check(var, e)),
        Type::Function(params, ret) => {
            params.iter().any(|p| occurs_check(var, p)) || occurs_check(var, ret)
        }
        Type::Dictionary(k, v) => occurs_check(var, k) || occurs_check(var, v),
        Type::Async(inner) => occurs_check(var, inner),
        Type::Reference(inner) | Type::MutableReference(inner) => occurs_check(var, inner),
        Type::Pointer(inner) | Type::ConstPointer(inner) => occurs_check(var, inner),
        Type::TypeConstructor(_, args) => args.iter().any(|a| occurs_check(var, a)),
        _ => false,
    }
}

pub fn unify(t1: &Type, t2: &Type) -> Result<Substitution, InferenceError> {
    match (t1, t2) {
        (Type::Var(name), other) | (other, Type::Var(name)) => {
            if occurs_check(name, other) {
                Err(InferenceError::InfiniteType(name.clone(), other.clone()))
            } else {
                Ok(Substitution::singleton(name.clone(), other.clone()))
            }
        }

        (Type::TypeParam(name), other) | (other, Type::TypeParam(name)) => {
            if let Type::TypeParam(name2) = other {
                if name == name2 {
                    return Ok(Substitution::new());
                }
            }
            if occurs_check(name, other) {
                Err(InferenceError::InfiniteType(name.clone(), other.clone()))
            } else {
                Ok(Substitution::singleton(name.clone(), other.clone()))
            }
        }

        (Type::Int, Type::Int)
        | (Type::Float, Type::Float)
        | (Type::Bool, Type::Bool)
        | (Type::String, Type::String)
        | (Type::Char, Type::Char)
        | (Type::Unit, Type::Unit)
        | (Type::Never, Type::Never)
        | (Type::Void, Type::Void) => Ok(Substitution::new()),

        (Type::Generic(n1), Type::Generic(n2)) if n1 == n2 => Ok(Substitution::new()),

        (Type::Array(e1), Type::Array(e2)) => unify(e1, e2),

        (Type::Tuple(ts1), Type::Tuple(ts2)) => {
            if ts1.len() != ts2.len() {
                return Err(InferenceError::TypeMismatch(t1.clone(), t2.clone()));
            }
            let mut subst = Substitution::new();
            for (e1, e2) in ts1.iter().zip(ts2.iter()) {
                let s = unify(&subst.apply(e1), &subst.apply(e2))?;
                subst = subst.compose(&s);
            }
            Ok(subst)
        }

        (Type::Dictionary(k1, v1), Type::Dictionary(k2, _v2)) => {
            let s1 = unify(k1, k2)?;
            let k2_subst = s1.apply(k2);
            let v1_subst = s1.apply(v1);
            let v2_subst = s1.apply(&k2_subst);
            let s2 = unify(&v1_subst, &v2_subst)?;
            Ok(s1.compose(&s2))
        }

        (Type::Function(params1, ret1), Type::Function(params2, ret2)) => {
            if params1.len() != params2.len() {
                return Err(InferenceError::TypeMismatch(t1.clone(), t2.clone()));
            }
            let mut subst = Substitution::new();
            for (p1, p2) in params1.iter().zip(params2.iter()) {
                let s = unify(&subst.apply(p1), &subst.apply(p2))?;
                subst = subst.compose(&s);
            }
            let ret1_subst = subst.apply(ret1);
            let ret2_subst = subst.apply(ret2);
            let s = unify(&ret1_subst, &ret2_subst)?;
            Ok(subst.compose(&s))
        }

        (Type::Async(i1), Type::Async(i2)) => unify(i1, i2),

        (Type::Reference(i1), Type::Reference(i2)) => unify(i1, i2),
        (Type::MutableReference(i1), Type::MutableReference(i2)) => unify(i1, i2),

        (Type::Pointer(i1), Type::Pointer(i2)) => unify(i1, i2),
        (Type::ConstPointer(i1), Type::ConstPointer(i2)) => unify(i1, i2),

        (Type::TypeConstructor(n1, args1), Type::TypeConstructor(n2, args2)) => {
            if n1 != n2 || args1.len() != args2.len() {
                return Err(InferenceError::TypeMismatch(t1.clone(), t2.clone()));
            }
            let mut subst = Substitution::new();
            for (a1, a2) in args1.iter().zip(args2.iter()) {
                let s = unify(&subst.apply(a1), &subst.apply(a2))?;
                subst = subst.compose(&s);
            }
            Ok(subst)
        }

        _ => Err(InferenceError::TypeMismatch(t1.clone(), t2.clone())),
    }
}

pub struct TypeInferrer {
    var_counter: u64,
}

impl TypeInferrer {
    pub fn new() -> Self {
        Self { var_counter: 0 }
    }

    pub fn fresh_type_var(&mut self) -> Type {
        self.var_counter += 1;
        Type::Var(format!("?t{}", self.var_counter))
    }

    pub fn instantiate(&mut self, scheme: &TypeScheme) -> Type {
        if scheme.bound_vars.is_empty() {
            return scheme.ty.clone();
        }

        let mut subst = Substitution::new();
        for var in &scheme.bound_vars {
            let fresh = self.fresh_type_var();
            subst.mapping.insert(var.clone(), fresh);
        }

        subst.apply(&scheme.ty)
    }

    pub fn generalize(&self, env: &HashMap<String, TypeScheme>, ty: &Type) -> TypeScheme {
        let env_vars: Vec<TypeVar> = env
            .values()
            .flat_map(|scheme| self.free_type_vars(&scheme.ty))
            .collect();

        let ty_vars = self.free_type_vars(ty);
        let bound_vars: Vec<TypeVar> = ty_vars
            .into_iter()
            .filter(|v| !env_vars.contains(v))
            .collect();

        TypeScheme::new(bound_vars, ty.clone())
    }

    fn free_type_vars(&self, ty: &Type) -> Vec<TypeVar> {
        match ty {
            Type::Var(name) | Type::TypeParam(name) => vec![name.clone()],
            Type::Array(elem) => self.free_type_vars(elem),
            Type::Tuple(elems) => elems.iter().flat_map(|e| self.free_type_vars(e)).collect(),
            Type::Function(params, ret) => {
                let mut vars: Vec<TypeVar> =
                    params.iter().flat_map(|p| self.free_type_vars(p)).collect();
                vars.extend(self.free_type_vars(ret));
                vars
            }
            Type::Dictionary(k, v) => {
                let mut vars = self.free_type_vars(k);
                vars.extend(self.free_type_vars(v));
                vars
            }
            Type::Async(inner) => self.free_type_vars(inner),
            Type::Reference(inner) | Type::MutableReference(inner) => self.free_type_vars(inner),
            Type::Pointer(inner) | Type::ConstPointer(inner) => self.free_type_vars(inner),
            Type::TypeConstructor(_, args) => {
                args.iter().flat_map(|a| self.free_type_vars(a)).collect()
            }
            _ => vec![],
        }
    }
}

impl Default for TypeInferrer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unify_integers() {
        let result = unify(&Type::Int, &Type::Int);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unify_type_var() {
        let var = Type::Var("a".to_string());
        let int = Type::Int;
        let result = unify(&var, &int);
        assert!(result.is_ok());
        let subst = result.unwrap();
        assert_eq!(subst.apply(&var), int);
    }

    #[test]
    fn test_occurs_check() {
        let var = "a".to_string();
        let ty = Type::Array(Box::new(Type::Var(var.clone())));
        assert!(occurs_check(&var, &ty));
    }

    #[test]
    fn test_instantiate() {
        let mut inferrer = TypeInferrer::new();
        let scheme = TypeScheme::new(
            vec!["a".to_string()],
            Type::TypeConstructor("Option".to_string(), vec![Type::Var("a".to_string())]),
        );
        let instantiated = inferrer.instantiate(&scheme);
        match instantiated {
            Type::TypeConstructor(name, args) => {
                assert_eq!(name, "Option");
                assert_eq!(args.len(), 1);
                assert!(matches!(&args[0], Type::Var(v) if v.starts_with("?t")));
            }
            _ => panic!("Expected TypeConstructor"),
        }
    }

    #[test]
    fn test_substitution_compose() {
        let s1 = Substitution::singleton("a".to_string(), Type::Int);
        let s2 = Substitution::singleton("b".to_string(), Type::Var("a".to_string()));
        let composed = s1.compose(&s2);
        assert_eq!(composed.mapping.get("a"), Some(&Type::Int));
        assert_eq!(composed.mapping.get("b"), Some(&Type::Var("a".to_string())));

        let result = composed.apply(&Type::Var("b".to_string()));
        assert_eq!(result, Type::Int);
    }
}
