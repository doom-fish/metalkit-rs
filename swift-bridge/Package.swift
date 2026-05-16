// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "MetalKitBridge",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "MetalKitBridge",
            type: .static,
            targets: ["MetalKitBridge"])
    ],
    targets: [
        .target(
            name: "MetalKitBridge",
            path: "Sources/MetalKitBridge",
            publicHeadersPath: "include")
    ]
)
