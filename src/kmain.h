#ifndef KMAIN
#define KMAIN

#define DEVICE_NAME "LoxIO"
enum {
	CDEV_NOT_USED = 0,
	CDEV_EXCLUSIVE_OPEN = 1,
};
typedef struct lox_data_t {
	struct completion *done;
	char *program;
} lox_data_t;
#endif
