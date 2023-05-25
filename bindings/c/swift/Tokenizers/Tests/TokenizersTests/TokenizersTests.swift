import XCTest
@testable import Tokenizers

final class TokenizersTests: XCTestCase {
    var vocab: String {
        let url = Bundle.module.url(forResource: "gpt2-vocab", withExtension: "json")!
        return url.path
    }

    func testExample() throws {
        let model = try Model(vocab: vocab, unknownToken: "[UNK]")
        let id = model.idForToken(token: "hello")
        XCTAssertNotNil(id)
        XCTAssertEqual(id!, 31373)
        
        let shouldBeNil = model.idForToken(token: "thistokenisnotinthevocab")
        XCTAssertNil(shouldBeNil)
    }
}
