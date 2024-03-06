// See the following files for the implementation of these functions:
// - application_processor/rust/src/post_boot.rs
#ifndef POST_BOOT_H
#define POST_BOOT_H
#include <stdint.h>

typedef uint8_t i2c_addr_t;

/* Required POST_BOOT functions */
int secure_send(uint8_t address, uint8_t* buffer, uint8_t len);
int secure_receive(i2c_addr_t address, uint8_t* buffer);
int get_provisioned_ids(uint32_t* buffer);

/* mxc_delay.h */
int MXC_Delay(uint32_t us);

/* led.h */
#define LED1 0
#define LED2 1
#define LED3 2
int LED_Init(void);
void LED_On(unsigned int idx);
void LED_Off(unsigned int idx);
void LED_Toggle(unsigned int idx);

#endif
