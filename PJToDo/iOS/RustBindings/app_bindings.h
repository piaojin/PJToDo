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

typedef struct PJToDoServiceController PJToDoServiceController;

typedef struct PJToDoSettingsServiceController PJToDoSettingsServiceController;

typedef struct PJToDoTagServiceController PJToDoTagServiceController;

typedef struct PJToDoTypeServiceController PJToDoTypeServiceController;

typedef struct ToDo ToDo;

typedef struct ToDoInsert ToDoInsert;

typedef struct ToDoQuery ToDoQuery;

typedef struct ToDoSettings ToDoSettings;

typedef struct ToDoSettingsInsert ToDoSettingsInsert;

typedef struct ToDoTag ToDoTag;

typedef struct ToDoTagInsert ToDoTagInsert;

typedef struct ToDoType ToDoType;

typedef struct ToDoTypeInsert ToDoTypeInsert;

typedef struct Vec_ToDoQuery Vec_ToDoQuery;

typedef struct Vec_ToDoSettings Vec_ToDoSettings;

typedef struct Vec_ToDoTag Vec_ToDoTag;

typedef struct Vec_ToDoType Vec_ToDoType;

typedef struct Vec_Vec_ToDoQuery Vec_Vec_ToDoQuery;

typedef struct {
  void *user;
  void (*destroy)(void*);
  void (*insert_result)(void*, bool);
  void (*delete_result)(void*, bool);
  void (*update_result)(void*, bool);
  void (*find_byId_result)(void*, ToDoQuery*, bool);
  void (*find_byTitle_result)(void*, ToDoQuery*, bool);
  void (*fetch_data_result)(void*, bool);
  void (*find_byLike_result)(void*, bool);
  void (*todo_date_future_day_more_than_result)(void*, bool);
  void (*fetch_todos_order_by_state_result)(void*, bool);
} IPJToDoDelegate;

typedef struct {
  IPJToDoDelegate delegate;
  PJToDoServiceController *todo_service_controller;
  ToDoQuery *find_result_todo;
  ToDoInsert *insert_todo;
  Vec_Vec_ToDoQuery *todos;
  Vec_ToDoType *todo_types;
  Vec_ToDoTag *todo_tags;
  Vec_ToDoQuery *like_title_result_todos;
} PJToDoController;

typedef struct {
  void *user;
  void (*destroy)(void*);
  void (*insert_result)(void*, bool);
  void (*delete_result)(void*, bool);
  void (*update_result)(void*, bool);
  void (*fetch_data_result)(void*, bool);
} IPJToDoSettingsDelegate;

typedef struct {
  IPJToDoSettingsDelegate delegate;
  PJToDoSettingsServiceController *todo_settings_service_controller;
  ToDoSettingsInsert *insert_todo_settings;
  Vec_ToDoSettings *todo_settings;
} PJToDoSettingsController;

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

PJToDoController *createPJToDoController(IPJToDoDelegate delegate);

PJToDoSettingsController *createPJToDoSettingsController(IPJToDoSettingsDelegate delegate);

PJToDoTagController *createPJToDoTagController(IPJToDoTagDelegate delegate);

PJToDoTypeController *createPJToDoTypeController(IPJToDoTypeDelegate delegate);

ToDo *createToDo(void);

ToDoInsert *createToDoInsert(void);

ToDoQuery *createToDoQuery(void);

ToDoSettings *createToDoSettings(const char *remind_email, int32_t remind_days);

ToDoSettingsInsert *createToDoSettingsInsert(const char *remind_email, int32_t remind_days);

ToDoTag *createToDoTag(const char *tag_name);

ToDoTagInsert *createToDoTagInsert(const char *tag_name);

ToDoType *createToDoType(const char *type_name);

ToDoTypeInsert *createToDoTypeInsert(const char *type_name);

void deleteToDo(PJToDoController *ptr, int32_t toDoId);

void deleteToDoSettings(PJToDoSettingsController *ptr, int32_t toDoSettingsId);

void deleteToDoTag(PJToDoTagController *ptr, int32_t toDoTagId);

void deleteToDoType(PJToDoTypeController *ptr, int32_t toDoTypeId);

void fetchToDoData(PJToDoController *ptr);

void fetchToDoSettingsData(PJToDoSettingsController *ptr);

void fetchToDoTagData(PJToDoTagController *ptr);

void fetchToDoTypeData(PJToDoTypeController *ptr);

void findToDo(PJToDoController *ptr, int32_t toDoId);

void findToDoByTitle(PJToDoController *ptr, const char *title);

void findToDoLikeTitle(PJToDoController *ptr, const char *title);

void findToDoTag(PJToDoTagController *ptr, int32_t toDoTagId);

void findToDoTagByName(PJToDoTagController *ptr, const char *tag_name);

void findToDoType(PJToDoTypeController *ptr, int32_t toDoTypeId);

void findToDoTypeByName(PJToDoTypeController *ptr, const char *type_name);

void free_rust_PJToDoController(PJToDoController *ptr);

void free_rust_PJToDoSettingsController(PJToDoSettingsController *ptr);

void free_rust_PJToDoTagController(PJToDoTagController *ptr);

void free_rust_PJToDoTypeController(PJToDoTypeController *ptr);

void free_rust_object(void *ptr);

int32_t getToDoCountOfSections(const PJToDoController *ptr);

int32_t getToDoCountsAtSection(const PJToDoController *ptr, int32_t section);

const char *getToDoInsertContent(const ToDoInsert *ptr);

const char *getToDoInsertCreateTime(const ToDoInsert *ptr);

const char *getToDoInsertDueTime(const ToDoInsert *ptr);

const char *getToDoInsertRemindTime(const ToDoInsert *ptr);

