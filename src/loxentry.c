#include "vm.h"
#include "loxentry.h"
#include "kmain.h"
#include "linux/completion.h"
#include "linux/slab.h"
static void runString(const char *source)
{
	InterpretResult result = interpret(source);
	if (result == INTERPRET_COMPILE_ERROR)
		printk(KERN_ALERT "Compile error\n");
	if (result == INTERPRET_RUNTIME_ERROR)
		printk(KERN_ALERT "Runtime error\n");
}
static int loxStart(char *program)
{
	initVM();
	runString(program);
	freeVM();
	return 0;
}
int threadedLox(void *data)
{
	int retval = 0;
	struct lox_data_t *lox_data = (struct lox_data_t *)data;
	pr_alert("Threaded lox started\n");
	pr_alert("Program: %s\n", lox_data->program);
	retval = loxStart(lox_data->program);
	complete(lox_data->done);
	kfree(lox_data->program);
	return 0;
}
bool lox_data_free(lox_data_t **lox_data)
{
	if (*lox_data != NULL) {
		wait_for_completion((*lox_data)->done);
		kfree((*lox_data)->done);
		kfree(*lox_data);
		*lox_data = NULL;
		return true;
	}
	return false;
}
