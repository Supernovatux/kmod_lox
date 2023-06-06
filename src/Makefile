obj-m += kmain_lox.o
SRCS := chunk.c compile.c debug.c kmain.c memory.c object.c scanner.c table.c value.c vm.c buffio.c kmain.c
kmain_lox-y := $(SRCS:.c=.o)
KBUILD_CFLAGS += -W -O3 -g0 
all:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean
