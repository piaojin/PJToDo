//
//  RustBindings.h
//  RustIntToSwift
//
//  Created by Zoey Weng on 12/22/19.
//  Copyright © 2019 piaojin. All rights reserved.
//

#ifndef RustBindings_h
#define RustBindings_h

#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct Person Person;

typedef struct {
  void *user;
  void (*destroy)(void *user);
  void (*did_update)(void *user, bool isSuccess);
} IPersonUIControllerDelegate;

typedef struct {
  IPersonUIControllerDelegate delegate;
  Person *person;
} IPersonUIController;

extern int32_t addition(int32_t, int32_t);

extern char *str_from_rust(void);

extern IPersonUIController *create_person_uicontroller(IPersonUIControllerDelegate delegate);

extern void free_person_uicontroller(IPersonUIController *);

extern void update_person(IPersonUIController *, int32_t, const char *);

extern int32_t get_person_age(const IPersonUIController *);

extern char * get_person_name(const IPersonUIController *);
#endif /* RustBindings_h */
