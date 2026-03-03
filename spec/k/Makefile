# Makefile for X-Lang K Framework Specification

# K Framework directory - adjust this to your K installation
K_HOME ?= $(HOME)/.k
K_BIN ?= $(K_HOME)/bin

# K binaries
K_COMPILE = $(K_BIN)/kompile
K_RUN = $(K_BIN)/krun
K_PROVE = $(K_BIN)/kprove

# X-Lang specification
XLANG_SYNTAX = x-lang-syntax.k
XLANG_MAIN = x-lang.k
XLANG_DEFS = $(XLANG_SYNTAX) $(XLANG_MAIN)

# Compilation output
K_OUTPUT = .k-xlang
K_COMPILED = $(K_OUTPUT)/x-lang-kompiled

# Test files
TEST_DIR = tests
TEST_FILES = $(wildcard $(TEST_DIR)/*.x)
TEST_EXPECTED = $(patsubst %.x,%.expected,$(TEST_FILES))
TEST_RESULTS = $(patsubst %.x,%.out,$(TEST_FILES))

# Default target
.PHONY: all
all: build test

# Build the K specification
.PHONY: build
build: $(K_COMPILED)

$(K_COMPILED): $(XLANG_DEFS)
	@echo "Compiling X-Lang K specification..."
	$(K_COMPILE) --backend llvm --directory $(K_OUTPUT) $(XLANG_MAIN)
	@echo "Compilation complete."

# Clean build artifacts
.PHONY: clean
clean:
	@echo "Cleaning K build artifacts..."
	rm -rf $(K_OUTPUT)
	rm -f $(TEST_RESULTS)
	@echo "Clean complete."

# Run a specific test file
.PHONY: run
run: build
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make run FILE=<test-file>"; \
		exit 1; \
	fi
	@echo "Running $(FILE)..."
	$(K_RUN) --directory $(K_OUTPUT) $(FILE)

# Run all tests
.PHONY: test
test: build $(TEST_RESULTS)
	@echo "All tests complete."

$(TEST_DIR)/%.out: $(TEST_DIR)/%.x
	@echo "Testing $<..."
	$(K_RUN) --directory $(K_OUTPUT) $< > $@
	@if [ -f $(patsubst %.out,%.expected,$@) ]; then \
		diff -u $(patsubst %.out,%.expected,$@) $@; \
		if [ $$? -eq 0 ]; then \
			echo "  PASS: $<"; \
		else \
			echo "  FAIL: $< (output differs from expected)"; \
		fi \
	else \
		echo "  (no expected output for $<)"; \
		cat $@; \
	fi

# Generate expected output for a test
.PHONY: gen-expected
gen-expected: build
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make gen-expected FILE=<test-file>"; \
		exit 1; \
	fi
	@echo "Generating expected output for $(FILE)..."
	$(K_RUN) --directory $(K_OUTPUT) $(FILE) > $(patsubst %.x,%.expected,$(FILE))
	@echo "Generated $(patsubst %.x,%.expected,$(FILE))"

# Prove a specification
.PHONY: prove
prove: build
	@if [ -z "$(SPEC)" ]; then \
		echo "Usage: make prove SPEC=<spec-file>"; \
		exit 1; \
	fi
	@echo "Proving $(SPEC)..."
	$(K_PROVE) --directory $(K_OUTPUT) $(SPEC)

# Interactive K shell
.PHONY: shell
shell: build
	$(K_RUN) --directory $(K_OUTPUT) --interactive

# Help
.PHONY: help
help:
	@echo "X-Lang K Framework Specification"
	@echo ""
	@echo "Targets:"
	@echo "  all          - Build and test (default)"
	@echo "  build        - Compile the K specification"
	@echo "  clean        - Remove build artifacts"
	@echo "  test         - Run all tests"
	@echo "  run          - Run a specific test (FILE=<path>)"
	@echo "  gen-expected - Generate expected output (FILE=<path>)"
	@echo "  prove        - Prove a specification (SPEC=<path>)"
	@echo "  shell        - Interactive K shell"
	@echo "  help         - Show this help"
	@echo ""
	@echo "Variables:"
	@echo "  K_HOME       - Path to K Framework installation (default: ~/.k)"
