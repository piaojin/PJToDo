/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef app_bindings_h
#define app_bindings_h

/* Generated with cbindgen:0.6.3 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file:
 *   1. Get the latest cbindgen using `cargo install --force cbindgen`
 *      a. Alternatively, you can clone `https://github.com/eqrion/cbindgen` and use a tagged release
 *   2. Run `rustup run nightly cbindgen toolkit/library/rust/ --lockfile Cargo.lock --crate webrender_bindings -o gfx/webrender_bindings/webrender_ffi_generated.h`
 */ 
 
 

#include "struct_heads.h"

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct ToDoTypeInsert ToDoTypeInsert;

typedef struct {
  void *user;
  void (*destroy)(void*);
} PJToDoTypeServiceDelegate;

typedef struct {
  void (*insert_to_do_type)(ToDoTypeInsert);
} ToDoTypeServiceViewModel;

typedef struct {
  PJToDoTypeServiceDelegate delegate;
  ToDoTypeServiceViewModel view_model;
} PJToDoTypeServiceImpl;

PJToDoTypeServiceImpl *createPJToDoTypeService(PJToDoTypeServiceDelegate delegate);

ToDoTypeInsert *createToDoTypeInsert(const char *type_name);

ToDoTypeServiceViewModel *createToDoTypeServiceViewModel(void);

void freePJToDoTypeServiceImpl(PJToDoTypeServiceImpl *ptr);

void init_hello_piaojin(void);

void insertToDoType(ToDoTypeInsert toDoType);

#endif /* app_bindings_h */
