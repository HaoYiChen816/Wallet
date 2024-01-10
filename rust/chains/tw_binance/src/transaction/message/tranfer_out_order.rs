// Copyright © 2017-2024 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use crate::address::BinanceAddress;
use crate::amino::AminoEncoder;
use crate::transaction::message::{BinanceMessage, TWBinanceProto, Token};
use serde::{Deserialize, Serialize};
use tw_coin_entry::coin_context::CoinContext;
use tw_coin_entry::coin_entry::CoinAddress;
use tw_coin_entry::error::{SigningError, SigningErrorType, SigningResult};
use tw_evm::address::Address as EthereumAddress;
use tw_hash::H160;
use tw_memory::Data;
use tw_proto::Binance::Proto;

#[derive(Deserialize, Serialize)]
pub struct TransferOutOrder {
    pub amount: Token,
    pub expire_time: i64,
    pub from: BinanceAddress,
    pub to: EthereumAddress,
}

impl TransferOutOrder {
    /// cbindgen:ignore
    pub const PREFIX: [u8; 4] = [0x80, 0x08, 0x19, 0xC0];
}

impl BinanceMessage for TransferOutOrder {
    fn to_amino_protobuf(&self) -> SigningResult<Data> {
        Ok(AminoEncoder::new(&Self::PREFIX)
            .extend_with_msg(&self.to_tw_proto())?
            .encode())
    }
}

impl TWBinanceProto for TransferOutOrder {
    type Proto<'a> = Proto::TransferOut<'a>;

    fn from_tw_proto(coin: &dyn CoinContext, msg: &Self::Proto<'_>) -> SigningResult<Self> {
        let from = BinanceAddress::from_key_hash_with_coin(coin, msg.from.to_vec())?;

        let to_bytes = H160::try_from(msg.to.as_ref())
            .map_err(|_| SigningError(SigningErrorType::Error_invalid_address))?;
        let to = EthereumAddress::from_bytes(to_bytes);

        let amount_proto = msg
            .amount
            .as_ref()
            .ok_or(SigningError(SigningErrorType::Error_invalid_params))?;

        Ok(TransferOutOrder {
            from,
            to,
            amount: Token::from_tw_proto(amount_proto),
            expire_time: msg.expire_time,
        })
    }

    fn to_tw_proto(&self) -> Self::Proto<'static> {
        Proto::TransferOut {
            from: self.from.data().into(),
            to: self.to.data().into(),
            amount: Some(self.amount.to_tw_proto()),
            expire_time: self.expire_time,
        }
    }
}
