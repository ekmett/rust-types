// extern crate tailcall;

use std::rc::Rc;
use std::borrow::Borrow;
use list::{self, *};
use name::*;

pub type Ix = u32;
pub type Lvl = u32;

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Tm {
  Var(Ix),
  App(Term,Term),
  Lam(Name,Term)
}

#[derive(Debug,PartialEq,Eq,Clone)]
#[repr(transparent)]
pub struct Term(Rc<Tm>);

impl Borrow<Tm> for Term { #[inline] fn borrow(&self) -> &Tm { self.0.borrow() } }
impl AsRef<Tm> for Term { #[inline] fn as_ref(&self) -> &Tm { self.0.borrow() } }
impl Unpin for Term {}

#[inline]
pub fn lam(n: Name, b: Term) -> Term { Term(Rc::new(Tm::Lam(n,b))) }
#[inline]
pub fn app(f: Term, a: Term) -> Term { Term(Rc::new(Tm::App(f,a))) }
#[inline]
pub fn var(i: Ix) -> Term { Term(Rc::new(Tm::Var(i))) }

pub type Env = List<Value>;

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Val {
  Lam(Env,Name,Term),
  Var(Lvl,Spine)
}

#[derive(Debug,PartialEq,Eq,Clone)]
#[repr(transparent)]
pub struct Value(Rc<Val>);

impl Borrow<Val> for Value { #[inline] fn borrow(&self) -> &Val { self.0.borrow() } }
impl AsRef<Val> for Value { #[inline] fn as_ref(&self) -> &Val { self.0.borrow() } }
impl Unpin for Value {}

#[inline]
pub fn vlam(e: &Env, n: Name, b: &Term) -> Value { Value(Rc::new(Val::Lam(e.clone(),n,b.clone()))) }
pub fn vvar(lvl: Lvl, s: &Spine) -> Value { Value(Rc::new(Val::Var(lvl,s.clone()))) }

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Sp {
  App(Spine,Value)
}

#[derive(Debug,PartialEq,Eq,Clone)]
#[repr(transparent)]
pub struct Spine(Option<Rc<Sp>>);
// impl Borrow<Sp> for Spine { #[inline] fn borrow(&self) -> &Sp { self.0.borrow() } }
// impl AsRef<Sp> for Spine { #[inline] fn as_ref(&self) -> &Sp { self.0.borrow() } }
// impl Unpin for Spine {}

pub fn sapp(s: &Spine, v: &Value) -> Spine { Spine(Some(Rc::new(Sp::App(s.clone(),v.clone())))) }
pub const fn snil() -> Spine { Spine(None) }

#[inline]
pub fn apply(fun: &Value, arg: Value) -> Value {
  match fun.borrow() {
    Val::Lam(e,_,b) => eval(&cons(arg,e.clone()),b),
    Val::Var(n,s) => vvar(*n,&sapp(s,&arg))
  }
}

pub fn lookup(_e: &Env, _i: Ix) -> Value {
  panic!("not written")
}

pub fn eval(e: &Env, t: &Term) -> Value {
  match t.borrow() {
    Tm::Var(i) => { lookup(e,*i) }
    Tm::App(f,x) => { 
       let fv = eval(e,f);
       let xv = eval(e,x);
       apply(&fv,xv)
    }
    Tm::Lam(n,b) => { vlam(e,*n,b) }
  }
}

pub fn main() {
  let mut names = Names::new();
  let x = names.get_or_intern("x");
  let y = names.get_or_intern("y");
  let ref empty_env = nil();
  let i = eval(empty_env,&lam(x,var(0)));
  let k = eval(empty_env,&lam(x,lam(y,var(1))));
  let ref ki_env = list::list![k,i];
  let ki = eval(ki_env,&app(var(0),var(1)));
  println!("{:#?}",ki);
}