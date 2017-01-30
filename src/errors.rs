error_chain! {
  links {}
  foreign_links {
  }
  errors {
    PathMultipleSegments {
      description("path has multiple segments")
      display("path has multiple segments")
    }
    ExprNotAPath {
      description("expr is not a path")
      display("expr is not a path")
    }
    PatNotAPath {
      description("pat is not a path")
      display("pat is not a path")
    }
    ExprNotALit {
      description("expr is not a lit")
      display("expr is not a lit")
    }
    ExprNotAAssign {
      description("expr is not a assign")
      display("expr is not a assign")
    }
    ExprNotATupOrParen {
      description("expr is not a tup or paren")
      display("expr is not a tup or paren")
    }
    AssignValueInvalid {
      description("assign value invalid")
      display("value must be a literal")
    }
    LitNotAnISize {
      description("lit not an isize")
      display("lit not an isize")
    }
    LitNotAString {
      description("lit not a string")
      display("lit not a string")
    }
    ItemNotAFnDecl {
      description("item is not a fndecl")
      display("item is not a fndecl")
    }
    RouteTooLessParam(num: usize) {
      description("route has too less param")
      display("route should not has only {} param", num)
    }
    RouteDuplicatedKey(key: String) {
      description("route has duplicated key")
      display("route has duplicated key '{}'", key)
    }
    RouteKeyUnexpected(key: String) {
      description("route has unexpected key")
      display("route has unexpected key '{}'", key)
    }
    RouteFirstKeyNotPath(key: String) {
      description("first key of route should be 'path'")
      display("the first key, if any, must be 'path', got '{}'", key)
    }
    RouteFirstKeyNotLitOrAssign {
      description(r#"first key of route should be 'path' or 'path = "foo"'"#)
      display(r#"the first key, if any, must be 'path' or 'path = "foo"'"#)
    }
    RouteParamNotFoundInFnInput(param: String) {
      description("route param not found in fn's inputs")
      display("route param '{}' not found in fn's inputs", param)
    }
    FnArgHasNoName {
      description("fnarg has no name")
      display("fn arg has no name")
    }
    FnArgHasNoTy {
      description("fnarg has no ty")
      display("fn arg has no ty")
    }
    LexError(e: ::proc_macro::LexError) {
      description("lex error")
      display("lex error '{:?}'", e)
    }
    FromFormContainTypeParam {
      description("FromForm contains type params")
      display("FromForm does not support type params")
    }
    FromFormOnlySupportNamedStruct {
      description("FromForm only support named struct")
      display("FromForm only support named struct")
    }
    FromFormTooMuchLifetimeParam {
      description("FromForm only support at most 1 lifetime")
      display("FromForm only support at most 1 lifetime")
    }
    ErrorCodeNotInteger {
      description("error code can only be integer")
      display("'code' in #[error(code)] can only be interger")
    }
    ErrorTooMuchParam {
      description("error has too much param")
      display("#[error(code)] can only have 1 param")
    }
    ErrorHandleTooMuchParam {
      description("error handle has too much param")
      display("error handle can only have 2 param at most")
    }
    ErrorHandleContainSelf {
      description("error handle contains self param")
      display("error handle for #[error(code)] can not have self parameter")
    }
    ErrorHandleUnexpectedParam {
      description("error handle has unexpected param")
      display("error handle for #[error(code)] has unexpected argument")
    }
  }
}