#include <linux/kernel.h>
#include <linux/syscalls.h>

SYSCALL_DEFINE0(hellothesis)

{
    printk("Hello from the thesis syscall\n");
    return 0;
}
