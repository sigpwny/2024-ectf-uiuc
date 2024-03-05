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

#define LEN_POISON 16
#define LEN_SEND_BUFFER 16
#define LEN_RECV_BUFFER 16

        // Testing for buffer overruns
        char poisonA[LEN_POISON] = "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55";
        char send_buf[] = "Hello from C!\r\n\0";
        char poisonB[LEN_POISON] = "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55";
        char recv_buf[] = "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        char poisonC[LEN_POISON] = "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55";
        uint8_t *send_buf_ptr = send_buf;
        uint8_t *recv_buf_ptr = recv_buf;
        // Receive a message from the AP
        int result_recv = secure_receive(recv_buf_ptr);
        if (result_recv < 0) {
            printf("Failed to receive message from the AP!\n");
        } else {
            printf("Received message of %d bytes from the AP: \n", result_recv);
            for (int i = 0; i < result_recv; i++) {
                printf("%c", recv_buf[i]);
            }
        }
        // Send a message to the AP
        secure_send(send_buf_ptr, LEN_SEND_BUFFER);
        // Check that poison buffers are still intact
        for (int i = 0; i < LEN_POISON; i++) {
            if (poisonA[i] != 0x55) {
                printf("PoisonA buffer has been modified!\n");
            }
            if (poisonB[i] != 0x55) {
                printf("PoisonB buffer has been modified!\n");
            }
            if (poisonC[i] != 0x55) {
                printf("PoisonC buffer has been modified!\n");
            }
        }
    }
#endif
}