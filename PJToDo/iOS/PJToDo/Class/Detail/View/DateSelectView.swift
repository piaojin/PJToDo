//
//  DateSelectView.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/8.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

class DateSelectView: UIView {

    lazy var datePicker: UIDatePicker = {
        let datePicker = UIDatePicker()
        datePicker.translatesAutoresizingMaskIntoConstraints = false
        return datePicker
    }()
    
    lazy var dateFormatter: DateFormatter = {
        let dateFormatter = DateFormatter()
        dateFormatter.dateFormat = "yyyy-MM-dd h:m"
        return dateFormatter
    }()
    
    var dateString: String {
        let dateString = self.dateFormatter.string(from: self.datePicker.date)
        return dateString
    }
    
    var dateValueChangedBlock: ((String) -> ())?
    
    var doneClosure: ((String) -> ())?
    
    lazy var doneView: UIView = {
        let doneView = UIView()
        doneView.translatesAutoresizingMaskIntoConstraints = false
        doneView.isUserInteractionEnabled = true
        doneView.backgroundColor = UIColor.colorWithRGB(red: 249, green: 249, blue: 249)
        
        let doneButton = UIButton()
        doneButton.setTitle("Done", for: .normal)
        doneButton.setTitleColor(UIColor.colorWithRGB(red: 0, green: 123, blue: 249), for: .normal)
        doneButton.translatesAutoresizingMaskIntoConstraints = false
        doneView.addSubview(doneButton)
        doneButton.topAnchor.constraint(equalTo: doneView.topAnchor).isActive = true
        doneButton.bottomAnchor.constraint(equalTo: doneView.bottomAnchor).isActive = true
        doneButton.trailingAnchor.constraint(equalTo: doneView.trailingAnchor, constant: -5).isActive = true
        doneButton.widthAnchor.constraint(equalToConstant: 60).isActive = true
        doneButton.addTarget(self, action: #selector(doneAction), for: .touchUpInside)
        return doneView
    }()
    
    override init(frame: CGRect) {
        super.init(frame: .zero)
        self.initView()
        self.initData()
    }
    
    required init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
    }

    private func initView() {
        self.addSubview(self.doneView)
        self.doneView.topAnchor.constraint(equalTo: self.topAnchor).isActive = true
        self.doneView.leadingAnchor.constraint(equalTo: self.leadingAnchor).isActive = true
        self.doneView.trailingAnchor.constraint(equalTo: self.trailingAnchor).isActive = true
        self.doneView.heightAnchor.constraint(equalToConstant: 40).isActive = true
        
        self.addSubview(self.datePicker)
        self.datePicker.topAnchor.constraint(equalTo: self.doneView.bottomAnchor).isActive = true
        self.datePicker.leadingAnchor.constraint(equalTo: self.leadingAnchor).isActive = true
        self.datePicker.trailingAnchor.constraint(equalTo: self.trailingAnchor).isActive = true
        self.datePicker.bottomAnchor.constraint(equalTo: self.bottomAnchor).isActive = true
    }
    
    private func initData() {
        self.datePicker.addTarget(self, action: #selector(dateValueChangedAction(datePicker:)), for: .valueChanged)
    }
    
    func setDate(_ dateString: String, animated: Bool) {
        if let date = self.dateFormatter.date(from: dateString) {
            self.datePicker.setDate(date, animated: animated)
        }
    }
    
    @objc private func dateValueChangedAction(datePicker: UIDatePicker) {
        self.dateValueChangedBlock?(self.dateString)
    }
    
    @objc private func doneAction() {
        self.doneClosure?(self.dateString)
    }
}
