//
//  PJBaseEmptyView.swift
//  Swift3Learn
//
//  Created by 飘金 on 2017/4/11.
//  Copyright © 2017年 飘金. All rights reserved.
//

import UIKit

public protocol PJBaseEmptyViewDelegate : NSObjectProtocol {
    /**
     为空时点击回调
     */
    func emptyClick()
}

open class PJBaseEmptyView: PJBaseView {
    
    lazy var emptyView: UIImageView = {
      let emptyView = UIImageView(image: UIImage(named: "empty_box"))
        emptyView.translatesAutoresizingMaskIntoConstraints = false
        return emptyView
    }()
    
    weak open var delegate:PJBaseEmptyViewDelegate?
    
    public override init(frame: CGRect) {
        super.init(frame: frame)
        self.initView()
    }
    
    required public init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
        self.initView()
    }
    
    convenience init() {
        self.init(frame: .zero)
    }
    
    private func initView() {
        self.addSubview(self.emptyView)
        self.emptyView.widthAnchor.constraint(equalToConstant: 70).isActive = true
        self.emptyView.heightAnchor.constraint(equalToConstant: 70).isActive = true
        self.emptyView.centerXAnchor.constraint(equalTo: self.centerXAnchor).isActive = true
        self.emptyView.centerYAnchor.constraint(equalTo: self.centerYAnchor).isActive = true
    }
    
    //点击空页面
    override open func viewClick(){
        self.delegate?.emptyClick()
    }
    
    /**
     *   设置出错时的提示文字
     */
    open func setEmptyText(text: String?) {
        self.setLabelText(text: text)
    }
}
