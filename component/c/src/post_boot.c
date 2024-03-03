// #include "mxc_delay.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>

typedef uint8_t i2c_addr_t;
// uint32_t SystemCoreClock __attribute__((section(".shared")));
// volatile uint32_t mailbox __attribute__((section(".mailbox")));

#ifndef POST_BOOT
// #include "led.h"
#endif

/******************************* POST BOOT FUNCTIONALITY *********************************/
/**
 * @brief Secure Send 
 * 
 * @param buffer: uint8_t*, pointer to data to be send
 * @param len: uint8_t, size of data to be sent 
 * 
 * Securely send data over I2C. This function is utilized in POST_BOOT functionality.
 * This function must be implemented by your team to align with the security requirements.
*/
void secure_send(uint8_t* buffer, uint8_t len) {
    // send_packet_and_ack(len, buffer); 
}

/**
 * @brief Secure Receive
 * 
 * @param buffer: uint8_t*, pointer to buffer to receive data to
 * 
 * @return int: number of bytes received, negative if error
 * 
 * Securely receive data over I2C. This function is utilized in POST_BOOT functionality.
 * This function must be implemented by your team to align with the security requirements.
*/
int secure_receive(uint8_t* buffer) {
    // return wait_and_receive_packet(buffer);
    return -1;
}

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
        printf("Hello from POST_BOOT!\n%d", number);
        fflush(stdout);
        number++;
        // LED_On(LED1);
        // MXC_Delay(500000);
        // LED_On(LED2);
        // MXC_Delay(500000);
        // LED_On(LED3);
        // MXC_Delay(500000);
        // LED_Off(LED1);
        // MXC_Delay(500000);
        // LED_Off(LED2);
        // MXC_Delay(500000);
        // LED_Off(LED3);
        // MXC_Delay(500000);
    }
#endif
}