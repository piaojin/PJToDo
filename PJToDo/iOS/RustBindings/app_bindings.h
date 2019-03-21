/* This Source Code Form is subject to the PJTodo
 * gitHub: https://github.com/piaojin/PJToDo
 * Rust on iOS.
 * Copyright © 2018年 piaojin. All rights reserved.
 */

#ifndef app_bindings_h
#define app_bindings_h

/* Generated with cbindgen:0.8.2 */

/* Generated with cbindgen */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file:
 *   1. Get the latest cbindgen using `cargo install --force cbindgen`
 *      a. Alternatively, you can clone `https://github.com/eqrion/cbindgen` and use a tagged release
 *   2. Run `rustup run nightly cbindgen path/to/your/crate/projrct --lockfile Cargo.lock --crate your/crate/name -o path/to/save/head_ffi_generated.h`
 */ 
 
 

#include "struct_heads.h"

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct PJToDoSearchServiceController PJToDoSearchServiceController;

typedef struct PJToDoServiceController PJToDoServiceController;

typedef struct PJToDoSettingsServiceController PJToDoSettingsServiceController;

typedef struct PJToDoTagServiceController PJToDoTagServiceController;

typedef struct PJToDoTypeServiceController PJToDoTypeServiceController;

typedef struct ToDoInsert ToDoInsert;

typedef struct ToDoQuery ToDoQuery;

typedef struct ToDoSettings ToDoSettings;

typedef struct ToDoSettingsInsert ToDoSettingsInsert;

typedef struct ToDoTag ToDoTag;

typedef struct ToDoTagInsert ToDoTagInsert;

typedef struct ToDoType ToDoType;

typedef struct ToDoTypeInsert ToDoTypeInsert;

typedef struct Vec_String Vec_String;

typedef struct Vec_ToDoQuery Vec_ToDoQuery;

typedef struct Vec_ToDoSettings Vec_ToDoSettings;

typedef struct Vec_ToDoTag Vec_ToDoTag;

typedef struct Vec_ToDoType Vec_ToDoType;

typedef struct Vec_Vec_ToDoQuery Vec_Vec_ToDoQuery;

typedef struct {
  void *user;
  void (*destroy)(void *user);
  void (*request_result)(void *user, char *data, uint16_t statusCode, bool is_success);
} IPJToDoHttpRequestDelegate;

typedef struct {
  void *user;
  void (*destroy)(void *user);
  void (*insert_result)(void *user, bool isSuccess);
  void (*delete_result)(void *user, bool isSuccess);
  void (*update_result)(void *user, bool isSuccess);
  void (*find_byId_result)(void *user, ToDoQuery *toDo, bool isSuccess);
  void (*fetch_data_result)(void *user, bool isSuccess);
  void (*update_overdue_todos)(void *user, bool isSuccess);
} IPJToDoDelegate;

typedef struct {
  IPJToDoDelegate delegate;
  PJToDoServiceController *todo_service_controller;
  ToDoQuery *find_result_todo;
  ToDoInsert *insert_todo;
  Vec_Vec_ToDoQuery *todos;
  Vec_ToDoType *todo_types;
  Vec_ToDoTag *todo_tags;
  const Vec_String *sectionTitles;
} PJToDoController;

typedef struct {
  void *user;
  void (*destroy)(void *user);
  void (*find_byTitle_result)(void *user, ToDoQuery *toDo, bool isSuccess);
  void (*find_byLike_result)(void *user, bool isSuccess);
} IPJToDoSearchDelegate;

typedef struct {
  IPJToDoSearchDelegate delegate;
  PJToDoSearchServiceController *todo_search_service_controller;
  Vec_ToDoQuery *todos;
  Vec_ToDoType *todo_types;
  Vec_ToDoTag *todo_tags;
  ToDoQuery *find_result_todo;
} PJToDoSearchController;

typedef struct {
  void *user;
  void (*destroy)(void *user);
  void (*insert_result)(void *user, bool isSuccess);
  void (*delete_result)(void *user, bool isSuccess);
  void (*update_result)(void *user, bool isSuccess);
  void (*fetch_data_result)(void *user, bool isSuccess);
} IPJToDoSettingsDelegate;

typedef struct {
  IPJToDoSettingsDelegate delegate;
  PJToDoSettingsServiceController *todo_settings_service_controller;
  ToDoSettingsInsert *insert_todo_settings;
  Vec_ToDoSettings *todo_settings;
} PJToDoSettingsController;

