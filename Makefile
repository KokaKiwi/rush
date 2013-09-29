
RUSTC				?=	$(shell which rustc)
RUSTCFLAGS			:=

SOURCES				:=	$(shell find -type f -name "*.rs")

MAIN_SOURCE			:=	main.rs
MAIN_LL_SOURCE		:=	$(MAIN_SOURCE:.rs=.ll)

EXE_NAME			:=	rsh

all:				$(EXE_NAME) $(MAIN_LL_SOURCE)

$(EXE_NAME):		$(SOURCES)
	$(RUSTC) $(RUSTCFLAGS) -o $@ $(MAIN_SOURCE)

$(MAIN_LL_SOURCE):	$(SOURCES)
	$(RUSTC) $(RUSTCFLAGS) --emit-llvm -S -o $@ $(MAIN_SOURCE)

clean:
	rm -f $(EXE_NAME) $(MAIN_LL_SOURCE)
