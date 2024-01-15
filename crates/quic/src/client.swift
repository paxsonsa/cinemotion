// The Swift Programming Language
// https://docs.swift.org/swift-book
// 
// Swift Argument Parser
// https://swiftpackageindex.com/apple/swift-argument-parser/documentation

import ArgumentParser
import Network

@main
struct quicdemo: ParsableCommand {
    mutating func run() throws {
        let endpoint = NWEndpoint.hostPort(host: "127.0.0.1", port: 4567)
        
        let options =  NWProtocolQUIC.Options()
        options.direction = .bidirectional
        let parameters = NWParameters(quic: options)
        let connection = NWConnection(to: endpoint, using: parameters)
        connection.stateUpdateHandler = { state in
            print("hello")
            switch state {
            case .setup:
                print("setup")
            case .waiting(let err):
                print("waiting: \(err)")
            case .preparing:
                print("preping")
            case .ready:
                print("ready")
            case .failed(_):
                print("failure")
            case .cancelled:
                print("cancelled")
            @unknown default:
                print("")
            }
        }
        
        let queue = DispatchQueue(label: "conn")
        connection.start(queue: queue)
        
        print("waiting.")
        while true {
            switch connection.state {
            case .failed(let err):
                print("error: \(err)")
                break
            case .cancelled:
                print("cancelled")
                break
                
            default:
                continue
            }
            break
        }
        print("Done")
    }
}
