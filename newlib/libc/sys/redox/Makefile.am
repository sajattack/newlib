AUTOMAKE_OPTIONS = cygnus
INCLUDES = $(NEWLIB_CFLAGS) $(CROSS_CFLAGS) $(TARGET_CFLAGS)
AM_CCASFLAGS = $(INCLUDES)

noinst_LIBRARIES = lib.a

extra_objs =

lib_a_LIBADD = $(extra_objs)
lib_a_SOURCES = \
    crt0.c
lib_a_DEPENDENCIES = $(extra_objs)
lib_a_CCASFLAGS = $(AM_CCASFLAGS)
lib_a_CFLAGS = $(AM_CFLAGS)

rust_sources = \
    src/lib.rs \
    src/macros.rs \
    src/process.rs \
    src/file.rs \
    src/time.rs \
    src/unimpl.rs

$(srcdir)/target/x86_64-unknown-redox/release/libnewlib_redox.a: Cargo.toml $(rust_sources)
	cd $(srcdir); CC=x86_64-elf-redox-gcc xargo build --target x86_64-unknown-redox --release

# Modified from sys/phoenix
lib.a: $(extra_objs) $(srcdir)/target/x86_64-unknown-redox/release/libnewlib_redox.a
	rm -f $@
	rm -rf tmp
	mkdir tmp
	cd tmp; ar x $(srcdir)/target/x86_64-unknown-redox/release/libnewlib_redox.a
	$(AR) $(AR_FLAGS) $@ tmp/*.o $(extra_objs)
	$(RANLIB) $@
	rm -rf tmp

if MAY_SUPPLY_SYSCALLS
all: crt0.o
endif

ACLOCAL_AMFLAGS = -I ../../..
CONFIG_STATUS_DEPENDENCIES = $(newlib_basedir)/configure.host

include $(srcdir)/../../../Makefile.shared
