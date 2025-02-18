// SPDX-FileCopyrightText: © 2022 Foundation Devices, Inc. <hello@foundationdevices.com>
// SPDX-License-Identifier: GPL-3.0-or-later
//

#if defined(LV_LVGL_H_INCLUDE_SIMPLE)
#include "lvgl.h"
#else
#include "lvgl/lvgl.h"
#endif


#ifndef LV_ATTRIBUTE_MEM_ALIGN
#define LV_ATTRIBUTE_MEM_ALIGN
#endif

#ifndef LV_ATTRIBUTE_IMG_ICON_EDIT1
#define LV_ATTRIBUTE_IMG_ICON_EDIT1
#endif

const LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST LV_ATTRIBUTE_IMG_ICON_EDIT1 uint8_t ICON_EDIT1_map[] = {
  0x00, 0x00, 0x00, 0x00, 	/*Color of index 0*/
  0xfe, 0xfe, 0xfe, 0x8f, 	/*Color of index 1*/

  0x00, 0x00, 0x00, 
  0x00, 0x07, 0xc0, 
  0x00, 0x0f, 0xe0, 
  0x00, 0x1e, 0x60, 
  0x00, 0x3c, 0x60, 
  0x00, 0x78, 0xe0, 
  0x00, 0xf1, 0xe0, 
  0x01, 0xe3, 0xc0, 
  0x03, 0xc7, 0x80, 
  0x07, 0x8f, 0x00, 
  0x0f, 0x1e, 0x00, 
  0x1e, 0x3c, 0x00, 
  0x3c, 0x78, 0x00, 
  0x38, 0xf0, 0x00, 
  0x71, 0xe0, 0x00, 
  0x73, 0xc0, 0x00, 
  0x6f, 0x80, 0x00, 
  0x7f, 0x00, 0x00, 
  0xfc, 0x00, 0x00, 
  0x40, 0x00, 0x00, 
};

const lv_img_dsc_t ICON_EDIT1 = {
  .header.cf = LV_IMG_CF_INDEXED_1BIT,
  .header.always_zero = 0,
  .header.reserved = 0,
  .header.w = 20,
  .header.h = 20,
  .data_size = 68,
  .data = ICON_EDIT1_map,
};
