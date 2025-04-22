#![verus::trusted]
use vstd::map::*;
use vstd::modes::*;
use vstd::multiset::*;
use vstd::seq::*;
use vstd::set::*;

use vstd::pervasive::*;
use builtin::*;
use builtin_macros::*;

use crate::abstract_end_point_t::*;

verus! {
broadcast use vstd::seq_lib::group_seq_properties,
              vstd::set_lib::group_set_properties,
              vstd::map_lib::group_map_properties,
              vstd::multiset::group_multiset_properties;

pub enum SingleMessage<MT> {
    Message {
        seqno: nat,
        dst: AbstractEndPoint,
        m: MT,
    },
    Ack {
        ack_seqno: nat,
    }, // I have received everything up to and including seqno
    InvalidMessage {}, // ... what parse returns for raw messages we can't otherwise parse into a valid message above
}

}
