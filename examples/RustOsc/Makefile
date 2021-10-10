
CHUGIN_NAME=rust2ck
CHUGIN_FILE=$(CHUGIN_NAME).chug
CHUGIN_DYLIB=target/debug/lib$(CHUGIN_NAME).dylib

CODESIGN_ID="Developer ID Application"
CWD=$(shell pwd)

$(CHUGIN_FILE): $(CHUGIN_DYLIB)
	cp $(CHUGIN_DYLIB) $(CHUGIN_FILE)
	codesign -s $(CODESIGN_ID) $(CHUGIN_FILE)

$(CHUGIN_DYLIB): 
	cargo build

.PHONY: run
run: $(CHUGIN_FILE)
	chuck -g$(CWD)/$(CHUGIN_FILE) -v5 test.ck

