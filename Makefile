
RUSTC				?=	$(shell which rustc)
RUSTCFLAGS			:=
RUSTOFLAGS			:=

SOURCES				:=	$(shell find rush -type f -name "*.rs")
SOURCES				+=	main.rs

MAIN_SOURCE			:=	main.rs
MAIN_LL_SOURCE		:=	$(MAIN_SOURCE:.rs=.ll)

EXE_NAME			:=	rsh
TEST_NAME			:=	rsh_tests

all:				$(EXE_NAME) $(TEST_NAME) $(MAIN_LL_SOURCE)

$(EXE_NAME):		$(SOURCES)
	$(RUSTC) $(RUSTCFLAGS) $(RUSTOFLAGS) -o $@ $(MAIN_SOURCE)

$(TEST_NAME):		$(SOURCES)
	$(RUSTC) $(RUSTCFLAGS) $(RUSTOFLAGS) --test -o $@ $(MAIN_SOURCE)

$(MAIN_LL_SOURCE):	$(SOURCES)
	$(RUSTC) $(RUSTCFLAGS) --emit-llvm -S -o $@ $(MAIN_SOURCE)

clean:
	rm -f $(EXE_NAME) $(TEST_NAME) $(MAIN_LL_SOURCE)
	rm -f start.ll start.o.ll

test:				$(TEST_NAME)
	@./$(TEST_NAME)
