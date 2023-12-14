/* This Source Code Form is subject to the PJTodo
 * gitHub: https://github.com/piaojin/PJToDo
 * Rust on iOS.
 * Copyright © 2018年 piaojin. All rights reserved.
 */

#ifndef app_bindings_h
#define app_bindings_h

/* Generated with cbindgen:0.26.0 */

/* Generated with cbindgen */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file:
 *   1. Get the latest cbindgen using `cargo install --force cbindgen`
 *      a. Alternatively, you can clone `https://github.com/eqrion/cbindgen` and use a tagged release
 *   2. Run `rustup run nightly cbindgen path/to/your/crate/projrct --lockfile Cargo.lock --crate your/crate/name -o path/to/save/head_ffi_generated.h`
 */ 
 
 

#import "struct_heads.h"

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

typedef struct IPJToDoHttpRequestDelegate {
  void *user;
  void (*destroy)(void *user);
  void (*request_result)(void *user, char *data, uint16_t statusCode, bool is_success);
} IPJToDoHttpRequestDelegate;

typedef struct IPJToDoDelegate {
  void *user;
  void (*destroy)(void *user);
  void (*insert_result)(void *user, bool isSuccess);
  void (*delete_result)(void *user, bool isSuccess);
  void (*update_result)(void *user, bool isSuccess);
  void (*find_byId_result)(void *user, struct ToDoQuery *toDo, bool isSuccess);
  void (*fetch_data_result)(void *user, bool isSuccess);
  void (*update_overdue_todos)(void *user, bool isSuccess);
} IPJToDoDelegate;

typedef struct PJToDoController {
  struct IPJToDoDelegate delegate;
  struct PJToDoServiceController *todo_service_controller;
  struct ToDoQuery *find_result_todo;
  struct ToDoInsert *insert_todo;
  struct Vec_Vec_ToDoQuery *todos;
  struct Vec_ToDoType *todo_types;
  struct Vec_ToDoTag *todo_tags;
  const struct Vec_String *sectionTitles;
} PJToDoController;

typedef struct IPJToDoSearchDelegate {
  void *user;
  void (*destroy)(void *user);
  void (*find_byTitle_result)(void *user, struct ToDoQuery *toDo, bool isSuccess);
  void (*find_byLike_result)(void *user, bool isSuccess);
} IPJToDoSearchDelegate;

typedef struct PJToDoSearchController {
  struct IPJToDoSearchDelegate delegate;
  struct PJToDoSearchServiceController *todo_search_service_controller;
  struct Vec_ToDoQuery *todos;
  struct Vec_ToDoType *todo_types;
  struct Vec_ToDoTag *todo_tags;
  struct ToDoQuery *find_result_todo;
} PJToDoSearchController;

typedef struct IPJToDoSettingsDelegate {
  void *user;
  void (*destroy)(void *user);
  void (*insert_result)(void *user, bool isSuccess);
  void (*delete_result)(void *user, bool isSuccess);
  void (*update_result)(void *user, bool isSuccess);
  void (*fetch_data_result)(void *user, bool isSuccess);
} IPJToDoSettingsDelegate;

typedef struct PJToDoSettingsController {
  struct IPJToDoSettingsDelegate delegate;
  struct PJToDoSettingsServiceController *todo_settings_service_controller;
  struct ToDoSettingsInsert *insert_todo_settings;
  struct Vec_ToDoSettings *todo_settings;
} PJToDoSettingsController;

typedef struct IPJToDoTagDelegate {
  void *user;
  void (*destroy)(void *user);
  void (*insert_result)(void *user, bool isSuccess);
  void (*delete_result)(void *user, bool isSuccess);
  void (*update_result)(void *user, bool isSuccess);
  void (*find_byId_result)(void *user, struct ToDoTag *toDoTag, bool isSuccess);
  void (*find_byName_result)(void *user, struct ToDoTag *toDoTag, bool isSuccess);
  void (*fetch_data_result)(void *user, bool isSuccess);
} IPJToDoTagDelegate;

typedef struct PJToDoTagController {
  struct IPJToDoTagDelegate delegate;
  struct PJToDoTagServiceController *todo_tag_service_controller;
  struct ToDoTag *find_result_todo_tag;
  struct ToDoTagInsert *insert_todo_tag;
  struct Vec_ToDoTag *todo_tags;
} PJToDoTagController;

typedef struct IPJToDoTypeDelegate {
  void *user;
  void (*destroy)(void *user);
  void (*insert_result)(void *user, bool isSuccess);
  void (*delete_result)(void *user, bool isSuccess);
  void (*update_result)(void *user, bool isSuccess);
  void (*find_byId_result)(void *user, struct ToDoType *toDoType, bool isSuccess);
  void (*find_byName_result)(void *user, struct ToDoType *toDoType, bool isSuccess);
  void (*fetch_data_result)(void *user, bool isSuccess);
} IPJToDoTypeDelegate;

