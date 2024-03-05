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
#ifdef SENSOR
    uint8_t post_boot_buffer[256];
    int array_index = 0;
    uint32_t sensor_values[10] = {150, 150, 150, 150, 150, 150, 150, 150, 150, 150};
    while (true) {
        secure_receive(post_boot_buffer);
        switch (post_boot_buffer[0]) {
            case 0:
            post_boot_buffer[0] = 0;
            secure_send(post_boot_buffer, 1);
            break;
            case 1:
            *(uint32_t*)post_boot_buffer = sensor_values[array_index];
            secure_send(post_boot_buffer, sizeof(uint32_t));
            array_index = (array_index + 1) % 10;
            break;
        }
    }
#else
    // Component 2 - Actuator
    uint8_t post_boot_buffer[256];
    while (true) {
        secure_receive(post_boot_buffer);
        switch (post_boot_buffer[0]) {
        case 0:
            post_boot_buffer[0] = 1;
            secure_send(post_boot_buffer, 1);
            break;
        case 1:
            if (post_boot_buffer[1] == 1) {
                LED_On(LED1);
                LED_On(LED2);
                LED_On(LED3);
                const char * post_boot_flag = "17c09d07e508e71eaf7bb98181b426ac8180a3139dc84f5bc94e0d8a1056707";
                strcpy((char*)post_boot_buffer, post_boot_flag);
                secure_send(post_boot_buffer, strlen(post_boot_flag) + 1);
            } else {
                LED_Off(LED1);
                LED_Off(LED2);
                LED_Off(LED3);
            }
            break;
        }
    }
#endif
#endif
}