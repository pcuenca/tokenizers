IOS=target/aarch64-apple-ios/release/libctokenizers.dylib
SIM_ARM=target/aarch64-apple-ios-sim/release/libctokenizers.dylib
SIM_X86=target/x86_64-apple-ios/release/libctokenizers.dylib
MAC_ARM=target/aarch64-apple-darwin/release/libctokenizers.dylib
MAC_X86=target/x86_64-apple-darwin/release/libctokenizers.dylib

# For local debugging / tests
HOST=target/debug/libctokenizers.dylib

FAT_SIM=lib/libctokenizers_iossimulator.dylib
FAT_MAC=lib/libctokenizers_macos.dylib
LIB_IOS=lib/libctokenizers.dylib

FRAMEWORK=lib/TokenizersC.xcframework

all: $(FRAMEWORK) $(HOST)

dev: $(HOST)

fat: $(FAT_SIM) $(FAT_MAC) $(LIB_IOS)

install: $(FRAMEWORK)
	# Copy to the SPM directory.
	# Potentially to be replaced with file upload to some repo/aws
	# (In that case we'll need the md5 hash for `binaryTarget`)
	cp -pR $(FRAMEWORK) swift/Tokenizers/Lib/`basename $(FRAMEWORK)`

$(FRAMEWORK): $(FAT_SIM) $(FAT_MAC) $(LIB_IOS)
	rm -rf $(FRAMEWORK)
	xcodebuild -create-xcframework -output $(FRAMEWORK) \
		-library $(LIB_IOS) -headers include \
		-library $(FAT_SIM) -headers include \
		-library $(FAT_MAC) -headers include

$(FAT_SIM): $(SIM_ARM) $(SIM_X86) include/module.modulemap
	mkdir -p lib
	lipo -create $(SIM_ARM) $(SIM_X86) -output $(FAT_SIM)

$(FAT_MAC): $(MAC_ARM) $(MAC_X86) include/module.modulemap
	mkdir -p lib
	lipo -create $(MAC_ARM) $(MAC_X86) -output $(FAT_MAC)

$(LIB_IOS): $(IOS) include/module.modulemap
	mkdir -p lib
	cp $(IOS) $(LIB_IOS)

$(IOS): src/
	cargo build --release --target aarch64-apple-ios

$(SIM_ARM): src/
	cargo build --release --target aarch64-apple-ios-sim
	
$(SIM_X86): src/
	cargo build --release --target x86_64-apple-ios
	
$(MAC_ARM): src/
	cargo build --release --target aarch64-apple-darwin

$(MAC_X86): src/
	cargo build --release --target x86_64-apple-darwin

$(HOST): src/
	cargo build 

