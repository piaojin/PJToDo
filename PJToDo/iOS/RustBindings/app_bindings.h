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

typedef struct Vec_ToDoType Vec_ToDoType;

typedef struct {
  void *user;
  void (*destroy)(void*);
  void (*insert_result)(void*, bool);
  void (*delete_result)(void*, bool);
  void (*update_result)(void*, bool);
  void (*find_byId_result)(void*, ToDoType*, bool);
  void (*find_byName_result)(void*, ToDoType*, bool);
  void (*fetch_data_result)(void*, bool);
} IPJToDoTypeDelegate;

typedef struct {
  IPJToDoTypeDelegate delegate;
  PJToDoTypeServiceController *todo_typ_service_controller;
  ToDoType *find_result_todo_type;
  Vec_ToDoType *todo_types;
} PJToDoTypeController;

PJToDoTypeController *createPJToDoTypeController(IPJToDoTypeDelegate delegate);

ToDoType *createToDoType(const char *type_name);

ToDoTypeInsert *createToDoTypeInsert(const char *type_name);

void deleteToDoType(PJToDoTypeController *ptr, int32_t toDoTypeId);

void fetchToDoTypeData(PJToDoTypeController *ptr);

void findToDoType(PJToDoTypeController *ptr, int32_t toDoTypeId);

void findToDoTypeByName(PJToDoTypeController *ptr, const char *type_name);

void free_rust_PJToDoTypeController(PJToDoTypeController *ptr);

void free_rust_ToDoType(ToDoType *ptr);

void free_rust_ToDoTypeInsert(ToDoTypeInsert *ptr);

void free_rust_object(void *ptr);

int32_t getToDoTypeCount(const PJToDoTypeController *ptr);

const char *getToDoTypeInsertTypeName(const ToDoTypeInsert *ptr);

int32_t getToDoTypeTypeId(ToDoType *ptr);

const char *getToDoTypeTypeName(const ToDoType *ptr);

extern const char *get_db_gzip_path(void);

extern const char *get_db_path(void);

extern const char *get_db_uncompresses_path(void);

void init_core_lib(void);

void init_database(void);

void init_hello_piaojin(void);

void init_tables(void);

void insertToDoType(PJToDoTypeController *ptr, const ToDoTypeInsert *toDoType);

void setToDoTypeInsertTypeName(ToDoTypeInsert *ptr, const char *type_name);

void setToDoTypeTypeId(ToDoType *ptr, int32_t type_id);

void setToDoTypeTypeName(ToDoType *ptr, const char *type_name);

extern void test_pal_from_Swift(void);

void test_pal_from_rust(void);

const ToDoType *todoTypeAtIndex(const PJToDoTypeController *ptr, int32_t index);

void updateToDoType(PJToDoTypeController *ptr, const ToDoType *toDoType);

#endif /* app_bindings_h */
