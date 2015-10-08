#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use ast::{BinOp, Exp};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Exp {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{BinOp, Exp};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Exp<
        'input,
    >(
        input: &'input str,
    ) -> Result<Box<Exp>, __ParseError<usize,(usize, &'input str),()>>
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
        Exp(Box<Exp>),
        Float(f64),
        Int(i64),
        MulExp(Box<Exp>),
        MulOp(BinOp),
        Term(Box<Exp>),
        ____Exp(Box<Exp>),
    }

    // State 0
    //   Exp = (*) Exp AddOp MulExp [EOF]
    //   Exp = (*) Exp AddOp MulExp ["+"]
    //   Exp = (*) Exp AddOp MulExp ["-"]
    //   Exp = (*) MulExp [EOF]
    //   Exp = (*) MulExp ["+"]
    //   Exp = (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# [EOF]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["*"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["+"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["-"]
    //   Int = (*) r#"-?[0-9]+"# [EOF]
    //   Int = (*) r#"-?[0-9]+"# ["*"]
    //   Int = (*) r#"-?[0-9]+"# ["+"]
    //   Int = (*) r#"-?[0-9]+"# ["-"]
    //   MulExp = (*) MulExp MulOp Term [EOF]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) Term [EOF]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   Term = (*) Float [EOF]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Int [EOF]
    //   Term = (*) Int ["*"]
    //   Term = (*) Int ["+"]
    //   Term = (*) Int ["-"]
    //   Term = (*) "(" Exp ")" [EOF]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   __Exp = (*) Exp [EOF]
    //
    //   "(" -> Shift(S6)
    //   r#"-?[0-9]+"# -> Shift(S7)
    //   r#"-?[0-9]+\\.[0-9]+"# -> Shift(S8)
    //
    //   Exp -> S1
    //   Float -> S2
    //   Int -> S3
    //   MulExp -> S4
    //   Term -> S5
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
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (6, __tok0), __loc)) => {
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
                __Nonterminal::Int(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
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
    //   AddOp = (*) "+" [r#"-?[0-9]+"#]
    //   AddOp = (*) "+" [r#"-?[0-9]+\\.[0-9]+"#]
    //   AddOp = (*) "-" ["("]
    //   AddOp = (*) "-" [r#"-?[0-9]+"#]
    //   AddOp = (*) "-" [r#"-?[0-9]+\\.[0-9]+"#]
    //   Exp = Exp (*) AddOp MulExp [EOF]
    //   Exp = Exp (*) AddOp MulExp ["+"]
    //   Exp = Exp (*) AddOp MulExp ["-"]
    //   __Exp = Exp (*) [EOF]
    //
    //   EOF -> Reduce(__Exp = Exp => Call(ActionFn(0));)
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
        __sym0: &mut Option<Box<Exp>>,
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
                let __nt = super::__action0(input, __sym0);
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
    //
    //   EOF -> Reduce(Term = Float => Call(ActionFn(9));)
    //   "*" -> Reduce(Term = Float => Call(ActionFn(9));)
    //   "+" -> Reduce(Term = Float => Call(ActionFn(9));)
    //   "-" -> Reduce(Term = Float => Call(ActionFn(9));)
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
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
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
    //   Term = Int (*) [EOF]
    //   Term = Int (*) ["*"]
    //   Term = Int (*) ["+"]
    //   Term = Int (*) ["-"]
    //
    //   EOF -> Reduce(Term = Int => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = Int => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = Int => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = Int => Call(ActionFn(8));)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i64>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0);
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

    // State 4
    //   Exp = MulExp (*) [EOF]
    //   Exp = MulExp (*) ["+"]
    //   Exp = MulExp (*) ["-"]
    //   MulExp = MulExp (*) MulOp Term [EOF]
    //   MulExp = MulExp (*) MulOp Term ["*"]
    //   MulExp = MulExp (*) MulOp Term ["+"]
    //   MulExp = MulExp (*) MulOp Term ["-"]
    //   MulOp = (*) "*" ["("]
    //   MulOp = (*) "*" [r#"-?[0-9]+"#]
    //   MulOp = (*) "*" [r#"-?[0-9]+\\.[0-9]+"#]
    //
    //   EOF -> Reduce(Exp = MulExp => Call(ActionFn(2));)
    //   "*" -> Shift(S13)
    //   "+" -> Reduce(Exp = MulExp => Call(ActionFn(2));)
    //   "-" -> Reduce(Exp = MulExp => Call(ActionFn(2));)
    //
    //   MulOp -> S12
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Exp>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action2(input, __sym0);
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

    // State 5
    //   MulExp = Term (*) [EOF]
    //   MulExp = Term (*) ["*"]
    //   MulExp = Term (*) ["+"]
    //   MulExp = Term (*) ["-"]
    //
    //   EOF -> Reduce(MulExp = Term => Call(ActionFn(6));)
    //   "*" -> Reduce(MulExp = Term => Call(ActionFn(6));)
    //   "+" -> Reduce(MulExp = Term => Call(ActionFn(6));)
    //   "-" -> Reduce(MulExp = Term => Call(ActionFn(6));)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Exp>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
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

    // State 6
    //   Exp = (*) Exp AddOp MulExp [")"]
    //   Exp = (*) Exp AddOp MulExp ["+"]
    //   Exp = (*) Exp AddOp MulExp ["-"]
    //   Exp = (*) MulExp [")"]
    //   Exp = (*) MulExp ["+"]
    //   Exp = (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# [")"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["*"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["+"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["-"]
    //   Int = (*) r#"-?[0-9]+"# [")"]
    //   Int = (*) r#"-?[0-9]+"# ["*"]
    //   Int = (*) r#"-?[0-9]+"# ["+"]
    //   Int = (*) r#"-?[0-9]+"# ["-"]
    //   MulExp = (*) MulExp MulOp Term [")"]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) Term [")"]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   Term = (*) Float [")"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Int [")"]
    //   Term = (*) Int ["*"]
    //   Term = (*) Int ["+"]
    //   Term = (*) Int ["-"]
    //   Term = (*) "(" Exp ")" [")"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = "(" (*) Exp ")" [EOF]
    //   Term = "(" (*) Exp ")" ["*"]
    //   Term = "(" (*) Exp ")" ["+"]
    //   Term = "(" (*) Exp ")" ["-"]
    //
    //   "(" -> Shift(S19)
    //   r#"-?[0-9]+"# -> Shift(S20)
    //   r#"-?[0-9]+\\.[0-9]+"# -> Shift(S21)
    //
    //   Exp -> S14
    //   Float -> S15
    //   Int -> S16
    //   MulExp -> S17
    //   Term -> S18
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
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state14(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Float(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Int(__nt) => {
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
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   Int = r#"-?[0-9]+"# (*) [EOF]
    //   Int = r#"-?[0-9]+"# (*) ["*"]
    //   Int = r#"-?[0-9]+"# (*) ["+"]
    //   Int = r#"-?[0-9]+"# (*) ["-"]
    //
    //   EOF -> Reduce(Int = r#"-?[0-9]+"# => Call(ActionFn(11));)
    //   "*" -> Reduce(Int = r#"-?[0-9]+"# => Call(ActionFn(11));)
    //   "+" -> Reduce(Int = r#"-?[0-9]+"# => Call(ActionFn(11));)
    //   "-" -> Reduce(Int = r#"-?[0-9]+"# => Call(ActionFn(11));)
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
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Int(__nt)));
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
    //   Float = r#"-?[0-9]+\\.[0-9]+"# (*) [EOF]
    //   Float = r#"-?[0-9]+\\.[0-9]+"# (*) ["*"]
    //   Float = r#"-?[0-9]+\\.[0-9]+"# (*) ["+"]
    //   Float = r#"-?[0-9]+\\.[0-9]+"# (*) ["-"]
    //
    //   EOF -> Reduce(Float = r#"-?[0-9]+\\.[0-9]+"# => Call(ActionFn(12));)
    //   "*" -> Reduce(Float = r#"-?[0-9]+\\.[0-9]+"# => Call(ActionFn(12));)
    //   "+" -> Reduce(Float = r#"-?[0-9]+\\.[0-9]+"# => Call(ActionFn(12));)
    //   "-" -> Reduce(Float = r#"-?[0-9]+\\.[0-9]+"# => Call(ActionFn(12));)
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
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
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

    // State 9
    //   Exp = Exp AddOp (*) MulExp [EOF]
    //   Exp = Exp AddOp (*) MulExp ["+"]
    //   Exp = Exp AddOp (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# [EOF]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["*"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["+"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["-"]
    //   Int = (*) r#"-?[0-9]+"# [EOF]
    //   Int = (*) r#"-?[0-9]+"# ["*"]
    //   Int = (*) r#"-?[0-9]+"# ["+"]
    //   Int = (*) r#"-?[0-9]+"# ["-"]
    //   MulExp = (*) MulExp MulOp Term [EOF]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) Term [EOF]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   Term = (*) Float [EOF]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Int [EOF]
    //   Term = (*) Int ["*"]
    //   Term = (*) Int ["+"]
    //   Term = (*) Int ["-"]
    //   Term = (*) "(" Exp ")" [EOF]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //
    //   "(" -> Shift(S6)
    //   r#"-?[0-9]+"# -> Shift(S7)
    //   r#"-?[0-9]+\\.[0-9]+"# -> Shift(S8)
    //
    //   Float -> S2
    //   Int -> S3
    //   MulExp -> S22
    //   Term -> S5
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Exp>>,
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
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
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
                __Nonterminal::Int(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
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
    //   AddOp = "+" (*) [r#"-?[0-9]+"#]
    //   AddOp = "+" (*) [r#"-?[0-9]+\\.[0-9]+"#]
    //
    //   "(" -> Reduce(AddOp = "+" => Call(ActionFn(3));)
    //   r#"-?[0-9]+"# -> Reduce(AddOp = "+" => Call(ActionFn(3));)
    //   r#"-?[0-9]+\\.[0-9]+"# -> Reduce(AddOp = "+" => Call(ActionFn(3));)
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
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(input, __sym0);
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
    //   AddOp = "-" (*) [r#"-?[0-9]+"#]
    //   AddOp = "-" (*) [r#"-?[0-9]+\\.[0-9]+"#]
    //
    //   "(" -> Reduce(AddOp = "-" => Call(ActionFn(4));)
    //   r#"-?[0-9]+"# -> Reduce(AddOp = "-" => Call(ActionFn(4));)
    //   r#"-?[0-9]+\\.[0-9]+"# -> Reduce(AddOp = "-" => Call(ActionFn(4));)
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
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(input, __sym0);
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
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# [EOF]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["*"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["+"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["-"]
    //   Int = (*) r#"-?[0-9]+"# [EOF]
    //   Int = (*) r#"-?[0-9]+"# ["*"]
    //   Int = (*) r#"-?[0-9]+"# ["+"]
    //   Int = (*) r#"-?[0-9]+"# ["-"]
    //   MulExp = MulExp MulOp (*) Term [EOF]
    //   MulExp = MulExp MulOp (*) Term ["*"]
    //   MulExp = MulExp MulOp (*) Term ["+"]
    //   MulExp = MulExp MulOp (*) Term ["-"]
    //   Term = (*) Float [EOF]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Int [EOF]
    //   Term = (*) Int ["*"]
    //   Term = (*) Int ["+"]
    //   Term = (*) Int ["-"]
    //   Term = (*) "(" Exp ")" [EOF]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //
    //   "(" -> Shift(S6)
    //   r#"-?[0-9]+"# -> Shift(S7)
    //   r#"-?[0-9]+\\.[0-9]+"# -> Shift(S8)
    //
    //   Float -> S2
    //   Int -> S3
    //   Term -> S23
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Exp>>,
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
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
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
                __Nonterminal::Int(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
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
    //   MulOp = "*" (*) [r#"-?[0-9]+"#]
    //   MulOp = "*" (*) [r#"-?[0-9]+\\.[0-9]+"#]
    //
    //   "(" -> Reduce(MulOp = "*" => Call(ActionFn(7));)
    //   r#"-?[0-9]+"# -> Reduce(MulOp = "*" => Call(ActionFn(7));)
    //   r#"-?[0-9]+\\.[0-9]+"# -> Reduce(MulOp = "*" => Call(ActionFn(7));)
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
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0);
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
    //   AddOp = (*) "+" ["("]
    //   AddOp = (*) "+" [r#"-?[0-9]+"#]
    //   AddOp = (*) "+" [r#"-?[0-9]+\\.[0-9]+"#]
    //   AddOp = (*) "-" ["("]
    //   AddOp = (*) "-" [r#"-?[0-9]+"#]
    //   AddOp = (*) "-" [r#"-?[0-9]+\\.[0-9]+"#]
    //   Exp = Exp (*) AddOp MulExp [")"]
    //   Exp = Exp (*) AddOp MulExp ["+"]
    //   Exp = Exp (*) AddOp MulExp ["-"]
    //   Term = "(" Exp (*) ")" [EOF]
    //   Term = "(" Exp (*) ")" ["*"]
    //   Term = "(" Exp (*) ")" ["+"]
    //   Term = "(" Exp (*) ")" ["-"]
    //
    //   ")" -> Shift(S25)
    //   "+" -> Shift(S10)
    //   "-" -> Shift(S11)
    //
    //   AddOp -> S24
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Exp>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state25(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 15
    //   Term = Float (*) [")"]
    //   Term = Float (*) ["*"]
    //   Term = Float (*) ["+"]
    //   Term = Float (*) ["-"]
    //
    //   ")" -> Reduce(Term = Float => Call(ActionFn(9));)
    //   "*" -> Reduce(Term = Float => Call(ActionFn(9));)
    //   "+" -> Reduce(Term = Float => Call(ActionFn(9));)
    //   "-" -> Reduce(Term = Float => Call(ActionFn(9));)
    //
    pub fn __state15<
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
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0);
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

    // State 16
    //   Term = Int (*) [")"]
    //   Term = Int (*) ["*"]
    //   Term = Int (*) ["+"]
    //   Term = Int (*) ["-"]
    //
    //   ")" -> Reduce(Term = Int => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = Int => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = Int => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = Int => Call(ActionFn(8));)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<i64>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0);
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
    //   MulOp = (*) "*" ["("]
    //   MulOp = (*) "*" [r#"-?[0-9]+"#]
    //   MulOp = (*) "*" [r#"-?[0-9]+\\.[0-9]+"#]
    //
    //   ")" -> Reduce(Exp = MulExp => Call(ActionFn(2));)
    //   "*" -> Shift(S13)
    //   "+" -> Reduce(Exp = MulExp => Call(ActionFn(2));)
    //   "-" -> Reduce(Exp = MulExp => Call(ActionFn(2));)
    //
    //   MulOp -> S26
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Exp>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action2(input, __sym0);
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
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
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
    //
    //   ")" -> Reduce(MulExp = Term => Call(ActionFn(6));)
    //   "*" -> Reduce(MulExp = Term => Call(ActionFn(6));)
    //   "+" -> Reduce(MulExp = Term => Call(ActionFn(6));)
    //   "-" -> Reduce(MulExp = Term => Call(ActionFn(6));)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Exp>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
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
    //   Exp = (*) Exp AddOp MulExp [")"]
    //   Exp = (*) Exp AddOp MulExp ["+"]
    //   Exp = (*) Exp AddOp MulExp ["-"]
    //   Exp = (*) MulExp [")"]
    //   Exp = (*) MulExp ["+"]
    //   Exp = (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# [")"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["*"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["+"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["-"]
    //   Int = (*) r#"-?[0-9]+"# [")"]
    //   Int = (*) r#"-?[0-9]+"# ["*"]
    //   Int = (*) r#"-?[0-9]+"# ["+"]
    //   Int = (*) r#"-?[0-9]+"# ["-"]
    //   MulExp = (*) MulExp MulOp Term [")"]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) Term [")"]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   Term = (*) Float [")"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Int [")"]
    //   Term = (*) Int ["*"]
    //   Term = (*) Int ["+"]
    //   Term = (*) Int ["-"]
    //   Term = (*) "(" Exp ")" [")"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //   Term = "(" (*) Exp ")" [")"]
    //   Term = "(" (*) Exp ")" ["*"]
    //   Term = "(" (*) Exp ")" ["+"]
    //   Term = "(" (*) Exp ")" ["-"]
    //
    //   "(" -> Shift(S19)
    //   r#"-?[0-9]+"# -> Shift(S20)
    //   r#"-?[0-9]+\\.[0-9]+"# -> Shift(S21)
    //
    //   Exp -> S27
    //   Float -> S15
    //   Int -> S16
    //   MulExp -> S17
    //   Term -> S18
    pub fn __state19<
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
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym1));
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
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Float(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Int(__nt) => {
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
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 20
    //   Int = r#"-?[0-9]+"# (*) [")"]
    //   Int = r#"-?[0-9]+"# (*) ["*"]
    //   Int = r#"-?[0-9]+"# (*) ["+"]
    //   Int = r#"-?[0-9]+"# (*) ["-"]
    //
    //   ")" -> Reduce(Int = r#"-?[0-9]+"# => Call(ActionFn(11));)
    //   "*" -> Reduce(Int = r#"-?[0-9]+"# => Call(ActionFn(11));)
    //   "+" -> Reduce(Int = r#"-?[0-9]+"# => Call(ActionFn(11));)
    //   "-" -> Reduce(Int = r#"-?[0-9]+"# => Call(ActionFn(11));)
    //
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
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Int(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 21
    //   Float = r#"-?[0-9]+\\.[0-9]+"# (*) [")"]
    //   Float = r#"-?[0-9]+\\.[0-9]+"# (*) ["*"]
    //   Float = r#"-?[0-9]+\\.[0-9]+"# (*) ["+"]
    //   Float = r#"-?[0-9]+\\.[0-9]+"# (*) ["-"]
    //
    //   ")" -> Reduce(Float = r#"-?[0-9]+\\.[0-9]+"# => Call(ActionFn(12));)
    //   "*" -> Reduce(Float = r#"-?[0-9]+\\.[0-9]+"# => Call(ActionFn(12));)
    //   "+" -> Reduce(Float = r#"-?[0-9]+\\.[0-9]+"# => Call(ActionFn(12));)
    //   "-" -> Reduce(Float = r#"-?[0-9]+\\.[0-9]+"# => Call(ActionFn(12));)
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
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0);
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
    //   Exp = Exp AddOp MulExp (*) [EOF]
    //   Exp = Exp AddOp MulExp (*) ["+"]
    //   Exp = Exp AddOp MulExp (*) ["-"]
    //   MulExp = MulExp (*) MulOp Term [EOF]
    //   MulExp = MulExp (*) MulOp Term ["*"]
    //   MulExp = MulExp (*) MulOp Term ["+"]
    //   MulExp = MulExp (*) MulOp Term ["-"]
    //   MulOp = (*) "*" ["("]
    //   MulOp = (*) "*" [r#"-?[0-9]+"#]
    //   MulOp = (*) "*" [r#"-?[0-9]+\\.[0-9]+"#]
    //
    //   EOF -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(1));)
    //   "*" -> Shift(S13)
    //   "+" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(1));)
    //   "-" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(1));)
    //
    //   MulOp -> S12
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Exp>>,
        __sym1: &mut Option<BinOp>,
        __sym2: &mut Option<Box<Exp>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym3));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2);
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

    // State 23
    //   MulExp = MulExp MulOp Term (*) [EOF]
    //   MulExp = MulExp MulOp Term (*) ["*"]
    //   MulExp = MulExp MulOp Term (*) ["+"]
    //   MulExp = MulExp MulOp Term (*) ["-"]
    //
    //   EOF -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(5));)
    //   "*" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(5));)
    //   "+" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(5));)
    //   "-" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(5));)
    //
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Exp>>,
        __sym1: &mut Option<BinOp>,
        __sym2: &mut Option<Box<Exp>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2);
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

    // State 24
    //   Exp = Exp AddOp (*) MulExp [")"]
    //   Exp = Exp AddOp (*) MulExp ["+"]
    //   Exp = Exp AddOp (*) MulExp ["-"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# [")"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["*"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["+"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["-"]
    //   Int = (*) r#"-?[0-9]+"# [")"]
    //   Int = (*) r#"-?[0-9]+"# ["*"]
    //   Int = (*) r#"-?[0-9]+"# ["+"]
    //   Int = (*) r#"-?[0-9]+"# ["-"]
    //   MulExp = (*) MulExp MulOp Term [")"]
    //   MulExp = (*) MulExp MulOp Term ["*"]
    //   MulExp = (*) MulExp MulOp Term ["+"]
    //   MulExp = (*) MulExp MulOp Term ["-"]
    //   MulExp = (*) Term [")"]
    //   MulExp = (*) Term ["*"]
    //   MulExp = (*) Term ["+"]
    //   MulExp = (*) Term ["-"]
    //   Term = (*) Float [")"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Int [")"]
    //   Term = (*) Int ["*"]
    //   Term = (*) Int ["+"]
    //   Term = (*) Int ["-"]
    //   Term = (*) "(" Exp ")" [")"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //
    //   "(" -> Shift(S19)
    //   r#"-?[0-9]+"# -> Shift(S20)
    //   r#"-?[0-9]+\\.[0-9]+"# -> Shift(S21)
    //
    //   Float -> S15
    //   Int -> S16
    //   MulExp -> S28
    //   Term -> S18
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Exp>>,
        __sym1: &mut Option<BinOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Int(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::MulExp(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 25
    //   Term = "(" Exp ")" (*) [EOF]
    //   Term = "(" Exp ")" (*) ["*"]
    //   Term = "(" Exp ")" (*) ["+"]
    //   Term = "(" Exp ")" (*) ["-"]
    //
    //   EOF -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(10));)
    //   "*" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(10));)
    //   "+" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(10));)
    //   "-" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(10));)
    //
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Exp>>,
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
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action10(input, __sym0, __sym1, __sym2);
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

    // State 26
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# [")"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["*"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["+"]
    //   Float = (*) r#"-?[0-9]+\\.[0-9]+"# ["-"]
    //   Int = (*) r#"-?[0-9]+"# [")"]
    //   Int = (*) r#"-?[0-9]+"# ["*"]
    //   Int = (*) r#"-?[0-9]+"# ["+"]
    //   Int = (*) r#"-?[0-9]+"# ["-"]
    //   MulExp = MulExp MulOp (*) Term [")"]
    //   MulExp = MulExp MulOp (*) Term ["*"]
    //   MulExp = MulExp MulOp (*) Term ["+"]
    //   MulExp = MulExp MulOp (*) Term ["-"]
    //   Term = (*) Float [")"]
    //   Term = (*) Float ["*"]
    //   Term = (*) Float ["+"]
    //   Term = (*) Float ["-"]
    //   Term = (*) Int [")"]
    //   Term = (*) Int ["*"]
    //   Term = (*) Int ["+"]
    //   Term = (*) Int ["-"]
    //   Term = (*) "(" Exp ")" [")"]
    //   Term = (*) "(" Exp ")" ["*"]
    //   Term = (*) "(" Exp ")" ["+"]
    //   Term = (*) "(" Exp ")" ["-"]
    //
    //   "(" -> Shift(S19)
    //   r#"-?[0-9]+"# -> Shift(S20)
    //   r#"-?[0-9]+\\.[0-9]+"# -> Shift(S21)
    //
    //   Float -> S15
    //   Int -> S16
    //   Term -> S29
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Exp>>,
        __sym1: &mut Option<BinOp>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym2));
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
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Int(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 27
    //   AddOp = (*) "+" ["("]
    //   AddOp = (*) "+" [r#"-?[0-9]+"#]
    //   AddOp = (*) "+" [r#"-?[0-9]+\\.[0-9]+"#]
    //   AddOp = (*) "-" ["("]
    //   AddOp = (*) "-" [r#"-?[0-9]+"#]
    //   AddOp = (*) "-" [r#"-?[0-9]+\\.[0-9]+"#]
    //   Exp = Exp (*) AddOp MulExp [")"]
    //   Exp = Exp (*) AddOp MulExp ["+"]
    //   Exp = Exp (*) AddOp MulExp ["-"]
    //   Term = "(" Exp (*) ")" [")"]
    //   Term = "(" Exp (*) ")" ["*"]
    //   Term = "(" Exp (*) ")" ["+"]
    //   Term = "(" Exp (*) ")" ["-"]
    //
    //   ")" -> Shift(S30)
    //   "+" -> Shift(S10)
    //   "-" -> Shift(S11)
    //
    //   AddOp -> S24
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Exp>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 28
    //   Exp = Exp AddOp MulExp (*) [")"]
    //   Exp = Exp AddOp MulExp (*) ["+"]
    //   Exp = Exp AddOp MulExp (*) ["-"]
    //   MulExp = MulExp (*) MulOp Term [")"]
    //   MulExp = MulExp (*) MulOp Term ["*"]
    //   MulExp = MulExp (*) MulOp Term ["+"]
    //   MulExp = MulExp (*) MulOp Term ["-"]
    //   MulOp = (*) "*" ["("]
    //   MulOp = (*) "*" [r#"-?[0-9]+"#]
    //   MulOp = (*) "*" [r#"-?[0-9]+\\.[0-9]+"#]
    //
    //   ")" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(1));)
    //   "*" -> Shift(S13)
    //   "+" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(1));)
    //   "-" -> Reduce(Exp = Exp, AddOp, MulExp => Call(ActionFn(1));)
    //
    //   MulOp -> S26
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Exp>>,
        __sym1: &mut Option<BinOp>,
        __sym2: &mut Option<Box<Exp>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2);
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
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 29
    //   MulExp = MulExp MulOp Term (*) [")"]
    //   MulExp = MulExp MulOp Term (*) ["*"]
    //   MulExp = MulExp MulOp Term (*) ["+"]
    //   MulExp = MulExp MulOp Term (*) ["-"]
    //
    //   ")" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(5));)
    //   "*" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(5));)
    //   "+" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(5));)
    //   "-" -> Reduce(MulExp = MulExp, MulOp, Term => Call(ActionFn(5));)
    //
    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Exp>>,
        __sym1: &mut Option<BinOp>,
        __sym2: &mut Option<Box<Exp>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2);
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

    // State 30
    //   Term = "(" Exp ")" (*) [")"]
    //   Term = "(" Exp ")" (*) ["*"]
    //   Term = "(" Exp ")" (*) ["+"]
    //   Term = "(" Exp ")" (*) ["-"]
    //
    //   ")" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(10));)
    //   "*" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(10));)
    //   "+" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(10));)
    //   "-" -> Reduce(Term = "(", Exp, ")" => Call(ActionFn(10));)
    //
    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Box<Exp>>,
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
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action10(input, __sym0, __sym1, __sym2);
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
                        '0' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
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
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
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
                        '.' => {
                            __current_state = 8;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 9;
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
    __0: Box<Exp>,
) -> Box<Exp>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    __0: Box<Exp>,
    __1: BinOp,
    __2: Box<Exp>,
) -> Box<Exp>
{
    Box::new(Exp::BinExp(__0, __1, __2))
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    __0: Box<Exp>,
) -> Box<Exp>
{
    (__0)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> BinOp
{
    BinOp::Plus
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> BinOp
{
    BinOp::Minus
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    __0: Box<Exp>,
    __1: BinOp,
    __2: Box<Exp>,
) -> Box<Exp>
{
    Box::new(Exp::BinExp(__0, __1, __2))
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __0: Box<Exp>,
) -> Box<Exp>
{
    (__0)
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> BinOp
{
    BinOp::Star
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: i64,
) -> Box<Exp>
{
    Box::new(Exp::Int(__0))
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: f64,
) -> Box<Exp>
{
    Box::new(Exp::Float(__0))
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    _: &'input str,
    __0: Box<Exp>,
    _: &'input str,
) -> Box<Exp>
{
    (__0)
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> i64
{
    i64::from_str(__0).unwrap()
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> f64
{
    f64::from_str(__0).unwrap()
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
