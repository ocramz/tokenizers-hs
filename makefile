# # from https://github.com/aisamanra/rust-haskell-ffi/blob/master/Makefile

RC = LD_LIBRARY_PATH=/usr/local/lib rustc
#GHC = ghc
# STACK = stack build

all: main

libtokenizers.a: ffi.rs
	$(RC) --crate-type staticlib ffi.rs

main: libtokenizers.a
	stack build


# libfact.a: fact.rs
# 	$(RC) --crate-type staticlib fact.rs

# libpoint.a: point.rs
# 	$(RC) --crate-type staticlib point.rs

# main: libfact.a libpoint.a main.hs
# 	$(GHC) main.hs libfact.a libpoint.a -lpthread -o main

# clean:
# 	rm -f libfact.a libpoint.a main.hi main.o main
