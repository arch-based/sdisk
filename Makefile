# sgpasswd - suckless password generator utility

.POSIX:

include config.mk

all: sdisk

sdisk: $(SRC)
	$(CC) build --release
	
install: sdisk
	cp $(TARGETDIR)/sdisk $(DESTDIR)

clean:
	rm -rf $(TARGETDIR)/sdisk

uninstall:
	rm $(DESTDIR)/sdisk
