// SPDX-FileCopyrightText:  © 2023 Foundation Devices, Inc. <hello@foundationdevices.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#include <string.h>
#include "py/obj.h"
#include "py/objexcept.h"
#include "foundation.h"

/// package: foundation.monero

/// def verify_address(address: str) -> bool:
///     """
///     """
STATIC mp_obj_t mod_foundation_monero_verify_address(mp_obj_t address_obj)
{
    mp_check_self(mp_obj_is_str(address_obj));
    GET_STR_DATA_LEN(address_obj, address, address_len);

    return mod_foundation_monero_verify_address(address, address_len) ? mp_const_true : mp_const_false;
}
STATIC MP_DEFINE_CONST_FUN_OBJ_1(mod_foundation_monero_verify_address_obj, mod_foundation_monero_verify_address);

STATIC const mp_rom_map_elem_t mod_foundation_monero_globals_table[] = {
    {MP_ROM_QSTR(MP_QSTR_verify_address), MP_ROM_PTR(&mod_foundation_monero_verify_address_obj)},
};
STATIC MP_DEFINE_CONST_DICT(mod_foundation_monero_globals, mod_foundation_monero_globals_table);

STATIC const mp_obj_module_t mod_foundation_monero_module = {
    .base = {&mp_type_module},
    .globals = (mp_obj_dict_t *)&mod_foundation_monero_globals,
};
*/
