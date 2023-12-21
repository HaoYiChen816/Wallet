// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use tw_evm::message::eip712::message_types::MessageTypesBuilder;
use tw_evm::message::eip712::property::PropertyType;

/// Represents an amount type that belongs to a particular order.
pub struct TypeMsgAmount;

impl TypeMsgAmount {
    pub fn eip712_type(msg_idx: usize) -> String {
        format!("TypeMsg{msg_idx}Amount")
    }

    pub fn declare_eip712_type(msg_idx: usize, message_types: &mut MessageTypesBuilder) {
        let type_msg_amount_type = Self::eip712_type(msg_idx);
        if let Some(mut builder) = message_types.add_custom_type(type_msg_amount_type) {
            builder.add_property("amount", PropertyType::String);
            builder.add_property("denom", PropertyType::String);
        }
    }
}
