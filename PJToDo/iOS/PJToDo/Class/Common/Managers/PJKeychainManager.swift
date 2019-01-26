//
//  PJKeychainManager.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/13.
//  Copyright © 2019 piaojin. All rights reserved.
//

import Foundation

@objc public class PJKeychainManagerOC: NSObject {
    @objc public static func readSensitiveString(withService service: String, sensitiveKey: String, accessGroup: String? = nil) throws -> String {
        if let str = try? PJKeychainManager.readSensitiveString(withService: service, sensitiveKey: sensitiveKey) {
            return str
        }
        return ""
    }
}

public struct PJKeychainManager {
    // MARK: Types
    
    enum KeychainError: Error {
        case noSensitive
        case unexpectedSensitiveData
        case unexpectedItemData
        case unhandledError(status: OSStatus)
    }
    
    // MARK: Keychain access
    
    public static func readSensitiveString(withService service: String, sensitiveKey: String, accessGroup: String? = nil) throws -> String {
        /*
         Build a query to find the item that matches the service, account and
         access group.
         */
        var query = PJKeychainManager.keychainQuery(withService: service, sensitiveKey: sensitiveKey, accessGroup: accessGroup)
        query[kSecMatchLimit as String] = kSecMatchLimitOne
        query[kSecReturnAttributes as String] = kCFBooleanTrue
        query[kSecReturnData as String] = kCFBooleanTrue
        
        // Try to fetch the existing keychain item that matches the query.
        var queryResult: AnyObject?
        let status = withUnsafeMutablePointer(to: &queryResult) {
            SecItemCopyMatching(query as CFDictionary, UnsafeMutablePointer($0))
        }
        
        // Check the return status and throw an error if appropriate.
        guard status != errSecItemNotFound else { throw KeychainError.noSensitive }
        guard status == noErr else { throw KeychainError.unhandledError(status: status) }
        
        // Parse the password string from the query result.
        guard let existingItem = queryResult as? [String : AnyObject],
            let passwordData = existingItem[kSecValueData as String] as? Data,
            let password = String(data: passwordData, encoding: String.Encoding.utf8)
            else {
                throw KeychainError.unexpectedSensitiveData
        }
        
        return password
    }
    
    public static func saveSensitiveString(withService service: String, sensitiveKey: String, sensitiveString: String, accessGroup: String? = nil) throws {
        // Encode the password into an Data object.
        let encodedPassword = sensitiveString.data(using: String.Encoding.utf8)!
        
        do {
            // Check for an existing item in the keychain.
            try _ = PJKeychainManager.readSensitiveString(withService: service, sensitiveKey: sensitiveKey, accessGroup: accessGroup)
            
            // Update the existing item with the new password.
            var attributesToUpdate = [String : AnyObject]()
            attributesToUpdate[kSecValueData as String] = encodedPassword as AnyObject?
            
            let query = PJKeychainManager.keychainQuery(withService: service, sensitiveKey: sensitiveKey, accessGroup: accessGroup)
            let status = SecItemUpdate(query as CFDictionary, attributesToUpdate as CFDictionary)
            
            // Throw an error if an unexpected status was returned.
            guard status == noErr else { throw KeychainError.unhandledError(status: status) }
        }
        catch KeychainError.noSensitive {
            /*
             No password was found in the keychain. Create a dictionary to save
             as a new keychain item.
             */
            var newItem = PJKeychainManager.keychainQuery(withService: service, sensitiveKey: sensitiveKey, accessGroup: accessGroup)
            newItem[kSecValueData as String] = encodedPassword as AnyObject?
            
            // Add a the new item to the keychain.
            let status = SecItemAdd(newItem as CFDictionary, nil)
            
            // Throw an error if an unexpected status was returned.
            guard status == noErr else { throw KeychainError.unhandledError(status: status) }
        }
    }
    
    public static func renameSensitiveString(_ newSensitiveKey: String, oldSensitiveKey: String, service: String, sensitiveKey: String, accessGroup: String? = nil) throws {
        // Try to update an existing item with the new account name.
        var attributesToUpdate = [String : AnyObject]()
        attributesToUpdate[kSecAttrAccount as String] = newSensitiveKey as AnyObject?
        
        let query = PJKeychainManager.keychainQuery(withService: service, sensitiveKey: oldSensitiveKey, accessGroup: accessGroup)
        let status = SecItemUpdate(query as CFDictionary, attributesToUpdate as CFDictionary)
        
        // Throw an error if an unexpected status was returned.
        guard status == noErr || status == errSecItemNotFound else { throw KeychainError.unhandledError(status: status) }
    }
    
    public static func deleteItem(withService service: String, sensitiveKey: String, accessGroup: String? = nil) throws {
        // Delete the existing item from the keychain.
        let query = PJKeychainManager.keychainQuery(withService: service, sensitiveKey: sensitiveKey, accessGroup: accessGroup)
        let status = SecItemDelete(query as CFDictionary)
        
        // Throw an error if an unexpected status was returned.
        guard status == noErr || status == errSecItemNotFound else { throw KeychainError.unhandledError(status: status) }
    }
    
    // MARK: Convenience
    
    private static func keychainQuery(withService service: String, sensitiveKey: String, accessGroup: String? = nil) -> [String : AnyObject] {
        var query = [String : AnyObject]()
        query[kSecClass as String] = kSecClassGenericPassword
        query[kSecAttrService as String] = service as AnyObject?
        query[kSecAttrAccount as String] = sensitiveKey as AnyObject?
        
        if let accessGroup = accessGroup {
            query[kSecAttrAccessGroup as String] = accessGroup as AnyObject?
        }
        return query
    }
}
