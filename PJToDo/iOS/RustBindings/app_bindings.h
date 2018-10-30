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

typedef struct PJToDoTypeServiceController PJToDoTypeServiceController;

typedef struct ToDoType ToDoType;

typedef struct ToDoTypeInsert ToDoTypeInsert;

typedef struct {
  void *user;
  void (*destroy)(void*);
  void (*insert_result)(void*, bool);
  void (*delete_result)(void*, bool);
} IPJToDoTypeDelegate;

typedef struct {
  IPJToDoTypeDelegate delegate;
  PJToDoTypeServiceController *todo_typ_service_controller;
  ToDoTypeInsert *todo_type_insert;
} PJToDoTypeController;

typedef struct {
  void (*insert_to_do_type)(ToDoTypeInsert);
} PJToDoTypeViewModel;

PJToDoTypeController *createPJToDoTypeController(IPJToDoTypeDelegate delegate);

PJToDoTypeViewModel *createPJToDoTypeViewModel(void);

ToDoType *createToDoType(const char *type_name);

ToDoTypeInsert *createToDoTypeInsert(const char *type_name);

void deleteToDoType(PJToDoTypeController *ptr, int32_t toDoTypeId);

void free_rust_PJToDoTypeController(PJToDoTypeController *ptr);

void free_rust_object(void *ptr);

const ToDoTypeInsert *getToDoType(const PJToDoTypeController *ptr);

extern const char *get_db_gzip_path(void);

extern const char *get_db_path(void);

extern const char *get_db_uncompresses_path(void);

void init_core_lib(void);

void init_database(void);

void init_hello_piaojin(void);

void init_tables(void);

void insertToDoType(PJToDoTypeController *ptr, const ToDoTypeInsert *toDoType);

void insertToDoType2(ToDoTypeInsert toDoType);

void setToDoTypeTypeName(ToDoType *ptr, const char *type_name);

extern void test_pal_from_Swift(void);

void test_pal_from_rust(void);

#endif /* app_bindings_h */
