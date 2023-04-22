// SPDX-FileCopyrightText:  © 2023 Foundation Devices, Inc. <hello@foundationdevices.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#include <string.h>
#include "py/obj.h"
#include "py/objexcept.h"
#include "foundation.h"

/// package: foundation.monero

/// def new_seed(entropy: bytes) -> bytes:
///     """
///     """
STATIC mp_obj_t mod_foundation_monero_new_seed(mp_obj_t entropy_obj)
{
    mp_buffer_info_t entropy = {0};
    mp_get_buffer_raise(entropy_obj, &entropy, MP_BUFFER_READ);
    // Ensure 32-bytes of entropy were provided
    if (entropy.len != 32) {
        return mp_const_false;
    }

    vstr_t vstr;
    // Allocate 32-bytes to store the seed
    vstr_init(&vstr, 32);
    size_t res_len = monero_seed_new((uint8_t (*)[32]) entropy.buf, (uint8_t *) vstr.buf, vstr.alloc);
    if (res_len == 0) {
      return mp_const_false;
    }
    vstr.len = res_len;
    return mp_obj_new_str_from_vstr(&mp_type_bytes, &vstr);
}
STATIC MP_DEFINE_CONST_FUN_OBJ_1(mod_foundation_monero_new_seed_obj, mod_foundation_monero_new_seed);

/// def seed_to_string(seed: bytes) -> str:
///     """
///     """
STATIC mp_obj_t mod_foundation_monero_seed_to_string(mp_obj_t seed_obj)
{
    mp_buffer_info_t seed = {0};
    mp_get_buffer_raise(seed_obj, &seed, MP_BUFFER_READ);
    // Ensure 32-bytes were provided for a seed
    if (seed.len != 32) {
        return mp_const_false;
    }

    vstr_t vstr;
    // Allocate 256-bytes (+1 for any null terminator) to store the seed string
    vstr_init(&vstr, 256 + 1);
    size_t res_len = monero_seed_to_string((uint8_t (*)[32]) seed.buf, (uint8_t *) vstr.buf, vstr.alloc);
    if (res_len == 0) {
      return mp_const_false;
    }
    vstr.len = res_len;
    return mp_obj_new_str_from_vstr(&mp_type_str, &vstr);
}
STATIC MP_DEFINE_CONST_FUN_OBJ_1(mod_foundation_monero_seed_to_string_obj, mod_foundation_monero_seed_to_string);

/*
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
*/

STATIC const mp_rom_map_elem_t mod_foundation_monero_globals_table[] = {
    //{MP_ROM_QSTR(MP_QSTR_verify_address), MP_ROM_PTR(&mod_foundation_monero_verify_address_obj)},
    {MP_ROM_QSTR(MP_QSTR_new_seed), MP_ROM_PTR(&mod_foundation_monero_new_seed_obj)},
    {MP_ROM_QSTR(MP_QSTR_seed_to_string), MP_ROM_PTR(&mod_foundation_monero_seed_to_string_obj)},
};
STATIC MP_DEFINE_CONST_DICT(mod_foundation_monero_globals, mod_foundation_monero_globals_table);

STATIC const mp_obj_module_t mod_foundation_monero_module = {
    .base = {&mp_type_module},
    .globals = (mp_obj_dict_t *)&mod_foundation_monero_globals,
};