typedef struct {
  void *user;
  void (*destroy)(void *user);
  void (*insert_result)(void *user, bool isSuccess);
  void (*delete_result)(void *user, bool isSuccess);
  void (*update_result)(void *user, bool isSuccess);
  void (*find_byId_result)(void *user, ToDoTag *toDoTag, bool isSuccess);
  void (*find_byName_result)(void *user, ToDoTag *toDoTag, bool isSuccess);
  void (*fetch_data_result)(void *user, bool isSuccess);
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
  void (*destroy)(void *user);
  void (*insert_result)(void *user, bool isSuccess);
  void (*delete_result)(void *user, bool isSuccess);
  void (*update_result)(void *user, bool isSuccess);
  void (*find_byId_result)(void *user, ToDoType *toDoType, bool isSuccess);
  void (*find_byName_result)(void *user, ToDoType *toDoType, bool isSuccess);
  void (*fetch_data_result)(void *user, bool isSuccess);
} IPJToDoTypeDelegate;

typedef struct {
  IPJToDoTypeDelegate delegate;
  PJToDoTypeServiceController *todo_typ_service_controller;
  ToDoType *find_result_todo_type;
  ToDoTypeInsert *insert_todo_type;
  Vec_ToDoType *todo_types;
} PJToDoTypeController;

extern const char *get_authorization_str(void);

extern const char *get_db_path(void);

void pj_authorizations(IPJToDoHttpRequestDelegate delegate, const char *authorization);

char *pj_convert_base64str_to_str(const char *ptr);

char *pj_convert_str_to_base64str(const char *ptr);

PJToDoController *pj_create_PJToDoController(IPJToDoDelegate delegate);

PJToDoSearchController *pj_create_PJToDoSearchController(IPJToDoSearchDelegate delegate);

PJToDoSettingsController *pj_create_PJToDoSettingsController(IPJToDoSettingsDelegate delegate);

PJToDoTagController *pj_create_PJToDoTagController(IPJToDoTagDelegate delegate);

PJToDoTypeController *pj_create_PJToDoTypeController(IPJToDoTypeDelegate delegate);

ToDoInsert *pj_create_ToDoInsert(void);

ToDoQuery *pj_create_ToDoQuery(void);

ToDoSettings *pj_create_ToDoSettings(const char *remind_email, int32_t remind_days);

ToDoSettingsInsert *pj_create_ToDoSettingsInsert(const char *remind_email, int32_t remind_days);

ToDoTag *pj_create_ToDoTag(const char *tag_name);

ToDoTagInsert *pj_create_ToDoTagInsert(const char *tag_name);

ToDoType *pj_create_ToDoType(const char *type_name);

ToDoTypeInsert *pj_create_ToDoTypeInsert(const char *type_name);

void pj_create_folder(const char *folder_path);

void pj_create_repos(IPJToDoHttpRequestDelegate delegate);

void pj_create_repos_file(IPJToDoHttpRequestDelegate delegate, const char *request_url, const char *path, const char *message, const char *content, const char *sha);

void pj_delete_repos(IPJToDoHttpRequestDelegate delegate, const char *repos_url);

void pj_delete_repos_file(IPJToDoHttpRequestDelegate delegate, const char *request_url, const char *path, const char *message, const char *content, const char *sha);

void pj_delete_todo(PJToDoController *ptr, int32_t section, int32_t index, int32_t toDoId);

void pj_delete_todo_by_id(PJToDoController *ptr, int32_t toDoId);

void pj_delete_todo_settings(PJToDoSettingsController *ptr, int32_t toDoSettingsId);

void pj_delete_todo_tag(PJToDoTagController *ptr, int32_t toDoTagId);

void pj_delete_todo_type(PJToDoTypeController *ptr, int32_t toDoTypeId);

void pj_download_file(IPJToDoHttpRequestDelegate delegate, const char *request_url, const char *save_path);

void pj_fetch_todo_data(PJToDoController *ptr);

void pj_fetch_todo_settings_data(PJToDoSettingsController *ptr);

void pj_fetch_todo_tag_data(PJToDoTagController *ptr);

void pj_fetch_todo_type_data(PJToDoTypeController *ptr);

void pj_find_todo(PJToDoController *ptr, int32_t toDoId);

void pj_find_todo_by_title(PJToDoSearchController *ptr, const char *title);

void pj_find_todo_like_title(PJToDoSearchController *ptr, const char *title);

void pj_find_todo_tag(PJToDoTagController *ptr, int32_t toDoTagId);

void pj_find_todo_tag_by_name(PJToDoTagController *ptr, const char *tag_name);

void pj_find_todo_type(PJToDoTypeController *ptr, int32_t toDoTypeId);

void pj_find_todo_type_by_name(PJToDoTypeController *ptr, const char *type_name);

void pj_free_rust_PJToDoController(PJToDoController *ptr);

