/**
 * @file    wrapper.c
 * @brief   Small wrapper around MSDK driver library for the MAX78000
 */

#include "led.h"
#include "trng.h"
#include "uart.h"

mxc_uart_regs_t* UART ; //defined UART

/**
 * @brief Initialize the system.
 * 
 * This function initializes the system peripherals and sets up the board link.
 */
void init_system(void) {
  // TODO: Add any peripheral initialization here
}
// baudrate=115200,
// parity=serial.PARITY_NONE,
// stopbits=serial.STOPBITS_ONE,
// bytesize=serial.EIGHTBITS,
// TODO: Add any other wrapper functions here
static void uart_init(void){
  MXC_UART_Init(UART, 115200, mxc_uart_clock_t clock); //dont know the clock here
  MXC_UART_SetParity(UART, MXC_UART_PARITY_DISABLE);
  MXC_UART_SetStopBits(UART, MXC_UART_STOP_1);
}