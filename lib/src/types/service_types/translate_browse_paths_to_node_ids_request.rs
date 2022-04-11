// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock
//
// This file was autogenerated from Opc.Ua.Types.bsd by tools/schema/gen_types.js
//
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
use std::io::{Read, Write};
#[allow(unused_imports)]
use crate::types::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    request_header::RequestHeader,
    service_types::BrowsePath,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TranslateBrowsePathsToNodeIdsRequest {
    pub request_header: RequestHeader,
    pub browse_paths: Option<Vec<BrowsePath>>,
}

impl MessageInfo for TranslateBrowsePathsToNodeIdsRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::TranslateBrowsePathsToNodeIdsRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<TranslateBrowsePathsToNodeIdsRequest> for TranslateBrowsePathsToNodeIdsRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += byte_len_array(&self.browse_paths);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += write_array(stream, &self.browse_paths)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_options)?;
        let browse_paths: Option<Vec<BrowsePath>> = read_array(stream, decoding_options)?;
        Ok(TranslateBrowsePathsToNodeIdsRequest {
            request_header,
            browse_paths,
        })
    }
}