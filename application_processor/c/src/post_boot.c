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
 * @param address: i2c_addr_t, I2C address of recipient
 * @param buffer: uint8_t*, pointer to data to be send
 * @param len: uint8_t, size of data to be sent 
 * 
 * Securely send data over I2C. This function is utilized in POST_BOOT functionality.
 * This function must be implemented by your team to align with the security requirements.

*/
int secure_send(uint8_t address, uint8_t* buffer, uint8_t len) {
    // return send_packet(address, len, buffer);
    return -1;
}

/**
 * @brief Secure Receive
 * 
 * @param address: i2c_addr_t, I2C address of sender
 * @param buffer: uint8_t*, pointer to buffer to receive data to
 * 
 * @return int: number of bytes received, negative if error
 * 
 * Securely receive data over I2C. This function is utilized in POST_BOOT functionality.
 * This function must be implemented by your team to align with the security requirements.
*/
int secure_receive(i2c_addr_t address, uint8_t* buffer) {
    // return poll_and_receive_packet(address, buffer);
    return -1;
}

/**
 * @brief Get Provisioned IDs
 * 
 * @param uint32_t* buffer
 * 
 * @return int: number of ids
 * 
 * Return the currently provisioned IDs and the number of provisioned IDs
 * for the current AP. This functionality is utilized in POST_BOOT functionality.
 * This function must be implemented by your team.
*/
int get_provisioned_ids(uint32_t* buffer) {
    // memcpy(buffer, flash_status.component_ids, flash_status.component_cnt * sizeof(uint32_t));
    // return flash_status.component_cnt;
    return 0;
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
    uint32_t number = 0;
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