typedef struct PJToDoTypeController {
  struct IPJToDoTypeDelegate delegate;
  struct PJToDoTypeServiceController *todo_typ_service_controller;
  struct ToDoType *find_result_todo_type;
  struct ToDoTypeInsert *insert_todo_type;
  struct Vec_ToDoType *todo_types;
} PJToDoTypeController;

struct ToDoTypeInsert *pj_create_ToDoTypeInsert(const char *type_name);

struct ToDoType *pj_create_ToDoType(const char *type_name);

void pj_set_todo_type_name(struct ToDoType *ptr, const char *type_name);

char *pj_get_todo_type_name(const struct ToDoType *ptr);

void pj_set_todo_type_id(struct ToDoType *ptr, int32_t type_id);

int32_t pj_get_todo_type_id(struct ToDoType *ptr);

void pj_set_todo_type_insert_name(struct ToDoTypeInsert *ptr, const char *type_name);

char *pj_get_todo_type_insert_name(const struct ToDoTypeInsert *ptr);

struct ToDoTagInsert *pj_create_ToDoTagInsert(const char *tag_name);

struct ToDoTag *pj_create_ToDoTag(const char *tag_name);

void pj_set_todo_tag_name(struct ToDoTag *ptr, const char *tag_name);

char *pj_get_todo_tag_name(const struct ToDoTag *ptr);

void pj_set_todo_tag_id(struct ToDoTag *ptr, int32_t type_id);

int32_t pj_get_todo_tag_id(struct ToDoTag *ptr);

void pj_set_todo_tag_insert_name(struct ToDoTagInsert *ptr, const char *tag_name);

char *pj_get_todo_tag_insert_name(const struct ToDoTagInsert *ptr);

struct ToDoInsert *pj_create_ToDoInsert(void);

struct ToDoQuery *pj_create_ToDoQuery(void);

void pj_set_todo_insert_title(struct ToDoInsert *ptr, const char *title);

char *pj_get_todo_insert_title(const struct ToDoInsert *ptr);

void pj_set_todo_insert_content(struct ToDoInsert *ptr, const char *content);

char *pj_get_todo_insert_content(const struct ToDoInsert *ptr);

void pj_set_todo_insert_duetime(struct ToDoInsert *ptr, const char *due_time);

char *pj_get_todo_insert_duetime(const struct ToDoInsert *ptr);

void pj_set_todo_insert_remind_time(struct ToDoInsert *ptr, const char *remind_time);

char *pj_get_todo_insert_remind_time(const struct ToDoInsert *ptr);

void pj_set_todo_insert_create_time(struct ToDoInsert *ptr, const char *create_time);

char *pj_get_todo_insert_create_time(const struct ToDoInsert *ptr);

void pj_set_todo_insert_update_time(struct ToDoInsert *ptr, const char *update_time);

char *pj_get_todo_insert_update_time(const struct ToDoInsert *ptr);

void pj_set_todo_insert_todo_type_id(struct ToDoInsert *ptr, int32_t to_do_type_id);

int32_t pj_get_todo_insert_todo_type_id(struct ToDoInsert *ptr);

void pj_set_todo_insert_todo_tag_id(struct ToDoInsert *ptr, int32_t to_do_tag_id);

int32_t pj_get_todo_insert_todo_tag_id(struct ToDoInsert *ptr);

int32_t pj_get_todo_insert_todo_priority(struct ToDoInsert *ptr);

void pj_set_todo_insert_todo_priority(struct ToDoInsert *ptr, int32_t priority);

void pj_set_todo_insert_state(struct ToDoInsert *ptr, int32_t state);

int32_t pj_get_todo_insert_state(struct ToDoInsert *ptr);

void pj_set_todo_query_title(struct ToDoQuery *ptr, const char *title);

char *pj_get_todo_query_title(const struct ToDoQuery *ptr);

void pj_set_todo_query_id(struct ToDoQuery *ptr, int32_t _id);

int32_t pj_get_todo_query_id(struct ToDoQuery *ptr);

void pj_set_todo_query_content(struct ToDoQuery *ptr, const char *content);

char *pj_get_todo_query_content(const struct ToDoQuery *ptr);

void pj_set_todo_query_duetime(struct ToDoQuery *ptr, const char *due_time);

char *pj_get_todo_query_duetime(const struct ToDoQuery *ptr);

void pj_set_todo_query_remind_time(struct ToDoQuery *ptr, const char *remind_time);

char *pj_get_todo_query_remind_time(const struct ToDoQuery *ptr);

void pj_set_todo_query_create_time(struct ToDoQuery *ptr, const char *create_time);

char *pj_get_todo_query_create_time(const struct ToDoQuery *ptr);

