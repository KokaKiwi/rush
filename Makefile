RUSTC			?=	rustc
RUSTCFLAGS		:=

RUSTDOC			?=	rustdoc
RUSTDOCFLAGS	:=

SOURCES			:=	$(shell find src -type f -name '*.rs')
MAIN_SOURCE		:=	src/main.rs

EXE_NAME		:=	rush
TEST_NAME		:=	rush_tests

all:			$(EXE_NAME)

$(EXE_NAME):	$(SOURCES)
	$(RUSTC) $(RUSTCFLAGS) -o $@ $(MAIN_SOURCE)

$(TEST_NAME):	$(SOURCES)
	$(RUSTC) $(RUSTCFLAGS) --test -o $@ $(MAIN_SOURCE)

doc:
	$(RUSTDOC) $(RUSTDOCFLAGS) $(MAIN_SOURCE)

test:			$(TEST_NAME)
	./$(TEST_NAME)

bench:			$(TEST_NAME)
	./$(TEST_NAME) --bench

clean:
	rm -f $(EXE_NAME) $(TEST_NAME)

.PHONY:			all doc test bench clean
