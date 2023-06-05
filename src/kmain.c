#include "kmain.h"
#include "loxentry.h"
#include "buffio.h"
#include "chunk.h"
#include "common.h"
#include "linux/completion.h"
#include "linux/slab.h"
#include "vm.h"
#include <linux/atomic.h>
#include <linux/cdev.h>
#include <linux/delay.h>
#include <linux/device.h>
#include <linux/fs.h>
#include <linux/init.h>
#include <linux/kernel.h>
#include <linux/kthread.h>
#include <linux/module.h>
#include <linux/sched.h>
#include <linux/string.h>
#include <linux/uaccess.h>
static int device_open(struct inode *, struct file *);
static int device_release(struct inode *, struct file *);
static ssize_t device_read(struct file *, char __user *, size_t, loff_t *);
static ssize_t device_write(struct file *, const char __user *, size_t,
			    loff_t *);
struct lox_data_t *lox_data = NULL;
static struct task_struct *interpreter_thread;
#define SUCCESS 0
MODULE_LICENSE("GPL");
MODULE_AUTHOR("Thulashitharan");
MODULE_DESCRIPTION("A lox module");
MODULE_VERSION("0.01");
static int major; /* major number assigned to our device driver */
static atomic_t already_open = ATOMIC_INIT(CDEV_NOT_USED);
static struct class *cls;
static struct file_operations chardev_fops = {
	.read = device_read,
	.write = device_write,
	.open = device_open,
	.release = device_release,
};
buffio *lox_input_buffer = NULL;
buffio *lox_output_buffer = NULL;

static int __init chardev_init(void)
{
	major = register_chrdev(0, DEVICE_NAME, &chardev_fops);
	if (major < 0) {
		pr_alert("Registering char device failed with %d\n", major);
		return major;
	}
	cls = class_create(THIS_MODULE, DEVICE_NAME);
	device_create(cls, NULL, MKDEV(major, 0), NULL, DEVICE_NAME);
	pr_info("Device created on /dev/%s\n", DEVICE_NAME);
	lox_input_buffer = kmalloc(sizeof(buffio), GFP_KERNEL);
	lox_output_buffer = kmalloc(sizeof(buffio), GFP_KERNEL);
	initBuff(lox_input_buffer, NULL, NULL);
	initBuff(lox_output_buffer, NULL, NULL);
	return SUCCESS;
}
static void __exit chardev_exit(void)
{
	device_destroy(cls, MKDEV(major, 0));
	class_destroy(cls);
	/* Unregister the device */
	unregister_chrdev(major, DEVICE_NAME);
	/* Freeing buffer memory */
	freeBuff(lox_input_buffer);
	freeBuff(lox_output_buffer);
}

static int device_open(struct inode *inode, struct file *file)
{
	if (atomic_cmpxchg(&already_open, CDEV_NOT_USED, CDEV_EXCLUSIVE_OPEN))
		return -EBUSY;
	try_module_get(THIS_MODULE);
	return SUCCESS;
}
static int device_release(struct inode *inode, struct file *file)
{
	atomic_set(&already_open, CDEV_NOT_USED);
	module_put(THIS_MODULE);
	return SUCCESS;
}

static ssize_t device_read(struct file *filp, char __user *buffer,
			   size_t length, loff_t *offset)
{
	int bytes_read = 0;
	char *msg_ptr = NULL;
	lox_data_free(&lox_data);
	msg_ptr = lox_output_buffer->buffer;
	if (!*(msg_ptr + *offset)) {
		*offset = 0;
		return 0;
	}
	msg_ptr += *offset;
	while (length && *msg_ptr) {
		put_user(*(msg_ptr), buffer++);
		msg_ptr++;
		length--;
		bytes_read++;
	}
	*offset += bytes_read;
	return bytes_read;
}
static ssize_t device_write(struct file *filp, const char __user *buff,
			    size_t len, loff_t *off)
{
	int bytes_wrote = 0;
	char *msg_ptr = NULL, *msg = NULL;
	lox_data_free(&lox_data);
	msg = kcalloc(len, sizeof(char), GFP_KERNEL);
	msg_ptr = msg;

	while (len) {
		get_user(*(msg_ptr++), buff++);
		len--;
		bytes_wrote++;
	}
	writeRaw(lox_input_buffer, msg, len, off);
	lox_data = kmalloc(sizeof(lox_data_t), GFP_KERNEL);
	lox_data->done = kmalloc(sizeof(struct completion), GFP_KERNEL);
	init_completion(lox_data->done);
	lox_data->program = msg;
	interpreter_thread =
		kthread_run(threadedLox, (void *)lox_data, "lox_thread");
	pr_info("lox_input_buffer->buffer: %s\n", msg);
	return bytes_wrote;
}
module_init(chardev_init);
module_exit(chardev_exit);
