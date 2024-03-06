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
        printf("Hello from AP POST_BOOT! %d\n", number);
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

#define LEN_POISON 16
#define LEN_SEND_BUFFER 16
#define LEN_RECV_BUFFER 16
#define LEN_PROVISIONED_IDS 1

        // Testing for buffer overruns
        char poisonA[LEN_POISON] = "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55";
        char send_buf[] = "Hello from AP\r\n\0";
        char poisonB[LEN_POISON] = "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55";
        char recv_buf[] = "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        char poisonC[LEN_POISON] = "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55";
        uint32_t provisioned_ids[] = {0, 0};
        char poisonD[LEN_POISON] = "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55";
        uint8_t *send_buf_ptr = send_buf;
        uint8_t *recv_buf_ptr = recv_buf;
        uint32_t *provisioned_ids_ptr = provisioned_ids;
        // Send a message to each provisioned ID
        int comp_count = get_provisioned_ids(provisioned_ids_ptr);
        for (int i = 0; i < comp_count; i++) {
            uint8_t addr = (uint8_t)provisioned_ids[i];
            int result_send = secure_send(addr, send_buf_ptr, LEN_SEND_BUFFER);
            if (result_send != 0) {
                printf("Failed to send message to provisioned ID %d\r\n", addr);
            }
        }
        // Receive a message from the first provisioned ID
        int result_recv = secure_receive((i2c_addr_t)provisioned_ids[0], recv_buf_ptr);
        if (result_recv < 0) {
            printf("Failed to receive message from provisioned ID %d\r\n", (i2c_addr_t)provisioned_ids[0]);
        } else {
            printf("Received message of %d bytes from provisioned ID %d: ", result_recv, (i2c_addr_t)provisioned_ids[0]);
            for (int i = 0; i < result_recv; i++) {
                printf("%c", recv_buf[i]);
            }
        }
        // Check that poison buffers are still intact
        for (int i = 0; i < LEN_POISON; i++) {
            if (poisonA[i] != 0x55) {
                printf("PoisonA buffer has been modified!\r\n");
            }
            if (poisonB[i] != 0x55) {
                printf("PoisonB buffer has been modified!\r\n");
            }
            if (poisonC[i] != 0x55) {
                printf("PoisonC buffer has been modified!\r\n");
            }
            if (poisonD[i] != 0x55) {
                printf("PoisonD buffer has been modified!\r\n");
            }
        }
    }
#endif
}