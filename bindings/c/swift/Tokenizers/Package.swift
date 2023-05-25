// swift-tools-version: 5.8
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Tokenizers",
    platforms: [.iOS(.v16), .macOS(.v13)],
    products: [
        .library(name: "Tokenizers", targets: ["Tokenizers"]),
    ],
    targets: [
        .target(name: "Tokenizers", dependencies: ["TokenizersC"]),
        .binaryTarget(name: "TokenizersC", path: "Lib/TokenizersC.xcframework"),
        .testTarget(name: "TokenizersTests", dependencies: ["Tokenizers"], resources: [.process("Resources")]),
    ]
)
