#![verus::trusted]
//! Translates file Distributed/Protocol/SHT/Message.i.dfy
use builtin::*;
use builtin_macros::*;

use crate::abstract_end_point_t::*;
use crate::app_interface_t::*;
use crate::keys_t::*;
use crate::network_t::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub enum Message {
    GetRequest {
        key: AbstractKey,
    },
    SetRequest {
        key: AbstractKey,
        value: Option<AbstractValue>,
    },
    Reply {
        key: AbstractKey,
        value: Option<AbstractValue>,
    },
    Redirect {
        key: AbstractKey,
        id: AbstractEndPoint,
    },
    Shard {
        range: KeyRange<AbstractKey>,
        recipient: AbstractEndPoint,
    },
    Delegate {
        range: KeyRange<AbstractKey>,
        h: Hashtable,
    },
}

}
