// See the following files for the implementation of these functions:
// - component/rust/src/post_boot.rs
#ifndef POST_BOOT_H
#define POST_BOOT_H
#include <stdint.h>

/* Required POST_BOOT functions */
void secure_send(uint8_t* buffer, uint8_t len);
int secure_receive(uint8_t* buffer);

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
