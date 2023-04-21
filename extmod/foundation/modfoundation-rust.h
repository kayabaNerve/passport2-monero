
// SPDX-FileCopyrightText:  © 2023 Foundation Devices, Inc. <hello@foundationdevices.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#include <string.h>
#include "py/obj.h"
#include "py/objexcept.h"
#include "foundation.h"

/// package: foundation.rust

/// def alloc_setup():
///     """
///     """
STATIC mp_obj_t mod_foundation_rust_alloc_setup()
{
    setup_alloc();
    return mp_const_true;
}
STATIC MP_DEFINE_CONST_FUN_OBJ_0(mod_foundation_rust_alloc_setup_obj, mod_foundation_rust_alloc_setup);

STATIC const mp_rom_map_elem_t mod_foundation_rust_globals_table[] = {
    // This should be setup_alloc as well, yet that fails to compile?
    {MP_ROM_QSTR(MP_QSTR_alloc_setup), MP_ROM_PTR(&mod_foundation_rust_alloc_setup_obj)},
};
STATIC MP_DEFINE_CONST_DICT(mod_foundation_rust_globals, mod_foundation_rust_globals_table);

STATIC const mp_obj_module_t mod_foundation_rust_module = {
    .base = {&mp_type_module},
    .globals = (mp_obj_dict_t *)&mod_foundation_rust_globals,
};