void pj_free_rust_PJToDoSearchController(PJToDoSearchController *ptr);

void pj_free_rust_PJToDoSettingsController(PJToDoSettingsController *ptr);

void pj_free_rust_PJToDoTagController(PJToDoTagController *ptr);

void pj_free_rust_PJToDoTypeController(PJToDoTypeController *ptr);

void pj_free_rust_object(void *ptr);

void pj_free_rust_string(char *ptr);

void pj_get_repos(IPJToDoHttpRequestDelegate delegate, const char *repos_url);

void pj_get_repos_file(IPJToDoHttpRequestDelegate delegate, const char *request_url);

const ToDoTag *pj_get_search_todo_tag_with_id(const PJToDoSearchController *ptr, int32_t tag_id);

const ToDoType *pj_get_search_todo_type_with_id(const PJToDoSearchController *ptr, int32_t type_id);

int32_t pj_get_todo_counts_at_section(const PJToDoController *ptr, int32_t section);

char *pj_get_todo_insert_content(const ToDoInsert *ptr);

char *pj_get_todo_insert_create_time(const ToDoInsert *ptr);

char *pj_get_todo_insert_duetime(const ToDoInsert *ptr);

char *pj_get_todo_insert_remind_time(const ToDoInsert *ptr);

int32_t pj_get_todo_insert_state(ToDoInsert *ptr);

char *pj_get_todo_insert_title(const ToDoInsert *ptr);

int32_t pj_get_todo_insert_todo_priority(ToDoInsert *ptr);

int32_t pj_get_todo_insert_todo_tag_id(ToDoInsert *ptr);

int32_t pj_get_todo_insert_todo_type_id(ToDoInsert *ptr);

char *pj_get_todo_insert_update_time(const ToDoInsert *ptr);

int32_t pj_get_todo_number_of_sections(const PJToDoController *ptr);

char *pj_get_todo_query_content(const ToDoQuery *ptr);

char *pj_get_todo_query_create_time(const ToDoQuery *ptr);

char *pj_get_todo_query_duetime(const ToDoQuery *ptr);

int32_t pj_get_todo_query_id(ToDoQuery *ptr);

char *pj_get_todo_query_remind_time(const ToDoQuery *ptr);

int32_t pj_get_todo_query_state(ToDoQuery *ptr);

char *pj_get_todo_query_title(const ToDoQuery *ptr);

int32_t pj_get_todo_query_todo_priority(ToDoQuery *ptr);

int32_t pj_get_todo_query_todo_tag_id(ToDoQuery *ptr);

int32_t pj_get_todo_query_todo_type_id(ToDoQuery *ptr);

char *pj_get_todo_query_update_time(const ToDoQuery *ptr);

int32_t pj_get_todo_settings_count(const PJToDoSettingsController *ptr);

int32_t pj_get_todo_settings_id(ToDoSettings *ptr);

int32_t pj_get_todo_settings_insert_remind_days(ToDoSettingsInsert *ptr);

char *pj_get_todo_settings_insert_remind_email(const ToDoSettingsInsert *ptr);

int32_t pj_get_todo_settings_remind_days(ToDoSettings *ptr);

char *pj_get_todo_settings_remind_email(const ToDoSettings *ptr);

int32_t pj_get_todo_tag_count(const PJToDoTagController *ptr);

int32_t pj_get_todo_tag_id(ToDoTag *ptr);

char *pj_get_todo_tag_insert_name(const ToDoTagInsert *ptr);

char *pj_get_todo_tag_name(const ToDoTag *ptr);

int32_t pj_get_todo_type_count(const PJToDoTypeController *ptr);

int32_t pj_get_todo_type_id(ToDoType *ptr);

char *pj_get_todo_type_insert_name(const ToDoTypeInsert *ptr);

char *pj_get_todo_type_name(const ToDoType *ptr);

void pj_init_corelib(void);

void pj_init_data_base(void);

void pj_init_hello_piaojin(void);

void pj_init_tables(void);

void pj_insert_todo(PJToDoController *ptr, ToDoInsert *toDo);

void pj_insert_todo_settings(PJToDoSettingsController *ptr, ToDoSettingsInsert *toDoSettings);

void pj_insert_todo_tag(PJToDoTagController *ptr, ToDoTagInsert *toDoTag);

void pj_insert_todo_type(PJToDoTypeController *ptr, ToDoTypeInsert *toDoType);

void pj_login(IPJToDoHttpRequestDelegate delegate, const char *name, const char *password);

void pj_logout(void);

void pj_remove_file(const char *file_path);

void pj_remove_folder(const char *folder_path, bool all);

void pj_request_user_info(IPJToDoHttpRequestDelegate delegate);

