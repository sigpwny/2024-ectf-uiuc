/**
 * @file    wrapper.c
 * @brief   Small wrapper around MSDK driver library for the MAX78000
 */

#include "led.h"
#include "trng.h"

/**
 * @brief Initialize the system.
 * 
 * This function initializes the system peripherals and sets up the board link.
 */
void init_system(void) {
  // TODO: Add any peripheral initialization here
}

// TODO: Add any other wrapper functions here

void led_on(unsigned int idx) {
  LED_On(idx);
}