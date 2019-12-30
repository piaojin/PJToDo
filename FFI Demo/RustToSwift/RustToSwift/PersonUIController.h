//
//  PersonUIController.h
//  RustToSwift
//
//  Created by Zoey Weng on 12/23/19.
//  Copyright © 2019 piaojin. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "RustBindings.h"

NS_ASSUME_NONNULL_BEGIN

@protocol PersonUIControllerDelegate <NSObject>

- (void) didUpdate:(BOOL) isSuccess;

@end

@interface PersonUIController : NSObject

@property (nonatomic, weak) id<PersonUIControllerDelegate> delegate;

- (instancetype) init: (id<PersonUIControllerDelegate>) delegate;

- (void) updatePerson: (int32_t)age name: (NSString *) name;

- (NSString *) getPersonName;

- (int32_t) getPersonAge;

@end

NS_ASSUME_NONNULL_END
