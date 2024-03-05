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
    // Application Processor
    uint8_t post_boot_buffer[256];
    printf("Post boot: Insulin Pump started!\n ");
    uint32_t post_boot_component_ids[10];
    int post_boot_component_cnt;
    post_boot_component_cnt = get_provisioned_ids(post_boot_component_ids);
    uint8_t post_boot_sensor_addr = 0;
    uint8_t post_boot_actuator_addr = 0;
    for (int i = 0; i < post_boot_component_cnt; i++) {
        uint8_t addr = (uint8_t)(post_boot_component_ids[i] & 0xFF);
        printf("Found address: %d\n", addr);
        post_boot_buffer[0] = 0;
        secure_send(addr, post_boot_buffer, 1);
        secure_receive(addr, post_boot_buffer);
        switch (post_boot_buffer[0]) {
            case 0:
                printf("Found address for sensor: %d\n", addr);
                post_boot_sensor_addr = addr;
                break;
            case 1:
                printf("Found address for actuator: %d\n", addr);
                post_boot_actuator_addr = addr;
                break;
        }
    }
    uint32_t sensor_values[5] = {0, 0, 0, 0, 0};
    int array_index = 0;
    while (true) {
        post_boot_buffer[0] = 1;
        secure_send(post_boot_sensor_addr, post_boot_buffer, 1);
        secure_receive(post_boot_sensor_addr, post_boot_buffer);
        sensor_values[array_index] = *(uint32_t*)post_boot_buffer;
        array_index = (array_index + 1) % 5;
        uint32_t sensor_sum = 0;
        for (int i = 0; i < 5; i++) {
            sensor_sum += sensor_values[i];
        }
        float sensor_avg = ((float)sensor_sum) / 5.0;
        if (sensor_avg > 128.0) {
            post_boot_buffer[0] = 1;
            post_boot_buffer[1] = 1;
            secure_send(post_boot_actuator_addr, post_boot_buffer, 2);
            secure_receive(post_boot_actuator_addr, post_boot_buffer);
            printf("%%success: %s\n%%", post_boot_buffer);
        } else {
            post_boot_buffer[0] = 1;
            post_boot_buffer[1] = 0;
            secure_send(post_boot_actuator_addr, post_boot_buffer, 2);
        }
        MXC_Delay(500000);
    }
#endif
}