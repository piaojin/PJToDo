//
//  PJStringExtension.swift
//  PJToDo
//
//  Created by Zoey Weng on 2018/12/4.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

public extension String {
    public static func create(cString: UnsafeMutablePointer<Int8>, encoding: String.Encoding = .utf8) -> String {
        //get string pointer from rust and use the pointer to create a Swift String and then free the rust string
        if let str = String(cString: cString, encoding: encoding) {
            free_rust_string(cString)
            return str
        }
        return ""
    }
    
    public static func convertToBase64String(str: String, encoding: String.Encoding = .utf8) -> String {
        return self.create(cString: convertStrToBase64Str(str), encoding: encoding)
    }
    
    public func pj_bridgeObjectiveC() -> NSString {
        return NSString(string: self)
    }
    
    public func pj_boundingSizeBySize(_ size: CGSize, font: UIFont) -> CGSize {
        if self.isEmpty { return CGSize.zero }
        return pj_attributedTextConstrainedToSize(NSAttributedString(string: self, attributes: [NSAttributedStringKey.font: font]), size: size, numberOfLines: 0)
    }
    
    public func pj_sizeByAttributes(_ attributes: [NSAttributedStringKey: Any]?) -> CGSize {
        return self.pj_bridgeObjectiveC().size(withAttributes: attributes)
    }
}

public func pj_attributedTextConstrainedToSize(_ attributedText: NSAttributedString, size: CGSize, numberOfLines: Int) -> CGSize {
    if attributedText.length == 0 {
        return CGSize.zero
    }
    
    var range = CFRangeMake(0, 0)
    var constraintSize = CGSize(width: size.width, height: size.height)
    let framesetter = CTFramesetterCreateWithAttributedString(attributedText)
    if numberOfLines == 1 {
        constraintSize = CGSize(width: CGFloat.greatestFiniteMagnitude, height: CGFloat.greatestFiniteMagnitude)
    }
    else if numberOfLines > 0 {
        let path = CGMutablePath()
        path.addRect(CGRect(x: 0, y: 0, width: constraintSize.width, height: constraintSize.height))
        let frame = CTFramesetterCreateFrame(framesetter, CFRangeMake(0, 0), path, nil)
        let lines = CTFrameGetLines(frame)
        if CFArrayGetCount(lines) > 0 {
            let lastVisibleLineIndex = min(numberOfLines, CFArrayGetCount(lines)) - 1
            let lastVisibleLine = CFArrayGetValueAtIndex(lines, lastVisibleLineIndex)
            let rangeToLayout = CTLineGetStringRange(unsafeBitCast(lastVisibleLine, to: CTLine.self))
            range = CFRangeMake(0, rangeToLayout.location + rangeToLayout.length)
        }
    }
    let newSize = CTFramesetterSuggestFrameSizeWithConstraints(framesetter, range, nil, constraintSize, nil)
    return CGSize(width: ceil(newSize.width), height: ceil(newSize.height))
}