int32_t getToDoInsertState(ToDoInsert *ptr);

const char *getToDoInsertTitle(const ToDoInsert *ptr);

const char *getToDoInsertUpdateTime(const ToDoInsert *ptr);

int32_t getToDoInsert_ToDoTagId(ToDoInsert *ptr);

int32_t getToDoInsert_ToDoTypeId(ToDoInsert *ptr);

const char *getToDoQueryContent(const ToDoQuery *ptr);

const char *getToDoQueryCreateTime(const ToDoQuery *ptr);

const char *getToDoQueryDueTime(const ToDoQuery *ptr);

int32_t getToDoQueryId(ToDoQuery *ptr);

const char *getToDoQueryRemindTime(const ToDoQuery *ptr);

int32_t getToDoQueryState(ToDoQuery *ptr);

const char *getToDoQueryTitle(const ToDoQuery *ptr);

const char *getToDoQueryUpdateTime(const ToDoQuery *ptr);

int32_t getToDoQuery_ToDoTagId(ToDoQuery *ptr);

int32_t getToDoQuery_ToDoTypeId(ToDoQuery *ptr);

int32_t getToDoSettingsCount(const PJToDoSettingsController *ptr);

int32_t getToDoSettingsId(ToDoSettings *ptr);

int32_t getToDoSettingsInsertRemindDays(ToDoSettingsInsert *ptr);

const char *getToDoSettingsInsertRemindEmail(const ToDoSettingsInsert *ptr);

int32_t getToDoSettingsRemindDays(ToDoSettings *ptr);

const char *getToDoSettingsRemindEmail(const ToDoSettings *ptr);

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

void insertToDo(PJToDoController *ptr, ToDoInsert *toDo);

void insertToDoSettings(PJToDoSettingsController *ptr, ToDoSettingsInsert *toDoSettings);

void insertToDoTag(PJToDoTagController *ptr, ToDoTagInsert *toDoTag);

void insertToDoType(PJToDoTypeController *ptr, ToDoTypeInsert *toDoType);

void setToDoInsertContent(ToDoInsert *ptr, const char *content);

void setToDoInsertCreateTime(ToDoInsert *ptr, const char *create_time);

void setToDoInsertDueTime(ToDoInsert *ptr, const char *due_time);

void setToDoInsertRemindTime(ToDoInsert *ptr, const char *remind_time);

void setToDoInsertState(ToDoInsert *ptr, int32_t state);

void setToDoInsertTitle(ToDoInsert *ptr, const char *title);

void setToDoInsertUpdateTime(ToDoInsert *ptr, const char *update_time);

void setToDoInsert_ToDoTagId(ToDoInsert *ptr, int32_t to_do_tag_id);

void setToDoInsert_ToDoTypeId(ToDoInsert *ptr, int32_t to_do_type_id);

void setToDoQueryContent(ToDoQuery *ptr, const char *content);

void setToDoQueryCreateTime(ToDoQuery *ptr, const char *create_time);

void setToDoQueryDueTime(ToDoQuery *ptr, const char *due_time);

void setToDoQueryId(ToDoQuery *ptr, int32_t _id);

void setToDoQueryRemindTime(ToDoQuery *ptr, const char *remind_time);

void setToDoQueryState(ToDoQuery *ptr, int32_t state);

void setToDoQueryTitle(ToDoQuery *ptr, const char *title);

void setToDoQueryUpdateTime(ToDoQuery *ptr, const char *update_time);

void setToDoQuery_ToDoTagId(ToDoQuery *ptr, int32_t to_do_tag_id);

void setToDoQuery_ToDoTypeId(ToDoQuery *ptr, int32_t to_do_type_id);

void setToDoSettingsId(ToDoSettings *ptr, int32_t settins_id);

void setToDoSettingsInsertRemindDays(ToDoSettingsInsert *ptr, int32_t remind_days);

void setToDoSettingsInsertRemindEmail(ToDoSettingsInsert *ptr, const char *remind_email);

void setToDoSettingsRemindDays(ToDoSettings *ptr, int32_t remind_days);

void setToDoSettingsRemindEmail(ToDoSettings *ptr, const char *remind_email);

void setToDoTagId(ToDoTag *ptr, int32_t type_id);

void setToDoTagInsertName(ToDoTagInsert *ptr, const char *tag_name);

void setToDoTagName(ToDoTag *ptr, const char *tag_name);

void setToDoTypeId(ToDoType *ptr, int32_t type_id);

void setToDoTypeInsertName(ToDoTypeInsert *ptr, const char *type_name);

void setToDoTypeName(ToDoType *ptr, const char *type_name);

extern void test_pal_from_Swift(void);

void test_pal_from_rust(void);

const ToDoTag *toDoTagWithId(const PJToDoController *ptr, int32_t tag_id);

const ToDoType *toDoTypeWithId(const PJToDoController *ptr, int32_t type_id);

const ToDoQuery *todoAtSection(const PJToDoController *ptr, int32_t section, int32_t index);

const ToDoSettings *todoSettingsAtIndex(const PJToDoSettingsController *ptr, int32_t index);

const ToDoTag *todoTagAtIndex(const PJToDoTagController *ptr, int32_t index);

const ToDoType *todoTypeAtIndex(const PJToDoTypeController *ptr, int32_t index);

void updateToDo(PJToDoController *ptr, const ToDoQuery *toDo);

void updateToDoSettings(PJToDoSettingsController *ptr, const ToDoSettings *toDoSettings);

void updateToDoTag(PJToDoTagController *ptr, const ToDoTag *toDoTag);

void updateToDoType(PJToDoTypeController *ptr, const ToDoType *toDoType);

#endif /* app_bindings_h */
