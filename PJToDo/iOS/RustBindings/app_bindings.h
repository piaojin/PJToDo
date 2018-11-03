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

typedef struct PJToDoTagServiceController PJToDoTagServiceController;

typedef struct PJToDoTypeServiceController PJToDoTypeServiceController;

typedef struct ToDoTag ToDoTag;

typedef struct ToDoTagInsert ToDoTagInsert;

typedef struct ToDoType ToDoType;

typedef struct ToDoTypeInsert ToDoTypeInsert;

typedef struct Vec_ToDoTag Vec_ToDoTag;

typedef struct Vec_ToDoType Vec_ToDoType;

typedef struct {
  void *user;
  void (*destroy)(void*);
  void (*insert_result)(void*, bool);
  void (*delete_result)(void*, bool);
  void (*update_result)(void*, bool);
  void (*find_byId_result)(void*, ToDoTag*, bool);
  void (*find_byName_result)(void*, ToDoTag*, bool);
  void (*fetch_data_result)(void*, bool);
} IPJToDoTagDelegate;

typedef struct {
  IPJToDoTagDelegate delegate;
  PJToDoTagServiceController *todo_tag_service_controller;
  ToDoTag *find_result_todo_tag;
  ToDoTagInsert *insert_todo_tag;
  Vec_ToDoTag *todo_tags;
} PJToDoTagController;

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
  ToDoTypeInsert *insert_todo_type;
  Vec_ToDoType *todo_types;
} PJToDoTypeController;

PJToDoTagController *createPJToDoTagController(IPJToDoTagDelegate delegate);

PJToDoTypeController *createPJToDoTypeController(IPJToDoTypeDelegate delegate);

ToDoTag *createToDoTag(const char *tag_name);

ToDoTagInsert *createToDoTagInsert(const char *tag_name);

ToDoType *createToDoType(const char *type_name);

ToDoTypeInsert *createToDoTypeInsert(const char *type_name);

void deleteToDoTag(PJToDoTagController *ptr, int32_t toDoTagId);

void deleteToDoType(PJToDoTypeController *ptr, int32_t toDoTypeId);

void fetchToDoTagData(PJToDoTagController *ptr);

void fetchToDoTypeData(PJToDoTypeController *ptr);

void findToDoTag(PJToDoTagController *ptr, int32_t toDoTagId);

void findToDoTagByName(PJToDoTagController *ptr, const char *tag_name);

void findToDoType(PJToDoTypeController *ptr, int32_t toDoTypeId);

void findToDoTypeByName(PJToDoTypeController *ptr, const char *type_name);

void free_rust_PJToDoTagController(PJToDoTagController *ptr);

void free_rust_PJToDoTypeController(PJToDoTypeController *ptr);

void free_rust_ToDoTag(ToDoTag *ptr);

void free_rust_ToDoTagInsert(ToDoTagInsert *ptr);

void free_rust_ToDoType(ToDoType *ptr);

void free_rust_ToDoTypeInsert(ToDoTypeInsert *ptr);

void free_rust_object(void *ptr);

int32_t getToDoTagCount(const PJToDoTagController *ptr);

int32_t getToDoTagId(ToDoTag *ptr);

const char *getToDoTagInsertName(const ToDoTagInsert *ptr);

const char *getToDoTagName(const ToDoTag *ptr);

int32_t getToDoTypeCount(const PJToDoTypeController *ptr);

int32_t getToDoTypeId(ToDoType *ptr);

const char *getToDoTypeInsertName(const ToDoTypeInsert *ptr);

const char *getToDoTypeName(const ToDoType *ptr);

extern const char *get_db_gzip_path(void);

extern const char *get_db_path(void);

extern const char *get_db_uncompresses_path(void);

void init_core_lib(void);

void init_database(void);

void init_hello_piaojin(void);

void init_tables(void);

void insertToDoTag(PJToDoTagController *ptr, ToDoTagInsert *toDoTag);

void insertToDoType(PJToDoTypeController *ptr, ToDoTypeInsert *toDoType);

void setToDoTagId(ToDoTag *ptr, int32_t type_id);

void setToDoTagInsertName(ToDoTagInsert *ptr, const char *tag_name);

void setToDoTagName(ToDoTag *ptr, const char *tag_name);

void setToDoTypeId(ToDoType *ptr, int32_t type_id);

void setToDoTypeInsertName(ToDoTypeInsert *ptr, const char *type_name);

void setToDoTypeName(ToDoType *ptr, const char *type_name);

extern void test_pal_from_Swift(void);

void test_pal_from_rust(void);

const ToDoTag *todoTagAtIndex(const PJToDoTagController *ptr, int32_t index);

const ToDoType *todoTypeAtIndex(const PJToDoTypeController *ptr, int32_t index);

void updateToDoTag(PJToDoTagController *ptr, const ToDoTag *toDoTag);

void updateToDoType(PJToDoTypeController *ptr, const ToDoType *toDoType);

#endif /* app_bindings_h */