void pj_set_todo_query_update_time(struct ToDoQuery *ptr, const char *update_time);

char *pj_get_todo_query_update_time(const struct ToDoQuery *ptr);

void pj_set_todo_query_todo_type_id(struct ToDoQuery *ptr, int32_t to_do_type_id);

int32_t pj_get_todo_query_todo_type_id(struct ToDoQuery *ptr);

void pj_set_todo_query_todo_tag_id(struct ToDoQuery *ptr, int32_t to_do_tag_id);

int32_t pj_get_todo_query_todo_priority(struct ToDoQuery *ptr);

void pj_set_todo_query_todo_priority(struct ToDoQuery *ptr, int32_t priority);

int32_t pj_get_todo_query_todo_tag_id(struct ToDoQuery *ptr);

void pj_set_todo_query_state(struct ToDoQuery *ptr, int32_t state);

int32_t pj_get_todo_query_state(struct ToDoQuery *ptr);

void pj_free_rust_object(void *ptr);

void pj_free_rust_string(char *ptr);

char *pj_convert_str_to_base64str(const char *ptr);

char *pj_convert_base64str_to_str(const char *ptr);

void pj_create_folder(const char *folder_path);

void pj_remove_folder(const char *folder_path, bool all);

void pj_remove_file(const char *file_path);

struct ToDoSettingsInsert *pj_create_ToDoSettingsInsert(const char *remind_email, int32_t remind_days);

struct ToDoSettings *pj_create_ToDoSettings(const char *remind_email, int32_t remind_days);

void pj_set_todo_settings_remind_email(struct ToDoSettings *ptr, const char *remind_email);

char *pj_get_todo_settings_remind_email(const struct ToDoSettings *ptr);

void pj_set_todo_settings_id(struct ToDoSettings *ptr, int32_t settins_id);

int32_t pj_get_todo_settings_id(struct ToDoSettings *ptr);

void pj_set_todo_settings_remind_days(struct ToDoSettings *ptr, int32_t remind_days);

int32_t pj_get_todo_settings_remind_days(struct ToDoSettings *ptr);

void pj_set_todo_settings_insert_remind_email(struct ToDoSettingsInsert *ptr, const char *remind_email);

char *pj_get_todo_settings_insert_remind_email(const struct ToDoSettingsInsert *ptr);

void pj_set_todo_settings_insert_remind_days(struct ToDoSettingsInsert *ptr, int32_t remind_days);

int32_t pj_get_todo_settings_insert_remind_days(struct ToDoSettingsInsert *ptr);

void pj_login(struct IPJToDoHttpRequestDelegate delegate, const char *name, const char *password);

void pj_login_via_access_token(struct IPJToDoHttpRequestDelegate delegate);

void pj_authorizations(struct IPJToDoHttpRequestDelegate delegate, const char *authorization);

void pj_access_token(struct IPJToDoHttpRequestDelegate delegate, const char *code, const char *client_id, const char *client_secret);

void pj_request_user_info(struct IPJToDoHttpRequestDelegate delegate);

void pj_logout(void);

void pj_create_repos(struct IPJToDoHttpRequestDelegate delegate);

void pj_get_repos(struct IPJToDoHttpRequestDelegate delegate, const char *repos_url);

void pj_delete_repos(struct IPJToDoHttpRequestDelegate delegate, const char *repos_url);

void pj_create_repos_file(struct IPJToDoHttpRequestDelegate delegate, const char *request_url, const char *path, const char *message, const char *content);

void pj_update_repos_file(struct IPJToDoHttpRequestDelegate delegate, const char *request_url, const char *path, const char *message, const char *content, const char *sha);

void pj_delete_repos_file(struct IPJToDoHttpRequestDelegate delegate, const char *request_url, const char *path, const char *message, const char *content, const char *sha);

void pj_get_repos_file(struct IPJToDoHttpRequestDelegate delegate, const char *request_url);

void pj_download_file(struct IPJToDoHttpRequestDelegate delegate, const char *request_url, const char *save_path);

void pj_update_db_connection(void);

void pj_init_data_base(void);

void pj_init_tables(void);

void pj_test_pal_from_rust(void);

void pj_init_hello_piaojin(void);

void pj_init_corelib(void);

struct PJToDoController *pj_create_PJToDoController(struct IPJToDoDelegate delegate);

void pj_insert_todo(struct PJToDoController *ptr, struct ToDoInsert *toDo);

void pj_delete_todo(struct PJToDoController *ptr, int32_t section, int32_t index, int32_t toDoId);

void pj_delete_todo_by_id(struct PJToDoController *ptr, int32_t toDoId);

void pj_update_todo(struct PJToDoController *ptr, const struct ToDoQuery *toDo);

void pj_find_todo(struct PJToDoController *ptr, int32_t toDoId);

