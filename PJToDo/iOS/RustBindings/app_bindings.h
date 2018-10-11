/* This Source Code Form is subject to the PJTodo
 * gitHub: https://github.com/piaojin/PJToDo
 * Rust on iOS.
 * Copyright © 2018年 piaojin. All rights reserved.
 */

#ifndef app_bindings_h
#define app_bindings_h

/* Generated with cbindgen:0.6.3 */

/* Generated with cbindgen */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file:
 *   1. Get the latest cbindgen using `cargo install --force cbindgen`
 *      a. Alternatively, you can clone `https://github.com/eqrion/cbindgen` and use a tagged release
 *   2. Run `rustup run nightly cbindgen path/to/your/crate/projrct --lockfile Cargo.lock --crate your/crate/name -o path/to/save/head_ffi_generated.h`
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

extern void test_pal_from_Swift(void);

void test_pal_from_rust(void);

#endif /* app_bindings_h */
