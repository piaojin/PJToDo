//
//  ViewController.swift
//  RustIntToSwift
//
//  Created by Zoey Weng on 12/22/19.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

    var textView = UITextView()
    
    var personUIController: PersonUIController?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
        view.backgroundColor = .white
        initView()
        setUpUIController()
        initData()
    }
    
    private func initView() {
        textView.textColor = UIColor.systemBlue
        textView.font = UIFont.boldSystemFont(ofSize: 18)
        textView.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(textView)
        textView.heightAnchor.constraint(equalToConstant: 200).isActive = true
        textView.leadingAnchor.constraint(equalTo: view.leadingAnchor).isActive = true
        textView.trailingAnchor.constraint(equalTo: view.trailingAnchor).isActive = true
        textView.centerYAnchor.constraint(equalTo: view.centerYAnchor).isActive = true
        textView.centerXAnchor.constraint(equalTo: view.centerXAnchor).isActive = true
    }
    
    private func setUpUIController() {
        personUIController = PersonUIController(self)
    }
    
    private func initData() {
        textView.text = "Rust int to Swift: \(addition(6, 6)) \n"
        textView.insertText("String from Rust: \(String(cString: str_from_rust(), encoding: .utf8) ?? "") \n")
        
        personUIController?.updatePerson(20, name: "Chad")
    }
}

extension ViewController: PersonUIControllerDelegate {
    func didUpdate(_ isSuccess: Bool) {
        print("didUpdate: \(isSuccess)")
        textView.insertText("age: \(personUIController?.getPersonAge() ?? 0) \n")
        textView.insertText("name: \(personUIController?.getPersonName() ?? "") \n")
    }
}