const ToDoQuery *pj_search_todo_result_at_index(const PJToDoSearchController *ptr, int32_t index);

int32_t pj_search_todo_result_count(const PJToDoSearchController *ptr);

void pj_set_todo_insert_content(ToDoInsert *ptr, const char *content);

void pj_set_todo_insert_create_time(ToDoInsert *ptr, const char *create_time);

void pj_set_todo_insert_duetime(ToDoInsert *ptr, const char *due_time);

void pj_set_todo_insert_remind_time(ToDoInsert *ptr, const char *remind_time);

void pj_set_todo_insert_state(ToDoInsert *ptr, int32_t state);

void pj_set_todo_insert_title(ToDoInsert *ptr, const char *title);

void pj_set_todo_insert_todo_priority(ToDoInsert *ptr, int32_t priority);

void pj_set_todo_insert_todo_tag_id(ToDoInsert *ptr, int32_t to_do_tag_id);

void pj_set_todo_insert_todo_type_id(ToDoInsert *ptr, int32_t to_do_type_id);

void pj_set_todo_insert_update_time(ToDoInsert *ptr, const char *update_time);

void pj_set_todo_query_content(ToDoQuery *ptr, const char *content);

void pj_set_todo_query_create_time(ToDoQuery *ptr, const char *create_time);

void pj_set_todo_query_duetime(ToDoQuery *ptr, const char *due_time);

void pj_set_todo_query_id(ToDoQuery *ptr, int32_t _id);

void pj_set_todo_query_remind_time(ToDoQuery *ptr, const char *remind_time);

void pj_set_todo_query_state(ToDoQuery *ptr, int32_t state);

void pj_set_todo_query_title(ToDoQuery *ptr, const char *title);

void pj_set_todo_query_todo_priority(ToDoQuery *ptr, int32_t priority);

void pj_set_todo_query_todo_tag_id(ToDoQuery *ptr, int32_t to_do_tag_id);

void pj_set_todo_query_todo_type_id(ToDoQuery *ptr, int32_t to_do_type_id);

void pj_set_todo_query_update_time(ToDoQuery *ptr, const char *update_time);

void pj_set_todo_settings_id(ToDoSettings *ptr, int32_t settins_id);

void pj_set_todo_settings_insert_remind_days(ToDoSettingsInsert *ptr, int32_t remind_days);

void pj_set_todo_settings_insert_remind_email(ToDoSettingsInsert *ptr, const char *remind_email);

void pj_set_todo_settings_remind_days(ToDoSettings *ptr, int32_t remind_days);

void pj_set_todo_settings_remind_email(ToDoSettings *ptr, const char *remind_email);

void pj_set_todo_tag_id(ToDoTag *ptr, int32_t type_id);

void pj_set_todo_tag_insert_name(ToDoTagInsert *ptr, const char *tag_name);

void pj_set_todo_tag_name(ToDoTag *ptr, const char *tag_name);

void pj_set_todo_type_id(ToDoType *ptr, int32_t type_id);

void pj_set_todo_type_insert_name(ToDoTypeInsert *ptr, const char *type_name);

void pj_set_todo_type_name(ToDoType *ptr, const char *type_name);

void pj_test_pal_from_rust(void);

const ToDoQuery *pj_todo_at_section(const PJToDoController *ptr, int32_t section, int32_t index);

const ToDoSettings *pj_todo_settings_at_index(const PJToDoSettingsController *ptr, int32_t index);

const ToDoTag *pj_todo_tag_at_index(const PJToDoTagController *ptr, int32_t index);

const ToDoTag *pj_todo_tag_with_id(const PJToDoController *ptr, int32_t tag_id);

char *pj_todo_title_at_section(const PJToDoController *ptr, int32_t section);

const ToDoType *pj_todo_type_at_index(const PJToDoTypeController *ptr, int32_t index);

const ToDoType *pj_todo_type_with_id(const PJToDoController *ptr, int32_t type_id);

void pj_update_db_connection(void);

void pj_update_overdue_todos(const PJToDoController *ptr);

void pj_update_repos_file(IPJToDoHttpRequestDelegate delegate, const char *request_url, const char *path, const char *message, const char *content, const char *sha);

void pj_update_todo(PJToDoController *ptr, const ToDoQuery *toDo);

void pj_update_todo_settings(PJToDoSettingsController *ptr, const ToDoSettings *toDoSettings);

void pj_update_todo_tag(PJToDoTagController *ptr, const ToDoTag *toDoTag);

void pj_update_todo_type(PJToDoTypeController *ptr, const ToDoType *toDoType);

extern void test_pal_from_Swift(void);

#endif /* app_bindings_h */
