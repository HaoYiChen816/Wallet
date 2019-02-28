// Copyright © 2017-2018 Trust.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

import Foundation

public final class Wanchain: Ethereum {
    public override var chainID: Int {
        return 1
    }

    public override var coinType: CoinType {
        return .wanChain
    }
}
