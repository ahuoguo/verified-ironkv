#![verus::trusted]

use vstd::map::*;
use vstd::modes::*;
use vstd::multiset::*;
use vstd::seq::*;
use vstd::set::*;

use vstd::pervasive::*;
use builtin::*;
use builtin_macros::*;

use crate::app_interface_t::*;
use crate::keys_t::*;
use crate::message_t::*;
use crate::single_message_t::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub enum AppRequest {
    AppGetRequest{seqno:nat, key:AbstractKey},
    AppSetRequest{seqno:nat, key:AbstractKey, ov:Option<AbstractValue>},
}

pub enum AppReply {
    AppReply{g_seqno:nat, key:AbstractKey, ov:Option<AbstractValue>},
}

} // verus
