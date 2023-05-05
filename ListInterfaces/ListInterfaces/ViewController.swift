//
//  ViewController.swift
//  ListInterfaces
//
//  Created by Pamela Audi on 09/03/2023.
//

import UIKit
import Foundation

struct interface_data {
    let name: String
    let description: String
    let index: UInt32
    let mac: [UInt8]
    let ips: String
    let flag: UInt32

    init(   name: String,
            description: String,
            index: UInt32,
            mac: [UInt8],
            ips: String,
            flag: UInt32
        )
    {
        self.name = name
        self.description = description
        self.index = index
        self.mac = mac
        self.ips = ips
        self.flag = flag
    }

    static func from(pointer: InterfaceData) -> interface_data {
        let name = pointer.name
        let description = pointer.description
        let index = pointer.index
        let mac = Array([pointer.mac.0, pointer.mac.1, pointer.mac.2, pointer.mac.3, pointer.mac.4, pointer.mac.5][0..<6])
        let ips = pointer.ips
        let flag = pointer.flag

        return interface_data(  name: String(cString: name!),
                                description: String(cString: description!),
                                index: index,
                                mac: mac,
                                ips: String(cString: ips!),
                                flag: flag
                        )
    }
}

class ViewController: UIViewController {
    
    func getInterfacesData () -> [interface_data] {
        let count = get_interface_data_count()
        let pointer: UnsafePointer<InterfaceData> = UnsafePointer(get_interface_data()!)
        let buffer = UnsafeBufferPointer<InterfaceData>(start: pointer, count: Int(count))
        let data = buffer.map { interface_data.from(pointer: $0)}
        return data
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
        let data_array = getInterfacesData()
        
        for interfaceData in data_array {
            print("Name: \(interfaceData.name)")
            print("Description: \(interfaceData.description)")
            print("Index: \(interfaceData.index)")
            print("MAC: \(interfaceData.mac)")
            print("IPs: \(interfaceData.ips)")
            print("Flag: \(interfaceData.flag)")
            print("\n")
            
        }
    }

}

