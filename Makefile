srcdir	= .
bindir	= /usr/local/bin

build	:
	gcc -o roll roll.c -O3 -W -Wall -Werror

install	: build
	/usr/bin/mkdir -p $(bindir)
	/usr/bin/install -c -s $(srcdir)/roll $(bindir)

clean	:
	rm -f roll
