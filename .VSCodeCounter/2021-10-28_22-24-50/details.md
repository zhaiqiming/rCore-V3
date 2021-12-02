# Details

Date : 2021-10-28 22:24:50

Directory /Users/zhaiqiming/i_can_own_os/rCore-Tutorial-v3

Total : 81 files,  5232 codes, 475 comments, 524 blanks, all 6231 lines

[summary](results.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [Dockerfile](/Dockerfile) | Docker | 31 | 5 | 4 | 40 |
| [Makefile](/Makefile) | Makefile | 6 | 0 | 3 | 9 |
| [README.md](/README.md) | Markdown | 2 | 0 | 0 | 2 |
| [easy-fs-fuse/Cargo.toml](/easy-fs-fuse/Cargo.toml) | TOML | 9 | 1 | 2 | 12 |
| [easy-fs-fuse/src/main.rs](/easy-fs-fuse/src/main.rs) | Rust | 151 | 7 | 10 | 168 |
| [easy-fs/Cargo.toml](/easy-fs/Cargo.toml) | TOML | 8 | 1 | 2 | 11 |
| [easy-fs/src/bitmap.rs](/easy-fs/src/bitmap.rs) | Rust | 63 | 2 | 8 | 73 |
| [easy-fs/src/block_cache.rs](/easy-fs/src/block_cache.rs) | Rust | 109 | 4 | 15 | 128 |
| [easy-fs/src/block_dev.rs](/easy-fs/src/block_dev.rs) | Rust | 5 | 0 | 2 | 7 |
| [easy-fs/src/efs.rs](/easy-fs/src/efs.rs) | Rust | 139 | 7 | 11 | 157 |
| [easy-fs/src/layout.rs](/easy-fs/src/layout.rs) | Rust | 400 | 36 | 12 | 448 |
| [easy-fs/src/lib.rs](/easy-fs/src/lib.rs) | Rust | 15 | 0 | 3 | 18 |
| [easy-fs/src/vfs.rs](/easy-fs/src/vfs.rs) | Rust | 177 | 11 | 13 | 201 |
| [os/Cargo.toml](/os/Cargo.toml) | TOML | 20 | 1 | 3 | 24 |
| [os/Makefile](/os/Makefile) | Makefile | 76 | 6 | 22 | 104 |
| [os/build.rs](/os/build.rs) | Rust | 5 | 0 | 2 | 7 |
| [os/src/config.rs](/os/src/config.rs) | Rust | 34 | 2 | 6 | 42 |
| [os/src/console.rs](/os/src/console.rs) | Rust | 26 | 0 | 8 | 34 |
| [os/src/drivers/block/mod.rs](/os/src/drivers/block/mod.rs) | Rust | 25 | 0 | 5 | 30 |
| [os/src/drivers/block/sdcard.rs](/os/src/drivers/block/sdcard.rs) | Rust | 494 | 218 | 41 | 753 |
| [os/src/drivers/block/virtio_blk.rs](/os/src/drivers/block/virtio_blk.rs) | Rust | 66 | 0 | 11 | 77 |
| [os/src/drivers/mod.rs](/os/src/drivers/mod.rs) | Rust | 2 | 0 | 1 | 3 |
| [os/src/entry.asm](/os/src/entry.asm) | RISC-V Assembly | 11 | 0 | 1 | 12 |
| [os/src/fs/inode.rs](/os/src/fs/inode.rs) | Rust | 143 | 7 | 9 | 159 |
| [os/src/fs/mod.rs](/os/src/fs/mod.rs) | Rust | 13 | 0 | 3 | 16 |
| [os/src/fs/pipe.rs](/os/src/fs/pipe.rs) | Rust | 156 | 3 | 8 | 167 |
| [os/src/fs/stdio.rs](/os/src/fs/stdio.rs) | Rust | 42 | 1 | 4 | 47 |
| [os/src/lang_items.rs](/os/src/lang_items.rs) | Rust | 11 | 0 | 2 | 13 |
| [os/src/linker-k210.ld](/os/src/linker-k210.ld) | LinkerScript | 46 | 0 | 7 | 53 |
| [os/src/linker-qemu.ld](/os/src/linker-qemu.ld) | LinkerScript | 46 | 0 | 7 | 53 |
| [os/src/main.rs](/os/src/main.rs) | Rust | 46 | 0 | 6 | 52 |
| [os/src/mm/address.rs](/os/src/mm/address.rs) | Rust | 191 | 5 | 13 | 209 |
| [os/src/mm/frame_allocator.rs](/os/src/mm/frame_allocator.rs) | Rust | 117 | 3 | 13 | 133 |
| [os/src/mm/heap_allocator.rs](/os/src/mm/heap_allocator.rs) | Rust | 40 | 0 | 6 | 46 |
| [os/src/mm/memory_set.rs](/os/src/mm/memory_set.rs) | Rust | 330 | 18 | 10 | 358 |
| [os/src/mm/mod.rs](/os/src/mm/mod.rs) | Rust | 26 | 0 | 2 | 28 |
| [os/src/mm/page_table.rs](/os/src/mm/page_table.rs) | Rust | 238 | 3 | 14 | 255 |
| [os/src/sbi.rs](/os/src/sbi.rs) | Rust | 36 | 0 | 8 | 44 |
| [os/src/syscall/fs.rs](/os/src/syscall/fs.rs) | Rust | 103 | 2 | 6 | 111 |
| [os/src/syscall/mod.rs](/os/src/syscall/mod.rs) | Rust | 35 | 0 | 5 | 40 |
| [os/src/syscall/process.rs](/os/src/syscall/process.rs) | Rust | 91 | 16 | 8 | 115 |
| [os/src/task/context.rs](/os/src/task/context.rs) | Rust | 14 | 0 | 4 | 18 |
| [os/src/task/manager.rs](/os/src/task/manager.rs) | Rust | 28 | 1 | 5 | 34 |
| [os/src/task/mod.rs](/os/src/task/mod.rs) | Rust | 61 | 17 | 11 | 89 |
| [os/src/task/pid.rs](/os/src/task/pid.rs) | Rust | 93 | 2 | 10 | 105 |
| [os/src/task/processor.rs](/os/src/task/processor.rs) | Rust | 82 | 2 | 12 | 96 |
| [os/src/task/switch.S](/os/src/task/switch.S) | RISC-V Assembly | 27 | 9 | 2 | 38 |
| [os/src/task/switch.rs](/os/src/task/switch.rs) | Rust | 7 | 0 | 2 | 9 |
| [os/src/task/task.rs](/os/src/task/task.rs) | Rust | 195 | 28 | 7 | 230 |
| [os/src/timer.rs](/os/src/timer.rs) | Rust | 14 | 0 | 4 | 18 |
| [os/src/trap/context.rs](/os/src/trap/context.rs) | Rust | 34 | 1 | 3 | 38 |
| [os/src/trap/mod.rs](/os/src/trap/mod.rs) | Rust | 101 | 6 | 11 | 118 |
| [os/src/trap/trap.S](/os/src/trap/trap.S) | RISC-V Assembly | 50 | 18 | 2 | 70 |
| [user/Cargo.toml](/user/Cargo.toml) | TOML | 8 | 1 | 2 | 11 |
| [user/Makefile](/user/Makefile) | Makefile | 17 | 0 | 7 | 24 |
| [user/src/bin/cat.rs](/user/src/bin/cat.rs) | Rust | 31 | 0 | 3 | 34 |
| [user/src/bin/cmdline_args.rs](/user/src/bin/cmdline_args.rs) | Rust | 13 | 0 | 3 | 16 |
| [user/src/bin/exit.rs](/user/src/bin/exit.rs) | Rust | 25 | 0 | 5 | 30 |
| [user/src/bin/fantastic_text.rs](/user/src/bin/fantastic_text.rs) | Rust | 40 | 0 | 4 | 44 |
| [user/src/bin/filetest_simple.rs](/user/src/bin/filetest_simple.rs) | Rust | 33 | 0 | 5 | 38 |
| [user/src/bin/forktest.rs](/user/src/bin/forktest.rs) | Rust | 30 | 0 | 4 | 34 |
| [user/src/bin/forktest2.rs](/user/src/bin/forktest2.rs) | Rust | 28 | 0 | 5 | 33 |
| [user/src/bin/forktest_simple.rs](/user/src/bin/forktest_simple.rs) | Rust | 23 | 2 | 3 | 28 |
| [user/src/bin/forktree.rs](/user/src/bin/forktree.rs) | Rust | 31 | 0 | 7 | 38 |
| [user/src/bin/hello_world.rs](/user/src/bin/hello_world.rs) | Rust | 9 | 0 | 2 | 11 |
| [user/src/bin/initproc.rs](/user/src/bin/initproc.rs) | Rust | 31 | 0 | 3 | 34 |
| [user/src/bin/matrix.rs](/user/src/bin/matrix.rs) | Rust | 61 | 0 | 7 | 68 |
| [user/src/bin/pipe_large_test.rs](/user/src/bin/pipe_large_test.rs) | Rust | 52 | 13 | 4 | 69 |
| [user/src/bin/pipetest.rs](/user/src/bin/pipetest.rs) | Rust | 31 | 9 | 4 | 44 |
| [user/src/bin/run_pipe_test.rs](/user/src/bin/run_pipe_test.rs) | Rust | 18 | 0 | 3 | 21 |
| [user/src/bin/sleep.rs](/user/src/bin/sleep.rs) | Rust | 26 | 0 | 4 | 30 |
| [user/src/bin/sleep_simple.rs](/user/src/bin/sleep_simple.rs) | Rust | 16 | 0 | 3 | 19 |
| [user/src/bin/stack_overflow.rs](/user/src/bin/stack_overflow.rs) | Rust | 14 | 0 | 3 | 17 |
| [user/src/bin/user_shell.rs](/user/src/bin/user_shell.rs) | Rust | 124 | 5 | 9 | 138 |
| [user/src/bin/usertests.rs](/user/src/bin/usertests.rs) | Rust | 36 | 0 | 4 | 40 |
| [user/src/bin/yield.rs](/user/src/bin/yield.rs) | Rust | 15 | 0 | 2 | 17 |
| [user/src/console.rs](/user/src/console.rs) | Rust | 31 | 0 | 9 | 40 |
| [user/src/lang_items.rs](/user/src/lang_items.rs) | Rust | 11 | 0 | 1 | 12 |
| [user/src/lib.rs](/user/src/lib.rs) | Rust | 94 | 2 | 12 | 108 |
| [user/src/linker.ld](/user/src/linker.ld) | LinkerScript | 29 | 0 | 3 | 32 |
| [user/src/syscall.rs](/user/src/syscall.rs) | Rust | 65 | 0 | 14 | 79 |

[summary](results.md)