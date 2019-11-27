// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    variant::Variant,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryEventFieldList {
    pub event_fields: Option<Vec<Variant>>,
}

impl MessageInfo for HistoryEventFieldList {
    fn object_id(&self) -> ObjectId {
        ObjectId::HistoryEventFieldList_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<HistoryEventFieldList> for HistoryEventFieldList {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.event_fields);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.event_fields)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let event_fields: Option<Vec<Variant>> = read_array(stream, decoding_limits)?;
        Ok(HistoryEventFieldList {
            event_fields,
        })
    }
}
