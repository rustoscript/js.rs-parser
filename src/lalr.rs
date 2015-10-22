#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use ast::{BinOp, Exp, Stmt};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Exp {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{BinOp, Exp, Stmt};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Exp<
        'input,
    >(
        input: &'input str,
    ) -> Result<Exp, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Exp(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        AddOp(BinOp),
        Exp(Exp),
        Float(f64),
        MulExp(Exp),
        MulOp(BinOp),
        Stmt(Stmt),
        Term(Exp),
        Var(String),
        ____Exp(Exp),
        ____Stmt(Stmt),
    }

    // State 0
    //   Exp = (*) Exp AddOp MulExp [EOF]
    //   Exp = (*) Exp AddOp MulExp ["+"]
    //   Exp = (*) Exp AddOp MulExp ["-"]
    //   Exp = (*) MulExp [EOF]
    //   Exp = (*) MulExp ["+"]
    //   Exp = (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [EOF]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   MulExp = (*) MulExp MulOp Term [EOF]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) MulExp MulOp Term ["/"]
    //   MulExp = (*) Term [EOF]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   MulExp = (*) Term ["/"]
    //   Term = (*) Float [EOF]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Var [EOF]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) "(" Exp ")" [EOF]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [EOF]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //   __Exp = (*) Exp [EOF]
    //
    //   "(" -> Shift(S6)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S7)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S8)
    //
    //   Exp -> S1
    //   Float -> S2
    //   MulExp -> S3
    //   Term -> S4
    //   Var -> S5
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Exp(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Float(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   AddOp = (*) "+" ["("]
    //   AddOp = (*) "+" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "+" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   AddOp = (*) "-" ["("]
    //   AddOp = (*) "-" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "-" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   Exp = Exp (*) AddOp MulExp [EOF]
    //   Exp = Exp (*) AddOp MulExp ["+"]
    //   Exp = Exp (*) AddOp MulExp ["-"]
    //   __Exp = Exp (*) [EOF]
    //
    //   EOF -> Reduce(__Exp = Exp => Call(ActionFn(1));)
    //   "+" -> Shift(S10)
    //   "-" -> Shift(S11)
    //
    //   AddOp -> S9
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action1(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Exp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AddOp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 2
    //   Term = Float (*) [EOF]
    //   Term = Float (*) ["*"]
    //   Term = Float (*) ["+"]
    //   Term = Float (*) ["-"]
    //   Term = Float (*) ["/"]
    //
    //   EOF -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "*" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "+" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "-" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "/" -> Reduce(Term = Float => Call(ActionFn(12));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<f64>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 3
    //   Exp = MulExp (*) [EOF]
    //   Exp = MulExp (*) ["+"]
    //   Exp = MulExp (*) ["-"]
    //   MulExp = MulExp (*) MulOp Term [EOF]
    //   MulExp = MulExp (*) MulOp Term ["*"]
    //   MulExp = MulExp (*) MulOp Term ["+"]
    //   MulExp = MulExp (*) MulOp Term ["-"]
    //   MulExp = MulExp (*) MulOp Term ["/"]
    //   MulOp = (*) "*" ["("]
    //   MulOp = (*) "*" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "*" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   MulOp = (*) "/" ["("]
    //   MulOp = (*) "/" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "/" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   EOF -> Reduce(Exp = MulExp => Call(ActionFn(5));)
    //   "*" -> Shift(S13)
    //   "+" -> Reduce(Exp = MulExp => Call(ActionFn(5));)
    //   "-" -> Reduce(Exp = MulExp => Call(ActionFn(5));)
    //   "/" -> Shift(S14)
    //
    //   MulOp -> S12
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Exp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::MulOp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 4
    //   MulExp = Term (*) [EOF]
    //   MulExp = Term (*) ["*"]
    //   MulExp = Term (*) ["+"]
    //   MulExp = Term (*) ["-"]
    //   MulExp = Term (*) ["/"]
    //
    //   EOF -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "*" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "+" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "-" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "/" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MulExp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   Term = Var (*) [EOF]
    //   Term = Var (*) ["*"]
    //   Term = Var (*) ["+"]
    //   Term = Var (*) ["-"]
    //   Term = Var (*) ["/"]
    //
    //   EOF -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "*" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "+" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "-" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "/" -> Reduce(Term = Var => Call(ActionFn(13));)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action13(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 6
    //   Exp = (*) Exp AddOp MulExp [")"]
    //   Exp = (*) Exp AddOp MulExp ["+"]
    //   Exp = (*) Exp AddOp MulExp ["-"]
    //   Exp = (*) MulExp [")"]
    //   Exp = (*) MulExp ["+"]
    //   Exp = (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [")"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   MulExp = (*) MulExp MulOp Term [")"]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) MulExp MulOp Term ["/"]
    //   MulExp = (*) Term [")"]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   MulExp = (*) Term ["/"]
    //   Term = (*) Float [")"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Var [")"]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) "(" Exp ")" [")"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Term = "(" (*) Exp ")" [EOF]
    //   Term = "(" (*) Exp ")" ["*"]
    //   Term = "(" (*) Exp ")" ["+"]
    //   Term = "(" (*) Exp ")" ["-"]
    //   Term = "(" (*) Exp ")" ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [")"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //
    //   "(" -> Shift(S20)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S21)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S22)
    //
    //   Exp -> S15
    //   Float -> S16
    //   MulExp -> S17
    //   Term -> S18
    //   Var -> S19
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Exp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Float(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state19(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) [EOF]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["*"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["+"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["-"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["/"]
    //
    //   EOF -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "*" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "+" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "-" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "/" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Float(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 8
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) [EOF]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["*"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["+"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["-"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["/"]
    //
    //   EOF -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "*" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "+" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "-" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "/" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action16(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Var(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 9
    //   Exp = Exp AddOp (*) MulExp [EOF]
    //   Exp = Exp AddOp (*) MulExp ["+"]
    //   Exp = Exp AddOp (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [EOF]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   MulExp = (*) MulExp MulOp Term [EOF]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) MulExp MulOp Term ["/"]
    //   MulExp = (*) Term [EOF]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   MulExp = (*) Term ["/"]
    //   Term = (*) Float [EOF]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Var [EOF]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) "(" Exp ")" [EOF]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [EOF]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //
    //   "(" -> Shift(S6)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S7)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S8)
    //
    //   Float -> S2
    //   MulExp -> S23
    //   Term -> S4
    //   Var -> S5
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Float(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 10
    //   AddOp = "+" (*) ["("]
    //   AddOp = "+" (*) [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = "+" (*) [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   "(" -> Reduce(AddOp = "+" => Call(ActionFn(6));)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Reduce(AddOp = "+" => Call(ActionFn(6));)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Reduce(AddOp = "+" => Call(ActionFn(6));)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::AddOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 11
    //   AddOp = "-" (*) ["("]
    //   AddOp = "-" (*) [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = "-" (*) [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   "(" -> Reduce(AddOp = "-" => Call(ActionFn(7));)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Reduce(AddOp = "-" => Call(ActionFn(7));)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Reduce(AddOp = "-" => Call(ActionFn(7));)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::AddOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [EOF]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   MulExp = MulExp MulOp (*) Term [EOF]
    //   MulExp = MulExp MulOp (*) Term ["*"]
    //   MulExp = MulExp MulOp (*) Term ["+"]
    //   MulExp = MulExp MulOp (*) Term ["-"]
    //   MulExp = MulExp MulOp (*) Term ["/"]
    //   Term = (*) Float [EOF]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Var [EOF]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) "(" Exp ")" [EOF]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [EOF]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //
    //   "(" -> Shift(S6)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S7)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S8)
    //
    //   Float -> S2
    //   Term -> S24
    //   Var -> S5
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Float(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 13
    //   MulOp = "*" (*) ["("]
    //   MulOp = "*" (*) [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = "*" (*) [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   "(" -> Reduce(MulOp = "*" => Call(ActionFn(10));)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Reduce(MulOp = "*" => Call(ActionFn(10));)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Reduce(MulOp = "*" => Call(ActionFn(10));)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MulOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 14
    //   MulOp = "/" (*) ["("]
    //   MulOp = "/" (*) [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = "/" (*) [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   "(" -> Reduce(MulOp = "/" => Call(ActionFn(11));)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Reduce(MulOp = "/" => Call(ActionFn(11));)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Reduce(MulOp = "/" => Call(ActionFn(11));)
    //
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MulOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 15
    //   AddOp = (*) "+" ["("]
    //   AddOp = (*) "+" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "+" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   AddOp = (*) "-" ["("]
    //   AddOp = (*) "-" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "-" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   Exp = Exp (*) AddOp MulExp [")"]
    //   Exp = Exp (*) AddOp MulExp ["+"]
    //   Exp = Exp (*) AddOp MulExp ["-"]
    //   Term = "(" Exp (*) ")" [EOF]
    //   Term = "(" Exp (*) ")" ["*"]
    //   Term = "(" Exp (*) ")" ["+"]
    //   Term = "(" Exp (*) ")" ["-"]
    //   Term = "(" Exp (*) ")" ["/"]
    //
    //   ")" -> Shift(S26)
    //   "+" -> Shift(S10)
    //   "-" -> Shift(S11)
    //
    //   AddOp -> S25
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state26(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AddOp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 16
    //   Term = Float (*) [")"]
    //   Term = Float (*) ["*"]
    //   Term = Float (*) ["+"]
    //   Term = Float (*) ["-"]
    //   Term = Float (*) ["/"]
    //
    //   ")" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "*" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "+" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "-" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "/" -> Reduce(Term = Float => Call(ActionFn(12));)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<f64>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 17
    //   Exp = MulExp (*) [")"]
    //   Exp = MulExp (*) ["+"]
    //   Exp = MulExp (*) ["-"]
    //   MulExp = MulExp (*) MulOp Term [")"]
    //   MulExp = MulExp (*) MulOp Term ["*"]
    //   MulExp = MulExp (*) MulOp Term ["+"]
    //   MulExp = MulExp (*) MulOp Term ["-"]
    //   MulExp = MulExp (*) MulOp Term ["/"]
    //   MulOp = (*) "*" ["("]
    //   MulOp = (*) "*" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "*" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   MulOp = (*) "/" ["("]
    //   MulOp = (*) "/" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "/" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   ")" -> Reduce(Exp = MulExp => Call(ActionFn(5));)
    //   "*" -> Shift(S13)
    //   "+" -> Reduce(Exp = MulExp => Call(ActionFn(5));)
    //   "-" -> Reduce(Exp = MulExp => Call(ActionFn(5));)
    //   "/" -> Shift(S14)
    //
    //   MulOp -> S27
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Exp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::MulOp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
    //   MulExp = Term (*) [")"]
    //   MulExp = Term (*) ["*"]
    //   MulExp = Term (*) ["+"]
    //   MulExp = Term (*) ["-"]
    //   MulExp = Term (*) ["/"]
    //
    //   ")" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "*" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "+" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "-" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "/" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MulExp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 19
    //   Term = Var (*) [")"]
    //   Term = Var (*) ["*"]
    //   Term = Var (*) ["+"]
    //   Term = Var (*) ["-"]
    //   Term = Var (*) ["/"]
    //
    //   ")" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "*" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "+" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "-" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "/" -> Reduce(Term = Var => Call(ActionFn(13));)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action13(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 20
    //   Exp = (*) Exp AddOp MulExp [")"]
    //   Exp = (*) Exp AddOp MulExp ["+"]
    //   Exp = (*) Exp AddOp MulExp ["-"]
    //   Exp = (*) MulExp [")"]
    //   Exp = (*) MulExp ["+"]
    //   Exp = (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [")"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   MulExp = (*) MulExp MulOp Term [")"]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) MulExp MulOp Term ["/"]
    //   MulExp = (*) Term [")"]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   MulExp = (*) Term ["/"]
    //   Term = (*) Float [")"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Var [")"]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) "(" Exp ")" [")"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Term = "(" (*) Exp ")" [")"]
    //   Term = "(" (*) Exp ")" ["*"]
    //   Term = "(" (*) Exp ")" ["+"]
    //   Term = "(" (*) Exp ")" ["-"]
    //   Term = "(" (*) Exp ")" ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [")"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //
    //   "(" -> Shift(S20)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S21)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S22)
    //
    //   Exp -> S28
    //   Float -> S16
    //   MulExp -> S17
    //   Term -> S18
    //   Var -> S19
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Exp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Float(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state19(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 21
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) [")"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["*"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["+"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["-"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["/"]
    //
    //   ")" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "*" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "+" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "-" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "/" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Float(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 22
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) [")"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["*"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["+"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["-"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["/"]
    //
    //   ")" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "*" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "+" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "-" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "/" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action16(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Var(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 23
    //   Exp = Exp AddOp MulExp (*) [EOF]
    //   Exp = Exp AddOp MulExp (*) ["+"]
    //   Exp = Exp AddOp MulExp (*) ["-"]
    //   MulExp = MulExp (*) MulOp Term [EOF]
    //   MulExp = MulExp (*) MulOp Term ["*"]
    //   MulExp = MulExp (*) MulOp Term ["+"]
    //   MulExp = MulExp (*) MulOp Term ["-"]
    //   MulExp = MulExp (*) MulOp Term ["/"]
    //   MulOp = (*) "*" ["("]
    //   MulOp = (*) "*" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "*" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   MulOp = (*) "/" ["("]
    //   MulOp = (*) "/" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "/" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   EOF -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(4));)
    //   "*" -> Shift(S13)
    //   "+" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(4));)
    //   "-" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(4));)
    //   "/" -> Shift(S14)
    //
    //   MulOp -> S12
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
        __sym2: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym3));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Exp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::MulOp(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 24
    //   MulExp = MulExp MulOp Term (*) [EOF]
    //   MulExp = MulExp MulOp Term (*) ["*"]
    //   MulExp = MulExp MulOp Term (*) ["+"]
    //   MulExp = MulExp MulOp Term (*) ["-"]
    //   MulExp = MulExp MulOp Term (*) ["/"]
    //
    //   EOF -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "*" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "+" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "-" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "/" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
        __sym2: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MulExp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 25
    //   Exp = Exp AddOp (*) MulExp [")"]
    //   Exp = Exp AddOp (*) MulExp ["+"]
    //   Exp = Exp AddOp (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [")"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   MulExp = (*) MulExp MulOp Term [")"]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) MulExp MulOp Term ["/"]
    //   MulExp = (*) Term [")"]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   MulExp = (*) Term ["/"]
    //   Term = (*) Float [")"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Var [")"]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) "(" Exp ")" [")"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [")"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //
    //   "(" -> Shift(S20)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S21)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S22)
    //
    //   Float -> S16
    //   MulExp -> S29
    //   Term -> S18
    //   Var -> S19
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Float(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state19(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 26
    //   Term = "(" Exp ")" (*) [EOF]
    //   Term = "(" Exp ")" (*) ["*"]
    //   Term = "(" Exp ")" (*) ["+"]
    //   Term = "(" Exp ")" (*) ["-"]
    //   Term = "(" Exp ")" (*) ["/"]
    //
    //   EOF -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "*" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "+" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "-" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "/" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Exp>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 27
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [")"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   MulExp = MulExp MulOp (*) Term [")"]
    //   MulExp = MulExp MulOp (*) Term ["*"]
    //   MulExp = MulExp MulOp (*) Term ["+"]
    //   MulExp = MulExp MulOp (*) Term ["-"]
    //   MulExp = MulExp MulOp (*) Term ["/"]
    //   Term = (*) Float [")"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Var [")"]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) "(" Exp ")" [")"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [")"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //
    //   "(" -> Shift(S20)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S21)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S22)
    //
    //   Float -> S16
    //   Term -> S30
    //   Var -> S19
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Float(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state30(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state19(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 28
    //   AddOp = (*) "+" ["("]
    //   AddOp = (*) "+" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "+" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   AddOp = (*) "-" ["("]
    //   AddOp = (*) "-" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "-" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   Exp = Exp (*) AddOp MulExp [")"]
    //   Exp = Exp (*) AddOp MulExp ["+"]
    //   Exp = Exp (*) AddOp MulExp ["-"]
    //   Term = "(" Exp (*) ")" [")"]
    //   Term = "(" Exp (*) ")" ["*"]
    //   Term = "(" Exp (*) ")" ["+"]
    //   Term = "(" Exp (*) ")" ["-"]
    //   Term = "(" Exp (*) ")" ["/"]
    //
    //   ")" -> Shift(S31)
    //   "+" -> Shift(S10)
    //   "-" -> Shift(S11)
    //
    //   AddOp -> S25
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state31(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AddOp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 29
    //   Exp = Exp AddOp MulExp (*) [")"]
    //   Exp = Exp AddOp MulExp (*) ["+"]
    //   Exp = Exp AddOp MulExp (*) ["-"]
    //   MulExp = MulExp (*) MulOp Term [")"]
    //   MulExp = MulExp (*) MulOp Term ["*"]
    //   MulExp = MulExp (*) MulOp Term ["+"]
    //   MulExp = MulExp (*) MulOp Term ["-"]
    //   MulExp = MulExp (*) MulOp Term ["/"]
    //   MulOp = (*) "*" ["("]
    //   MulOp = (*) "*" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "*" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   MulOp = (*) "/" ["("]
    //   MulOp = (*) "/" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "/" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   ")" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(4));)
    //   "*" -> Shift(S13)
    //   "+" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(4));)
    //   "-" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(4));)
    //   "/" -> Shift(S14)
    //
    //   MulOp -> S27
    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
        __sym2: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Exp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::MulOp(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 30
    //   MulExp = MulExp MulOp Term (*) [")"]
    //   MulExp = MulExp MulOp Term (*) ["*"]
    //   MulExp = MulExp MulOp Term (*) ["+"]
    //   MulExp = MulExp MulOp Term (*) ["-"]
    //   MulExp = MulExp MulOp Term (*) ["/"]
    //
    //   ")" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "*" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "+" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "-" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "/" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //
    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
        __sym2: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MulExp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 31
    //   Term = "(" Exp ")" (*) [")"]
    //   Term = "(" Exp ")" (*) ["*"]
    //   Term = "(" Exp ")" (*) ["+"]
    //   Term = "(" Exp ")" (*) ["-"]
    //   Term = "(" Exp ")" (*) ["/"]
    //
    //   ")" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "*" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "+" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "-" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "/" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //
    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Exp>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Exp::parse_Exp;

mod __parse__Stmt {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{BinOp, Exp, Stmt};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Stmt<
        'input,
    >(
        input: &'input str,
    ) -> Result<Stmt, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Stmt(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        AddOp(BinOp),
        Exp(Exp),
        Float(f64),
        MulExp(Exp),
        MulOp(BinOp),
        Stmt(Stmt),
        Term(Exp),
        Var(String),
        ____Exp(Exp),
        ____Stmt(Stmt),
    }

    // State 0
    //   Stmt = (*) Var "=" Exp ";" [EOF]
    //   Stmt = (*) "var" Var "=" Exp ";" [EOF]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["="]
    //   __Stmt = (*) Stmt [EOF]
    //
    //   "var" -> Shift(S3)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S4)
    //
    //   Stmt -> S1
    //   Var -> S2
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state3(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Stmt(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   __Stmt = Stmt (*) [EOF]
    //
    //   EOF -> Reduce(__Stmt = Stmt => Call(ActionFn(0));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Stmt>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Stmt(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   Stmt = Var (*) "=" Exp ";" [EOF]
    //
    //   "=" -> Shift(S5)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 3
    //   Stmt = "var" (*) Var "=" Exp ";" [EOF]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["="]
    //
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S4)
    //
    //   Var -> S6
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Var(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 4
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["="]
    //
    //   "=" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action16(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Var(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   Exp = (*) Exp AddOp MulExp ["+"]
    //   Exp = (*) Exp AddOp MulExp ["-"]
    //   Exp = (*) Exp AddOp MulExp [";"]
    //   Exp = (*) MulExp ["+"]
    //   Exp = (*) MulExp ["-"]
    //   Exp = (*) MulExp [";"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [";"]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) MulExp MulOp Term ["/"]
    //   MulExp = (*) MulExp MulOp Term [";"]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   MulExp = (*) Term ["/"]
    //   MulExp = (*) Term [";"]
    //   Stmt = Var "=" (*) Exp ";" [EOF]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Float [";"]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) Var [";"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Term = (*) "(" Exp ")" [";"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [";"]
    //
    //   "(" -> Shift(S12)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S13)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S14)
    //
    //   Exp -> S7
    //   Float -> S8
    //   MulExp -> S9
    //   Term -> S10
    //   Var -> S11
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Exp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Float(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 6
    //   Stmt = "var" Var (*) "=" Exp ";" [EOF]
    //
    //   "=" -> Shift(S15)
    //
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 7
    //   AddOp = (*) "+" ["("]
    //   AddOp = (*) "+" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "+" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   AddOp = (*) "-" ["("]
    //   AddOp = (*) "-" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "-" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   Exp = Exp (*) AddOp MulExp ["+"]
    //   Exp = Exp (*) AddOp MulExp ["-"]
    //   Exp = Exp (*) AddOp MulExp [";"]
    //   Stmt = Var "=" Exp (*) ";" [EOF]
    //
    //   "+" -> Shift(S17)
    //   "-" -> Shift(S18)
    //   ";" -> Shift(S19)
    //
    //   AddOp -> S16
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AddOp(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 8
    //   Term = Float (*) ["*"]
    //   Term = Float (*) ["+"]
    //   Term = Float (*) ["-"]
    //   Term = Float (*) ["/"]
    //   Term = Float (*) [";"]
    //
    //   "*" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "+" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "-" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "/" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   ";" -> Reduce(Term = Float => Call(ActionFn(12));)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<f64>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 9
    //   Exp = MulExp (*) ["+"]
    //   Exp = MulExp (*) ["-"]
    //   Exp = MulExp (*) [";"]
    //   MulExp = MulExp (*) MulOp Term ["*"]
    //   MulExp = MulExp (*) MulOp Term ["+"]
    //   MulExp = MulExp (*) MulOp Term ["-"]
    //   MulExp = MulExp (*) MulOp Term ["/"]
    //   MulExp = MulExp (*) MulOp Term [";"]
    //   MulOp = (*) "*" ["("]
    //   MulOp = (*) "*" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "*" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   MulOp = (*) "/" ["("]
    //   MulOp = (*) "/" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "/" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   "*" -> Shift(S21)
    //   "+" -> Reduce(Exp = MulExp => Call(ActionFn(5));)
    //   "-" -> Reduce(Exp = MulExp => Call(ActionFn(5));)
    //   "/" -> Shift(S22)
    //   ";" -> Reduce(Exp = MulExp => Call(ActionFn(5));)
    //
    //   MulOp -> S20
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Exp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::MulOp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 10
    //   MulExp = Term (*) ["*"]
    //   MulExp = Term (*) ["+"]
    //   MulExp = Term (*) ["-"]
    //   MulExp = Term (*) ["/"]
    //   MulExp = Term (*) [";"]
    //
    //   "*" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "+" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "-" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "/" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   ";" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MulExp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 11
    //   Term = Var (*) ["*"]
    //   Term = Var (*) ["+"]
    //   Term = Var (*) ["-"]
    //   Term = Var (*) ["/"]
    //   Term = Var (*) [";"]
    //
    //   "*" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "+" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "-" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "/" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   ";" -> Reduce(Term = Var => Call(ActionFn(13));)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action13(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //   Exp = (*) Exp AddOp MulExp [")"]
    //   Exp = (*) Exp AddOp MulExp ["+"]
    //   Exp = (*) Exp AddOp MulExp ["-"]
    //   Exp = (*) MulExp [")"]
    //   Exp = (*) MulExp ["+"]
    //   Exp = (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [")"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   MulExp = (*) MulExp MulOp Term [")"]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) MulExp MulOp Term ["/"]
    //   MulExp = (*) Term [")"]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   MulExp = (*) Term ["/"]
    //   Term = (*) Float [")"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Var [")"]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) "(" Exp ")" [")"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Term = "(" (*) Exp ")" ["*"]
    //   Term = "(" (*) Exp ")" ["+"]
    //   Term = "(" (*) Exp ")" ["-"]
    //   Term = "(" (*) Exp ")" ["/"]
    //   Term = "(" (*) Exp ")" [";"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [")"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //
    //   "(" -> Shift(S28)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S29)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S30)
    //
    //   Exp -> S23
    //   Float -> S24
    //   MulExp -> S25
    //   Term -> S26
    //   Var -> S27
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state28(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Exp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Float(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 13
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["*"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["+"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["-"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["/"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) [";"]
    //
    //   "*" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "+" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "-" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "/" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   ";" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Float(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 14
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["*"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["+"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["-"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["/"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) [";"]
    //
    //   "*" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "+" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "-" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "/" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   ";" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action16(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Var(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 15
    //   Exp = (*) Exp AddOp MulExp ["+"]
    //   Exp = (*) Exp AddOp MulExp ["-"]
    //   Exp = (*) Exp AddOp MulExp [";"]
    //   Exp = (*) MulExp ["+"]
    //   Exp = (*) MulExp ["-"]
    //   Exp = (*) MulExp [";"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [";"]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) MulExp MulOp Term ["/"]
    //   MulExp = (*) MulExp MulOp Term [";"]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   MulExp = (*) Term ["/"]
    //   MulExp = (*) Term [";"]
    //   Stmt = "var" Var "=" (*) Exp ";" [EOF]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Float [";"]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) Var [";"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Term = (*) "(" Exp ")" [";"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [";"]
    //
    //   "(" -> Shift(S12)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S13)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S14)
    //
    //   Exp -> S31
    //   Float -> S8
    //   MulExp -> S9
    //   Term -> S10
    //   Var -> S11
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Exp(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state31(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                __Nonterminal::Float(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 16
    //   Exp = Exp AddOp (*) MulExp ["+"]
    //   Exp = Exp AddOp (*) MulExp ["-"]
    //   Exp = Exp AddOp (*) MulExp [";"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [";"]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) MulExp MulOp Term ["/"]
    //   MulExp = (*) MulExp MulOp Term [";"]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   MulExp = (*) Term ["/"]
    //   MulExp = (*) Term [";"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Float [";"]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) Var [";"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Term = (*) "(" Exp ")" [";"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [";"]
    //
    //   "(" -> Shift(S12)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S13)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S14)
    //
    //   Float -> S8
    //   MulExp -> S32
    //   Term -> S10
    //   Var -> S11
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Float(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state32(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 17
    //   AddOp = "+" (*) ["("]
    //   AddOp = "+" (*) [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = "+" (*) [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   "(" -> Reduce(AddOp = "+" => Call(ActionFn(6));)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Reduce(AddOp = "+" => Call(ActionFn(6));)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Reduce(AddOp = "+" => Call(ActionFn(6));)
    //
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::AddOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 18
    //   AddOp = "-" (*) ["("]
    //   AddOp = "-" (*) [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = "-" (*) [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   "(" -> Reduce(AddOp = "-" => Call(ActionFn(7));)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Reduce(AddOp = "-" => Call(ActionFn(7));)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Reduce(AddOp = "-" => Call(ActionFn(7));)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::AddOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 19
    //   Stmt = Var "=" Exp ";" (*) [EOF]
    //
    //   EOF -> Reduce(Stmt = Var, "=", Exp, ";" => Call(ActionFn(2));)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Exp>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Stmt(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 20
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [";"]
    //   MulExp = MulExp MulOp (*) Term ["*"]
    //   MulExp = MulExp MulOp (*) Term ["+"]
    //   MulExp = MulExp MulOp (*) Term ["-"]
    //   MulExp = MulExp MulOp (*) Term ["/"]
    //   MulExp = MulExp MulOp (*) Term [";"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Float [";"]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) Var [";"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Term = (*) "(" Exp ")" [";"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [";"]
    //
    //   "(" -> Shift(S12)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S13)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S14)
    //
    //   Float -> S8
    //   Term -> S33
    //   Var -> S11
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Float(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state33(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 21
    //   MulOp = "*" (*) ["("]
    //   MulOp = "*" (*) [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = "*" (*) [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   "(" -> Reduce(MulOp = "*" => Call(ActionFn(10));)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Reduce(MulOp = "*" => Call(ActionFn(10));)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Reduce(MulOp = "*" => Call(ActionFn(10));)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MulOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 22
    //   MulOp = "/" (*) ["("]
    //   MulOp = "/" (*) [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = "/" (*) [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   "(" -> Reduce(MulOp = "/" => Call(ActionFn(11));)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Reduce(MulOp = "/" => Call(ActionFn(11));)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Reduce(MulOp = "/" => Call(ActionFn(11));)
    //
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MulOp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 23
    //   AddOp = (*) "+" ["("]
    //   AddOp = (*) "+" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "+" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   AddOp = (*) "-" ["("]
    //   AddOp = (*) "-" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "-" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   Exp = Exp (*) AddOp MulExp [")"]
    //   Exp = Exp (*) AddOp MulExp ["+"]
    //   Exp = Exp (*) AddOp MulExp ["-"]
    //   Term = "(" Exp (*) ")" ["*"]
    //   Term = "(" Exp (*) ")" ["+"]
    //   Term = "(" Exp (*) ")" ["-"]
    //   Term = "(" Exp (*) ")" ["/"]
    //   Term = "(" Exp (*) ")" [";"]
    //
    //   ")" -> Shift(S35)
    //   "+" -> Shift(S17)
    //   "-" -> Shift(S18)
    //
    //   AddOp -> S34
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state35(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AddOp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 24
    //   Term = Float (*) [")"]
    //   Term = Float (*) ["*"]
    //   Term = Float (*) ["+"]
    //   Term = Float (*) ["-"]
    //   Term = Float (*) ["/"]
    //
    //   ")" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "*" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "+" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "-" -> Reduce(Term = Float => Call(ActionFn(12));)
    //   "/" -> Reduce(Term = Float => Call(ActionFn(12));)
    //
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<f64>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 25
    //   Exp = MulExp (*) [")"]
    //   Exp = MulExp (*) ["+"]
    //   Exp = MulExp (*) ["-"]
    //   MulExp = MulExp (*) MulOp Term [")"]
    //   MulExp = MulExp (*) MulOp Term ["*"]
    //   MulExp = MulExp (*) MulOp Term ["+"]
    //   MulExp = MulExp (*) MulOp Term ["-"]
    //   MulExp = MulExp (*) MulOp Term ["/"]
    //   MulOp = (*) "*" ["("]
    //   MulOp = (*) "*" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "*" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   MulOp = (*) "/" ["("]
    //   MulOp = (*) "/" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "/" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   ")" -> Reduce(Exp = MulExp => Call(ActionFn(5));)
    //   "*" -> Shift(S21)
    //   "+" -> Reduce(Exp = MulExp => Call(ActionFn(5));)
    //   "-" -> Reduce(Exp = MulExp => Call(ActionFn(5));)
    //   "/" -> Shift(S22)
    //
    //   MulOp -> S36
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Exp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::MulOp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 26
    //   MulExp = Term (*) [")"]
    //   MulExp = Term (*) ["*"]
    //   MulExp = Term (*) ["+"]
    //   MulExp = Term (*) ["-"]
    //   MulExp = Term (*) ["/"]
    //
    //   ")" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "*" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "+" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "-" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //   "/" -> Reduce(MulExp = Term => Call(ActionFn(9));)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MulExp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 27
    //   Term = Var (*) [")"]
    //   Term = Var (*) ["*"]
    //   Term = Var (*) ["+"]
    //   Term = Var (*) ["-"]
    //   Term = Var (*) ["/"]
    //
    //   ")" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "*" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "+" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "-" -> Reduce(Term = Var => Call(ActionFn(13));)
    //   "/" -> Reduce(Term = Var => Call(ActionFn(13));)
    //
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action13(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 28
    //   Exp = (*) Exp AddOp MulExp [")"]
    //   Exp = (*) Exp AddOp MulExp ["+"]
    //   Exp = (*) Exp AddOp MulExp ["-"]
    //   Exp = (*) MulExp [")"]
    //   Exp = (*) MulExp ["+"]
    //   Exp = (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [")"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   MulExp = (*) MulExp MulOp Term [")"]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) MulExp MulOp Term ["/"]
    //   MulExp = (*) Term [")"]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   MulExp = (*) Term ["/"]
    //   Term = (*) Float [")"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Var [")"]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) "(" Exp ")" [")"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Term = "(" (*) Exp ")" [")"]
    //   Term = "(" (*) Exp ")" ["*"]
    //   Term = "(" (*) Exp ")" ["+"]
    //   Term = "(" (*) Exp ")" ["-"]
    //   Term = "(" (*) Exp ")" ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [")"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //
    //   "(" -> Shift(S28)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S29)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S30)
    //
    //   Exp -> S37
    //   Float -> S24
    //   MulExp -> S25
    //   Term -> S26
    //   Var -> S27
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state28(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Exp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state37(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Float(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 29
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) [")"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["*"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["+"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["-"]
    //   Float = r#"-?[0-9]+(\\.[0-9]+)?"# (*) ["/"]
    //
    //   ")" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "*" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "+" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "-" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //   "/" -> Reduce(Float = r#"-?[0-9]+(\\.[0-9]+)?"# => Call(ActionFn(15));)
    //
    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Float(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 30
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) [")"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["*"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["+"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["-"]
    //   Var = r#"[A-Za-z_][0-9A-Za-z_]*"# (*) ["/"]
    //
    //   ")" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "*" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "+" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "-" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //   "/" -> Reduce(Var = r#"[A-Za-z_][0-9A-Za-z_]*"# => Call(ActionFn(16));)
    //
    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action16(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Var(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 31
    //   AddOp = (*) "+" ["("]
    //   AddOp = (*) "+" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "+" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   AddOp = (*) "-" ["("]
    //   AddOp = (*) "-" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "-" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   Exp = Exp (*) AddOp MulExp ["+"]
    //   Exp = Exp (*) AddOp MulExp ["-"]
    //   Exp = Exp (*) AddOp MulExp [";"]
    //   Stmt = "var" Var "=" Exp (*) ";" [EOF]
    //
    //   "+" -> Shift(S17)
    //   "-" -> Shift(S18)
    //   ";" -> Shift(S38)
    //
    //   AddOp -> S16
    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AddOp(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym3, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 32
    //   Exp = Exp AddOp MulExp (*) ["+"]
    //   Exp = Exp AddOp MulExp (*) ["-"]
    //   Exp = Exp AddOp MulExp (*) [";"]
    //   MulExp = MulExp (*) MulOp Term ["*"]
    //   MulExp = MulExp (*) MulOp Term ["+"]
    //   MulExp = MulExp (*) MulOp Term ["-"]
    //   MulExp = MulExp (*) MulOp Term ["/"]
    //   MulExp = MulExp (*) MulOp Term [";"]
    //   MulOp = (*) "*" ["("]
    //   MulOp = (*) "*" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "*" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   MulOp = (*) "/" ["("]
    //   MulOp = (*) "/" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "/" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   "*" -> Shift(S21)
    //   "+" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(4));)
    //   "-" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(4));)
    //   "/" -> Shift(S22)
    //   ";" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(4));)
    //
    //   MulOp -> S20
    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
        __sym2: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Exp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::MulOp(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 33
    //   MulExp = MulExp MulOp Term (*) ["*"]
    //   MulExp = MulExp MulOp Term (*) ["+"]
    //   MulExp = MulExp MulOp Term (*) ["-"]
    //   MulExp = MulExp MulOp Term (*) ["/"]
    //   MulExp = MulExp MulOp Term (*) [";"]
    //
    //   "*" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "+" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "-" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "/" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   ";" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //
    pub fn __state33<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
        __sym2: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MulExp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 34
    //   Exp = Exp AddOp (*) MulExp [")"]
    //   Exp = Exp AddOp (*) MulExp ["+"]
    //   Exp = Exp AddOp (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [")"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   MulExp = (*) MulExp MulOp Term [")"]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) MulExp MulOp Term ["/"]
    //   MulExp = (*) Term [")"]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   MulExp = (*) Term ["/"]
    //   Term = (*) Float [")"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Var [")"]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) "(" Exp ")" [")"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [")"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //
    //   "(" -> Shift(S28)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S29)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S30)
    //
    //   Float -> S24
    //   MulExp -> S39
    //   Term -> S26
    //   Var -> S27
    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state28(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Float(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state39(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 35
    //   Term = "(" Exp ")" (*) ["*"]
    //   Term = "(" Exp ")" (*) ["+"]
    //   Term = "(" Exp ")" (*) ["-"]
    //   Term = "(" Exp ")" (*) ["/"]
    //   Term = "(" Exp ")" (*) [";"]
    //
    //   "*" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "+" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "-" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "/" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   ";" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //
    pub fn __state35<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Exp>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 36
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# [")"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["*"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["+"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["-"]
    //   Float = (*) r#"-?[0-9]+(\\.[0-9]+)?"# ["/"]
    //   MulExp = MulExp MulOp (*) Term [")"]
    //   MulExp = MulExp MulOp (*) Term ["*"]
    //   MulExp = MulExp MulOp (*) Term ["+"]
    //   MulExp = MulExp MulOp (*) Term ["-"]
    //   MulExp = MulExp MulOp (*) Term ["/"]
    //   Term = (*) Float [")"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Float ["/"]
    //   Term = (*) Var [")"]
    //   Term = (*) Var ["*"]
    //   Term = (*) Var ["+"]
    //   Term = (*) Var ["-"]
    //   Term = (*) Var ["/"]
    //   Term = (*) "(" Exp ")" [")"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = (*) "(" Exp ")" ["/"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# [")"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["*"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["+"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["-"]
    //   Var = (*) r#"[A-Za-z_][0-9A-Za-z_]*"# ["/"]
    //
    //   "(" -> Shift(S28)
    //   r#"-?[0-9]+(\\.[0-9]+)?"# -> Shift(S29)
    //   r#"[A-Za-z_][0-9A-Za-z_]*"# -> Shift(S30)
    //
    //   Float -> S24
    //   Term -> S40
    //   Var -> S27
    pub fn __state36<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state28(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Float(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state40(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Var(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 37
    //   AddOp = (*) "+" ["("]
    //   AddOp = (*) "+" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "+" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   AddOp = (*) "-" ["("]
    //   AddOp = (*) "-" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   AddOp = (*) "-" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   Exp = Exp (*) AddOp MulExp [")"]
    //   Exp = Exp (*) AddOp MulExp ["+"]
    //   Exp = Exp (*) AddOp MulExp ["-"]
    //   Term = "(" Exp (*) ")" [")"]
    //   Term = "(" Exp (*) ")" ["*"]
    //   Term = "(" Exp (*) ")" ["+"]
    //   Term = "(" Exp (*) ")" ["-"]
    //   Term = "(" Exp (*) ")" ["/"]
    //
    //   ")" -> Shift(S41)
    //   "+" -> Shift(S17)
    //   "-" -> Shift(S18)
    //
    //   AddOp -> S34
    pub fn __state37<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AddOp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 38
    //   Stmt = "var" Var "=" Exp ";" (*) [EOF]
    //
    //   EOF -> Reduce(Stmt = "var", Var, "=", Exp, ";" => Call(ActionFn(3));)
    //
    pub fn __state38<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<Exp>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __nt = super::__action3(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Stmt(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 39
    //   Exp = Exp AddOp MulExp (*) [")"]
    //   Exp = Exp AddOp MulExp (*) ["+"]
    //   Exp = Exp AddOp MulExp (*) ["-"]
    //   MulExp = MulExp (*) MulOp Term [")"]
    //   MulExp = MulExp (*) MulOp Term ["*"]
    //   MulExp = MulExp (*) MulOp Term ["+"]
    //   MulExp = MulExp (*) MulOp Term ["-"]
    //   MulExp = MulExp (*) MulOp Term ["/"]
    //   MulOp = (*) "*" ["("]
    //   MulOp = (*) "*" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "*" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //   MulOp = (*) "/" ["("]
    //   MulOp = (*) "/" [r#"-?[0-9]+(\\.[0-9]+)?"#]
    //   MulOp = (*) "/" [r#"[A-Za-z_][0-9A-Za-z_]*"#]
    //
    //   ")" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(4));)
    //   "*" -> Shift(S21)
    //   "+" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(4));)
    //   "-" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(4));)
    //   "/" -> Shift(S22)
    //
    //   MulOp -> S36
    pub fn __state39<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
        __sym2: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Exp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::MulOp(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 40
    //   MulExp = MulExp MulOp Term (*) [")"]
    //   MulExp = MulExp MulOp Term (*) ["*"]
    //   MulExp = MulExp MulOp Term (*) ["+"]
    //   MulExp = MulExp MulOp Term (*) ["-"]
    //   MulExp = MulExp MulOp Term (*) ["/"]
    //
    //   ")" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "*" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "+" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "-" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //   "/" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(8));)
    //
    pub fn __state40<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Exp>,
        __sym1: &mut Option<BinOp>,
        __sym2: &mut Option<Exp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MulExp(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 41
    //   Term = "(" Exp ")" (*) [")"]
    //   Term = "(" Exp ")" (*) ["*"]
    //   Term = "(" Exp ")" (*) ["+"]
    //   Term = "(" Exp ")" (*) ["-"]
    //   Term = "(" Exp ")" (*) ["/"]
    //
    //   ")" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "*" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "+" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "-" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //   "/" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(14));)
    //
    pub fn __state41<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Exp>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action14(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Stmt::parse_Stmt;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '(' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        ')' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '*' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        '+' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        '-' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '/' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        ';' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '=' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '.' => {
                            __current_state = 13;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        's' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    __0: Stmt,
) -> Stmt
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    __0: Exp,
) -> Exp
{
    (__0)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    __0: String,
    _: &'input str,
    __1: Exp,
    _: &'input str,
) -> Stmt
{
    Stmt::Assign(__0, __1)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    _: &'input str,
    __0: String,
    _: &'input str,
    __1: Exp,
    _: &'input str,
) -> Stmt
{
    Stmt::Decl(__0, __1)
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    e: Exp,
    o: BinOp,
    m: Exp,
) -> Exp
{
    Exp::BinExp(Box::new(e), o, Box::new(m))
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    __0: Exp,
) -> Exp
{
    (__0)
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> BinOp
{
    BinOp::Plus
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> BinOp
{
    BinOp::Minus
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    m: Exp,
    o: BinOp,
    t: Exp,
) -> Exp
{
    Exp::BinExp(Box::new(m), o, Box::new(t))
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: Exp,
) -> Exp
{
    (__0)
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> BinOp
{
    BinOp::Star
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> BinOp
{
    BinOp::Slash
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    __0: f64,
) -> Exp
{
    Exp::Float(__0)
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    __0: String,
) -> Exp
{
    Exp::Var(__0)
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    _: &'input str,
    __0: Exp,
    _: &'input str,
) -> Exp
{
    (__0)
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> f64
{
    f64::from_str(__0).unwrap()
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> String
{
    String::from(__0)
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
