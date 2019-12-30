//
//  PersonUIController.m
//  RustToSwift
//
//  Created by Zoey Weng on 12/23/19.
//  Copyright © 2019 piaojin. All rights reserved.
//

#import "PersonUIController.h"
#import "RustBindings.h"

void destroy(void *user) {
    NSLog(@"destroy");
}

void didUpdate(void *user, bool isSuccess) {
    NSLog(@"didUpdate");
    PersonUIController *personUIController = (__bridge PersonUIController *)user;
    if (personUIController != nil && [personUIController.delegate respondsToSelector:@selector(didUpdate:)]) {
        [personUIController.delegate didUpdate:isSuccess];
    }
}

@interface PersonUIController ()

@property (nonatomic, assign) IPersonUIController *iPersonUIController;

@property (nonatomic, assign) IPersonUIControllerDelegate *iPersonUIControllerDelegate;

@end

@implementation PersonUIController

- (instancetype) init: (id<PersonUIControllerDelegate>) delegate {
    if (self = [super init]) {
        [self setUpUIController];
        self.delegate = delegate;
    }
    return self;
}

- (void) setUpUIController {
    self.iPersonUIControllerDelegate = (IPersonUIControllerDelegate *)malloc(sizeof(IPersonUIControllerDelegate));
    self.iPersonUIControllerDelegate->user = (__bridge void *)(self);
    self.iPersonUIControllerDelegate->destroy = destroy;
    self.iPersonUIControllerDelegate->did_update = didUpdate;
    self.iPersonUIController = create_person_uicontroller(*(self.iPersonUIControllerDelegate));
}

- (void) updatePerson: (int32_t)age name: (NSString *) name {
    update_person(self.iPersonUIController, age, [name UTF8String]);
}

- (NSString *) getPersonName {
    return [NSString stringWithUTF8String: get_person_name(self.iPersonUIController)];
}

- (int32_t) getPersonAge {
    return get_person_age(self.iPersonUIController);
}

- (void)dealloc {
    free_person_uicontroller(self.iPersonUIController);
    free(self.iPersonUIControllerDelegate);
}

@end
