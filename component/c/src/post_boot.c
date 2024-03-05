#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "post_boot.h"
// #include "led.h"
// #include "mxc_delay.h"


/**
 * @brief Enter post boot functionality
 * 
 * This function is called from the Rust code after boot verification passes.
 */
void post_boot() {
#ifdef POST_BOOT
    POST_BOOT
#else
    int number = 0;
    while (1) {
        printf("Hello from Component POST_BOOT! %d\n", number);
        fflush(stdout);
        number++;
        LED_On(LED1);
        MXC_Delay(500000);
        LED_On(LED2);
        MXC_Delay(500000);
        LED_On(LED3);
        MXC_Delay(500000);
        LED_Off(LED1);
        MXC_Delay(500000);
        LED_Off(LED2);
        MXC_Delay(500000);
        LED_Off(LED3);
        MXC_Delay(500000);
    }
#endif
}