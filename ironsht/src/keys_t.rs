#![verus::trusted]
use builtin::*;
use builtin_macros::*;
use vstd::map::*;
use vstd::modes::*;
use vstd::multiset::*;
use vstd::pervasive::*;
use vstd::prelude::*;
use vstd::seq::*;
use vstd::set::*;


use crate::app_interface_t::*;
use crate::verus_extra::clone_v::*;

verus! {

pub trait KeyTrait : Sized {

    spec fn zero_spec() -> Self where Self: std::marker::Sized;

    proof fn zero_properties()
        ensures
            forall |k:Self| k != Self::zero_spec() ==> (#[trigger] Self::zero_spec().cmp_spec(k)).lt();

    spec fn cmp_spec(self, other: Self) -> Ordering;

    // TODO: you can't have `broadcast group` in a trait
    broadcast proof fn eq_is_equality(a:Self, b:Self)
        ensures #![auto] a == b <==> a.cmp_spec(b).eq()
    {
        // TODO: why it can be automatically verified in `cmp_properties` but not here???
        Self::cmp_properties();
        assert(forall |a:Self, b:Self| #![auto] a == b <==> a.cmp_spec(b).eq());
    }

    broadcast proof fn eq_refl(a:Self)
        ensures #![auto] a.cmp_spec(a).eq()
    {
        Self::cmp_properties();
    }

    broadcast proof fn eq_comm(a:Self, b:Self)
        ensures #![auto] a.cmp_spec(b).eq() == b.cmp_spec(a).eq()
    {
        Self::cmp_properties();
    }

    broadcast proof fn eq_trans(a:Self, b:Self, c:Self)
        ensures #![auto] a.cmp_spec(b).eq() && b.cmp_spec(c).eq() ==> a.cmp_spec(c).eq()
    {
        Self::cmp_properties();
    }

    broadcast proof fn ne_asym(a:Self, b:Self)
        ensures #![auto] a.cmp_spec(b).lt() <==> b.cmp_spec(a).gt()
    {
        Self::cmp_properties();
    }

    broadcast proof fn ne_connected(a:Self, b:Self)
        ensures #![auto] a.cmp_spec(b).ne() ==> a.cmp_spec(b).lt() || b.cmp_spec(a).lt()
    {
        Self::cmp_properties();
    }

    broadcast proof fn trans_lt_lt(a:Self, b:Self, c:Self)
        ensures #![auto] a.cmp_spec(b).lt() && b.cmp_spec(c).lt() ==> a.cmp_spec(c).lt()
    {
        Self::cmp_properties();
    }

    broadcast proof fn trans_lt_le(a:Self, b:Self, c:Self)
        ensures #![auto] a.cmp_spec(b).lt() && b.cmp_spec(c).le() ==> a.cmp_spec(c).lt()
    {
        Self::cmp_properties();
    }

    broadcast proof fn trans_le_lt(a:Self, b:Self, c:Self)
        ensures #![auto] a.cmp_spec(b).le() && b.cmp_spec(c).lt() ==> a.cmp_spec(c).lt()
    {
        Self::cmp_properties();
    }

    proof fn cmp_properties()
        ensures
        // Equality is eq  --- TODO: Without this we need to redefine Seq, Set, etc. operators that use ==
        forall |a:Self, b:Self| #![auto] a == b <==> a.cmp_spec(b).eq(),
        // Reflexivity of equality
        forall |a:Self| #![auto] a.cmp_spec(a).eq(),
        // Commutativity of equality
        forall |a:Self, b:Self| (#[trigger] a.cmp_spec(b)).eq() == b.cmp_spec(a).eq(),
        // Transitivity of equality
        forall |a:Self, b:Self, c:Self|
            #[trigger] a.cmp_spec(b).eq() && #[trigger] b.cmp_spec(c).eq() ==> a.cmp_spec(c).eq(),
        // Inequality is asymmetric
        forall |a:Self, b:Self|
            #[trigger] a.cmp_spec(b).lt() <==> b.cmp_spec(a).gt(),
        // Connected
        forall |a:Self, b:Self|
            #![auto] a.cmp_spec(b).ne() ==> a.cmp_spec(b).lt() || b.cmp_spec(a).lt(),
        // Transitivity of inequality
        forall |a:Self, b:Self, c:Self|
            #[trigger] a.cmp_spec(b).lt() && #[trigger] b.cmp_spec(c).lt() ==> a.cmp_spec(c).lt(),
        forall |a:Self, b:Self, c:Self|
            #[trigger] a.cmp_spec(b).lt() && #[trigger] b.cmp_spec(c).le() ==> a.cmp_spec(c).lt(),
        forall |a:Self, b:Self, c:Self|
            #[trigger] a.cmp_spec(b).le() && #[trigger] b.cmp_spec(c).lt() ==> a.cmp_spec(c).lt();

    // zero should be smaller than all other keys
    fn zero() -> (z: Self)
        ensures z == Self::zero_spec();

    fn cmp(&self, other: &Self) -> (o: Ordering)
        requires true,
        ensures o == self.cmp_spec(*other);
}

// Based on Rust's Ordering
#[derive(Structural, PartialEq, Eq)]
pub enum Ordering {
    Less,
    Equal,
    Greater,
}

pub struct KeyIterator<K: KeyTrait + VerusClone> {
    // None means we hit the end
    pub k: Option<K>,
}

impl<K: KeyTrait + VerusClone> KeyIterator<K> {
    pub open spec fn new_spec(k: K) -> Self {
        KeyIterator { k: Some(k) }
    }

    pub open spec fn cmp_spec(self, other: Self) -> Ordering {
        match (self.k, other.k) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(i), Some(j)) => { i.cmp_spec(j) }
        }
    }

    pub open spec fn lt_spec(self, other: Self) -> bool {
        (!self.k.is_None() && other.k.is_None())
      || (!self.k.is_None() && !other.k.is_None() && self.k.get_Some_0().cmp_spec(other.k.get_Some_0()).lt())
    }

    // TODO: Use the name `spec_ge` instead of `geq_spec` to enable Verus magic overloading
    pub open spec fn geq_spec(self, other: Self) -> bool {
        !self.lt_spec(other) //|| self == other
    }
}

pub struct KeyRange<K: KeyTrait + VerusClone> {
    pub lo: KeyIterator<K>,
    pub hi: KeyIterator<K>,
}

impl<K: KeyTrait + VerusClone> KeyRange<K>
{
    pub open spec fn contains(self, k: K) -> bool
    {
        KeyIterator::<K>::between(self.lo, KeyIterator::<K>::new_spec(k), self.hi)
    }

    pub open spec fn is_empty(self) -> bool
    {
        self.lo.geq_spec(self.hi)
    }
}

impl<K: KeyTrait + VerusClone /*+ PartialOrd*/> KeyRange<K>
{
    pub fn contains_exec(&self, k: &K) -> (b: bool)
    ensures
        b == self.contains(*k),
    {
        let ki = KeyIterator { k: Some(k.clone()) };
        !ki.lt(&self.lo) && ki.lt(&self.hi)
    }
}

impl<K: VerusClone + KeyTrait> VerusClone for KeyIterator<K> {
    fn clone(&self) -> Self {
        KeyIterator {
            k: match &self.k {
                Some(v) => Some(v.clone()),
                None => None,
            },
        }
    }
}

impl<K: VerusClone + KeyTrait> VerusClone for KeyRange<K> {
    fn clone(&self) -> Self {
        KeyRange { lo: self.lo.clone(), hi: self.hi.clone() }
    }
}

#[derive(Eq,PartialEq,Hash)]
pub struct SHTKey {
    pub // workaround
        ukey: u64,
}

impl SHTKey {
    pub fn clone(&self) -> (out: SHTKey)
    ensures out == self
    {
        SHTKey{ ukey: self.ukey }
    }
}

/*
impl std::hash::Hash for SHTKey {
}

impl std::cmp::PartialEq for SHTKey {
}

impl std::cmp::Eq for SHTKey {
}
*/

impl KeyTrait for SHTKey {
    fn zero() -> (z: Self)
    {
        // This assert is necessary due to https://github.com/verus-lang/verus/issues/885
        assert(SHTKey{ukey: 0} == Self::zero_spec()); 
        SHTKey{ukey: 0}
    }

    open spec fn zero_spec() -> Self
    {
        SHTKey{ukey: 0}
    }

    proof fn zero_properties()
    {
        // Maybe this should not be necessary
        assert(forall |k:Self| k != Self::zero_spec() ==> (#[trigger] Self::zero_spec().cmp_spec(k)).lt());
    }

    open spec fn cmp_spec(self, other: Self) -> Ordering
    {
        if self.ukey < other.ukey {
            Ordering::Less
        } else if self.ukey == other.ukey {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }

    proof fn cmp_properties()
//        ensures
//        // Equality is eq  --- TODO: Without this we need to redefine Seq, Set, etc. operators that use ==
//        forall |a:Self, b:Self| #![auto] a == b <==> a.cmp_spec(b).eq(),
//        // Reflexivity of equality
//        forall |a:Self| #![auto] a.cmp_spec(a).eq(),
//        // Commutativity of equality
//        forall |a:Self, b:Self| (#[trigger] a.cmp_spec(b)).eq() == b.cmp_spec(a).eq(),
//        // Transitivity of equality
//        forall |a:Self, b:Self, c:Self|
//            #[trigger] a.cmp_spec(b).eq() && #[trigger] b.cmp_spec(c).eq() ==> a.cmp_spec(c).eq(),
//        // Inequality is asymmetric
//        forall |a:Self, b:Self|
//            #[trigger] a.cmp_spec(b).lt() <==> b.cmp_spec(a).gt(),
//        // Connected
//        forall |a:Self, b:Self|
//            #![auto] a.cmp_spec(b).ne() ==> a.cmp_spec(b).lt() || b.cmp_spec(a).lt(),
//        // Transitivity of inequality
//        forall |a:Self, b:Self, c:Self|
//            #[trigger] a.cmp_spec(b).lt() && #[trigger] b.cmp_spec(c).lt() ==> a.cmp_spec(c).lt(),
//        forall |a:Self, b:Self, c:Self|
//            #[trigger] a.cmp_spec(b).lt() && #[trigger] b.cmp_spec(c).le() ==> a.cmp_spec(c).lt(),
//        forall |a:Self, b:Self, c:Self|
//            #[trigger] a.cmp_spec(b).le() && #[trigger] b.cmp_spec(c).lt() ==> a.cmp_spec(c).lt()
    {
    }

    fn cmp(&self, other: &Self) -> (o: Ordering)
//        requires true,
//        ensures o == self.cmp_spec(*other)
    {
        if self.ukey < other.ukey {
            Ordering::Less
        } else if self.ukey == other.ukey {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl VerusClone for SHTKey {
    fn clone(&self) -> (o: Self)
        //ensures o == self
    {
        SHTKey{ukey: self.ukey}
    }
}

pub type AbstractKey = SHTKey;
pub type CKey = SHTKey;


pub broadcast proof fn eq_is_equality<K: KeyTrait + VerusClone>(a: K, b: K)
    ensures
        #![auto]
        a == b <==> a.cmp_spec(b).eq(),
{
    // TODO: why it can be automatically verified in `cmp_properties` but not here???
    K::cmp_properties();
    assert(forall|a: K, b: K| #![auto] a == b <==> a.cmp_spec(b).eq());
}

pub broadcast proof fn eq_refl<K: KeyTrait + VerusClone>(a: K)
    ensures
        #![auto]
        a.cmp_spec(a).eq(),
{
    K::cmp_properties();
}

pub broadcast proof fn eq_comm<K: KeyTrait + VerusClone>(a: K, b: K)
    ensures
        #![auto]
        a.cmp_spec(b).eq() == b.cmp_spec(a).eq(),
{
    K::cmp_properties();
}

pub broadcast proof fn eq_trans<K: KeyTrait + VerusClone>(a: K, b: K, c: K)
    ensures
        #![auto]
        a.cmp_spec(b).eq() && b.cmp_spec(c).eq() ==> a.cmp_spec(c).eq(),
{
    K::cmp_properties();
}

pub broadcast proof fn ne_asym<K: KeyTrait + VerusClone>(a: K, b: K)
    ensures
        #![auto]
        a.cmp_spec(b).lt() <==> b.cmp_spec(a).gt(),
{
    K::cmp_properties();
}

pub broadcast proof fn ne_connected<K: KeyTrait + VerusClone>(a: K, b: K)
    ensures
        #![auto]
        a.cmp_spec(b).ne() ==> a.cmp_spec(b).lt() || b.cmp_spec(a).lt(),
{
    K::cmp_properties();
}

pub broadcast proof fn trans_lt_lt<K: KeyTrait + VerusClone>(a: K, b: K, c: K)
    ensures
        #![auto]
        a.cmp_spec(b).lt() && b.cmp_spec(c).lt() ==> a.cmp_spec(c).lt(),
{
    K::cmp_properties();
}

pub broadcast proof fn trans_lt_le<K: KeyTrait + VerusClone>(a: K, b: K, c: K)
    ensures
        #![auto]
        a.cmp_spec(b).lt() && b.cmp_spec(c).le() ==> a.cmp_spec(c).lt(),
{
    K::cmp_properties();
}

pub broadcast proof fn trans_le_lt<K: KeyTrait + VerusClone>(a: K, b: K, c: K)
    ensures
        #![auto]
        a.cmp_spec(b).le() && b.cmp_spec(c).lt() ==> a.cmp_spec(c).lt(),
{
    K::cmp_properties();
}

pub broadcast group group_cmp_properties {
    eq_is_equality,
    eq_refl,
    eq_comm,
    eq_trans,
    ne_asym,
    ne_connected,
    trans_lt_lt,
    trans_lt_le,
    trans_le_lt,
}


// Moved from delegation_map_v for dependency cycles introduced by `broadcast` 
impl Ordering {
    pub open spec fn eq(self) -> bool {
        matches!(self, Ordering::Equal)
    }

    pub open spec fn ne(self) -> bool {
        !matches!(self, Ordering::Equal)
    }

    pub open spec fn lt(self) -> bool {
        matches!(self, Ordering::Less)
    }

    pub open spec fn gt(self) -> bool {
        matches!(self, Ordering::Greater)
    }

    pub open spec fn le(self) -> bool {
        !matches!(self, Ordering::Greater)
    }

    pub open spec fn ge(self) -> bool {
        !matches!(self, Ordering::Less)
    }

    pub fn is_eq(self) -> (b: bool)
        ensures
            b == self.eq(),
    {
        matches!(self, Ordering::Equal)
    }

    pub fn is_ne(self) -> (b: bool)
        ensures
            b == self.ne(),
    {
        !matches!(self, Ordering::Equal)
    }

    pub const fn is_lt(self) -> (b: bool)
        ensures
            b == self.lt(),
    {
        matches!(self, Ordering::Less)
    }

    pub const fn is_gt(self) -> (b: bool)
        ensures
            b == self.gt(),
    {
        matches!(self, Ordering::Greater)
    }

    pub const fn is_le(self) -> (b: bool)
        ensures
            b == self.le(),
    {
        !matches!(self, Ordering::Greater)
    }

    pub const fn is_ge(self) -> (b: bool)
        ensures
            b == self.ge(),
    {
        !matches!(self, Ordering::Less)
    }
}

} // verus!


