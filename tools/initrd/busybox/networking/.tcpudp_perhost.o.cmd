cmd_networking/tcpudp_perhost.o := riscv64-linux-musl-gcc -Wp,-MD,networking/.tcpudp_perhost.o.d   -std=gnu99 -Iinclude -Ilibbb  -include include/autoconf.h -D_GNU_SOURCE -DNDEBUG -D_LARGEFILE_SOURCE -D_LARGEFILE64_SOURCE -D_FILE_OFFSET_BITS=64 -DBB_VER='"1.33.1"'  -Wall -Wshadow -Wwrite-strings -Wundef -Wstrict-prototypes -Wunused -Wunused-parameter -Wunused-function -Wunused-value -Wmissing-prototypes -Wmissing-declarations -Wno-format-security -Wdeclaration-after-statement -Wold-style-definition -finline-limit=0 -fno-builtin-strlen -fomit-frame-pointer -ffunction-sections -fdata-sections -fno-guess-branch-probability -funsigned-char -static-libgcc -falign-functions=1 -falign-jumps=1 -falign-labels=1 -falign-loops=1 -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-builtin-printf -Os     -DKBUILD_BASENAME='"tcpudp_perhost"'  -DKBUILD_MODNAME='"tcpudp_perhost"' -c -o networking/tcpudp_perhost.o networking/tcpudp_perhost.c

deps_networking/tcpudp_perhost.o := \
  networking/tcpudp_perhost.c \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/stdc-predef.h \
  include/libbb.h \
    $(wildcard include/config/feature/shadowpasswds.h) \
    $(wildcard include/config/use/bb/shadow.h) \
    $(wildcard include/config/selinux.h) \
    $(wildcard include/config/feature/utmp.h) \
    $(wildcard include/config/locale/support.h) \
    $(wildcard include/config/use/bb/pwd/grp.h) \
    $(wildcard include/config/lfs.h) \
    $(wildcard include/config/feature/buffers/go/on/stack.h) \
    $(wildcard include/config/feature/buffers/go/in/bss.h) \
    $(wildcard include/config/variable/arch/pagesize.h) \
    $(wildcard include/config/feature/verbose.h) \
    $(wildcard include/config/feature/etc/services.h) \
    $(wildcard include/config/feature/ipv6.h) \
    $(wildcard include/config/feature/seamless/xz.h) \
    $(wildcard include/config/feature/seamless/lzma.h) \
    $(wildcard include/config/feature/seamless/bz2.h) \
    $(wildcard include/config/feature/seamless/gz.h) \
    $(wildcard include/config/feature/seamless/z.h) \
    $(wildcard include/config/float/duration.h) \
    $(wildcard include/config/feature/check/names.h) \
    $(wildcard include/config/feature/prefer/applets.h) \
    $(wildcard include/config/long/opts.h) \
    $(wildcard include/config/feature/pidfile.h) \
    $(wildcard include/config/feature/syslog.h) \
    $(wildcard include/config/feature/syslog/info.h) \
    $(wildcard include/config/warn/simple/msg.h) \
    $(wildcard include/config/feature/individual.h) \
    $(wildcard include/config/ash.h) \
    $(wildcard include/config/sh/is/ash.h) \
    $(wildcard include/config/bash/is/ash.h) \
    $(wildcard include/config/hush.h) \
    $(wildcard include/config/sh/is/hush.h) \
    $(wildcard include/config/bash/is/hush.h) \
    $(wildcard include/config/echo.h) \
    $(wildcard include/config/printf.h) \
    $(wildcard include/config/test.h) \
    $(wildcard include/config/test1.h) \
    $(wildcard include/config/test2.h) \
    $(wildcard include/config/kill.h) \
    $(wildcard include/config/killall.h) \
    $(wildcard include/config/killall5.h) \
    $(wildcard include/config/chown.h) \
    $(wildcard include/config/ls.h) \
    $(wildcard include/config/xxx.h) \
    $(wildcard include/config/route.h) \
    $(wildcard include/config/feature/hwib.h) \
    $(wildcard include/config/desktop.h) \
    $(wildcard include/config/feature/crond/d.h) \
    $(wildcard include/config/feature/setpriv/capabilities.h) \
    $(wildcard include/config/run/init.h) \
    $(wildcard include/config/feature/securetty.h) \
    $(wildcard include/config/pam.h) \
    $(wildcard include/config/use/bb/crypt.h) \
    $(wildcard include/config/feature/adduser/to/group.h) \
    $(wildcard include/config/feature/del/user/from/group.h) \
    $(wildcard include/config/ioctl/hex2str/error.h) \
    $(wildcard include/config/feature/editing.h) \
    $(wildcard include/config/feature/editing/history.h) \
    $(wildcard include/config/feature/tab/completion.h) \
    $(wildcard include/config/shell/ash.h) \
    $(wildcard include/config/shell/hush.h) \
    $(wildcard include/config/feature/editing/savehistory.h) \
    $(wildcard include/config/feature/username/completion.h) \
    $(wildcard include/config/feature/editing/vi.h) \
    $(wildcard include/config/feature/editing/save/on/exit.h) \
    $(wildcard include/config/pmap.h) \
    $(wildcard include/config/feature/show/threads.h) \
    $(wildcard include/config/feature/ps/additional/columns.h) \
    $(wildcard include/config/feature/topmem.h) \
    $(wildcard include/config/feature/top/smp/process.h) \
    $(wildcard include/config/pgrep.h) \
    $(wildcard include/config/pkill.h) \
    $(wildcard include/config/pidof.h) \
    $(wildcard include/config/sestatus.h) \
    $(wildcard include/config/unicode/support.h) \
    $(wildcard include/config/feature/mtab/support.h) \
    $(wildcard include/config/feature/clean/up.h) \
    $(wildcard include/config/feature/devfs.h) \
  include/platform.h \
    $(wildcard include/config/werror.h) \
    $(wildcard include/config/big/endian.h) \
    $(wildcard include/config/little/endian.h) \
    $(wildcard include/config/nommu.h) \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/limits.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/features.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/alltypes.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/limits.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/byteswap.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/stdint.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/stdint.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/endian.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/stdbool.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/unistd.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/posix.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/ctype.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/dirent.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/dirent.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/errno.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/errno.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/fcntl.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/fcntl.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/inttypes.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/netdb.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/netinet/in.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/socket.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/socket.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/setjmp.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/setjmp.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/signal.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/signal.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/paths.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/stdio.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/stdlib.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/alloca.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/stdarg.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/stddef.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/string.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/strings.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/libgen.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/poll.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/poll.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/ioctl.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/ioctl.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/ioctl_fix.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/mman.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/mman.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/resource.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/time.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/select.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/resource.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/stat.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/stat.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/types.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/sysmacros.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/wait.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/termios.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/termios.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/time.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/param.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/pwd.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/grp.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/mntent.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/statfs.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/sys/statvfs.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/bits/statfs.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/utmp.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/utmpx.h \
  /home/dry/riscv64-linux-musl-cross/riscv64-linux-musl/include/arpa/inet.h \
  include/pwd_.h \
  include/grp_.h \
  include/shadow_.h \
  include/xatonum.h \
  networking/tcpudp_perhost.h \

networking/tcpudp_perhost.o: $(deps_networking/tcpudp_perhost.o)

$(deps_networking/tcpudp_perhost.o):