void pj_fetch_todo_data(struct PJToDoController *ptr);

void pj_update_overdue_todos(const struct PJToDoController *ptr);

char *pj_todo_title_at_section(const struct PJToDoController *ptr, int32_t section);

const struct ToDoQuery *pj_todo_at_section(const struct PJToDoController *ptr, int32_t section, int32_t index);

int32_t pj_get_todo_number_of_sections(const struct PJToDoController *ptr);

int32_t pj_get_todo_counts_at_section(const struct PJToDoController *ptr, int32_t section);

const struct ToDoType *pj_todo_type_with_id(const struct PJToDoController *ptr, int32_t type_id);

const struct ToDoTag *pj_todo_tag_with_id(const struct PJToDoController *ptr, int32_t tag_id);

void pj_free_rust_PJToDoController(struct PJToDoController *ptr);

struct PJToDoSearchController *pj_create_PJToDoSearchController(struct IPJToDoSearchDelegate delegate);

void pj_find_todo_by_title(struct PJToDoSearchController *ptr, const char *title);

void pj_find_todo_like_title(struct PJToDoSearchController *ptr, const char *title);

const struct ToDoQuery *pj_search_todo_result_at_index(const struct PJToDoSearchController *ptr, int32_t index);

int32_t pj_search_todo_result_count(const struct PJToDoSearchController *ptr);

const struct ToDoType *pj_get_search_todo_type_with_id(const struct PJToDoSearchController *ptr, int32_t type_id);

const struct ToDoTag *pj_get_search_todo_tag_with_id(const struct PJToDoSearchController *ptr, int32_t tag_id);

void pj_free_rust_PJToDoSearchController(struct PJToDoSearchController *ptr);

struct PJToDoSettingsController *pj_create_PJToDoSettingsController(struct IPJToDoSettingsDelegate delegate);

void pj_insert_todo_settings(struct PJToDoSettingsController *ptr, struct ToDoSettingsInsert *toDoSettings);

void pj_delete_todo_settings(struct PJToDoSettingsController *ptr, int32_t toDoSettingsId);

void pj_update_todo_settings(struct PJToDoSettingsController *ptr, const struct ToDoSettings *toDoSettings);

void pj_fetch_todo_settings_data(struct PJToDoSettingsController *ptr);

const struct ToDoSettings *pj_todo_settings_at_index(const struct PJToDoSettingsController *ptr, int32_t index);

int32_t pj_get_todo_settings_count(const struct PJToDoSettingsController *ptr);

void pj_free_rust_PJToDoSettingsController(struct PJToDoSettingsController *ptr);

struct PJToDoTagController *pj_create_PJToDoTagController(struct IPJToDoTagDelegate delegate);

void pj_insert_todo_tag(struct PJToDoTagController *ptr, struct ToDoTagInsert *toDoTag);

void pj_delete_todo_tag(struct PJToDoTagController *ptr, int32_t toDoTagId);

void pj_update_todo_tag(struct PJToDoTagController *ptr, const struct ToDoTag *toDoTag);

void pj_find_todo_tag(struct PJToDoTagController *ptr, int32_t toDoTagId);

void pj_find_todo_tag_by_name(struct PJToDoTagController *ptr, const char *tag_name);

void pj_fetch_todo_tag_data(struct PJToDoTagController *ptr);

const struct ToDoTag *pj_todo_tag_at_index(const struct PJToDoTagController *ptr, int32_t index);

int32_t pj_get_todo_tag_count(const struct PJToDoTagController *ptr);

void pj_free_rust_PJToDoTagController(struct PJToDoTagController *ptr);

struct PJToDoTypeController *pj_create_PJToDoTypeController(struct IPJToDoTypeDelegate delegate);

void pj_insert_todo_type(struct PJToDoTypeController *ptr, struct ToDoTypeInsert *toDoType);

void pj_delete_todo_type(struct PJToDoTypeController *ptr, int32_t toDoTypeId);

void pj_update_todo_type(struct PJToDoTypeController *ptr, const struct ToDoType *toDoType);

void pj_find_todo_type(struct PJToDoTypeController *ptr, int32_t toDoTypeId);

void pj_find_todo_type_by_name(struct PJToDoTypeController *ptr, const char *type_name);

void pj_fetch_todo_type_data(struct PJToDoTypeController *ptr);

const struct ToDoType *pj_todo_type_at_index(const struct PJToDoTypeController *ptr, int32_t index);

int32_t pj_get_todo_type_count(const struct PJToDoTypeController *ptr);

void pj_free_rust_PJToDoTypeController(struct PJToDoTypeController *ptr);

extern void test_pal_from_Swift(void);

extern const char *get_db_path(void);

extern const char *get_authorization_str(void);

extern const char *get_access_token_str(void);

#endif /* app_bindings_h */
