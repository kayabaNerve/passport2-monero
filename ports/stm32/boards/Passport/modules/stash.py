# SPDX-FileCopyrightText: © 2020 Foundation Devices, Inc. <hello@foundationdevices.com>
# SPDX-License-Identifier: GPL-3.0-or-later
#
# SPDX-FileCopyrightText: 2018 Coinkite, Inc. <coldcardwallet.com>
# SPDX-License-Identifier: GPL-3.0-only
#
# (c) Copyright 2018 by Coinkite Inc. This file is part of Coldcard <coldcardwallet.com>
# and is covered by GPLv3 license found in COPYING.
#
# stash.py - encoding the ultrasecrets: seeds and words
#
import uctypes
import gc
from pincodes import SE_SECRET_LEN

from foundation import monero

def blank_object(item):
    # Use/abuse uctypes to blank objects until python. Will likely
    # even work on immutable types, so be careful. Also works
    # well to kill references to sensitive values (but not copies).
    #
    if isinstance(item, (bytearray, bytes, str)):
        addr, ln = uctypes.addressof(item), len(item)
        buf = uctypes.bytearray_at(addr, ln)
        for i in range(ln):
            buf[i] = 0
    else:
        raise TypeError(item)


# Chip can hold 72-bytes as a secret
# Monero seeds are 32-bytes

class SecretStash:

    @staticmethod
    def encode(master_secret):
        assert len(master_secret) == 32
        # Doesn't support words as it'd be 100 bytes, which is > 72
        return bytearray(master_secret)

    @staticmethod
    def decode(secret):
        # expecting 32-bytes of entropy
        assert len(secret) == 32

        assert monero.seed_to_string(secret) != False

        return 'master', secret


class SensitiveValues:
    # be a context manager, and holder to secrets in-memory

    def __init__(self, secret=None, for_backup=False):
        from common import system

        if secret is None:
            # fetch the secret from bootloader/atecc508a
            from common import pa

            if pa.is_secret_blank():
                raise ValueError('no secrets yet')

            self.secret = pa.fetch()
            self.spots = [self.secret]
        else:
            # sometimes we already know it
            # assert set(secret) != {0}
            self.secret = secret
            self.spots = []


    def __enter__(self):
        import chains

        # raw == secret, yet I'd rather leave raw also set to not risk breaking
        # any code expecting it
        self.mode, self.raw = SecretStash.decode(self.secret)

        self.spots.append(self.raw)

        self.chain = chains.current_chain()

        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        # Clear secrets from memory ... yes, they could have been
        # copied elsewhere, but in normal case, at least we blanked them.
        for item in self.spots:
            blank_object(item)

        if hasattr(self, 'secret'):
            # will be blanked from above
            del self.secret

        # just in case this holds some pointers?
        del self.spots

        # .. and some GC will help too!
        gc.collect()

        if exc_val:
            # An exception happened, but we've done cleanup already now, so
            # not a big deal. Cause it be raised again.
            return False

        return True

    def capture_view_pair(self):
        # track my xpubkey fingerprint & value in settings (not sensitive really)
        # - we share these on any USB connection
        import common
        from common import settings

        # Public spend key, private view key
        view_pair = monero.seed_to_view_pair(self.secret)
        self.view_pair = view_pair
        # print('capture_xpub(): xpub={}'.format(xpub))

        # Always store these volatile - Takes less than 1 second to recreate, and it will change whenever
        # a passphrase is entered, so no need to waste flash cycles on storing it.
        settings.set_volatile('view_pair', view_pair.hex())

        settings.set_volatile('chain', self.chain.ctype)
        settings.set('words', False)

    def register(self, item):
        # Caller can add his own sensitive (derived?) data to our wiper
        # typically would be byte arrays or byte strings, but also
        # supports bip32 nodes
        self.spots.append(item)

    def derive_subaddress(self, account, address, register=True):
        if self.view_pair is None:
            self.view_pair = monero.seed_to_view_pair(self.secret)

        return monero.derive_subaddress(self.view_pair, account, address)

# EOF
