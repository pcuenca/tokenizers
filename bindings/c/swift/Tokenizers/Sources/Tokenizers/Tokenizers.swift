import TokenizersC

// This is just a test.
// The API will be later seggregated to different modules for Encoders, Models, etc., just like in the other versions.

public class Model {
    let model: OpaquePointer
        
    public init(vocab: String, unknownToken: String) throws {
        guard let cModel = wordlevel_create(vocab, unknownToken) else { throw "Cannot create model, please check the vocab's file path." }
        self.model = cModel
    }
    
    deinit {
        wordlevel_destroy(model)
    }
}

public extension Model {
    func idForToken(token: String) -> UInt32? {
        let id = token_to_id(model, token)
        return id != UNKNOWN_TOKEN_ID ? id : nil
    }
}

extension String: Error {}
