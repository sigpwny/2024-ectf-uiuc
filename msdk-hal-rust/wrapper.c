/**
 * @file    wrapper.c
 * @brief   Small wrapper around MSDK driver library for the MAX78000
 */

#include "led.h"
#include "trng.h"
#include "uart.h"



/**
 * @brief Initialize the system.
 * 
 * This function initializes the system peripherals and sets up the board link.
 */
void init_system(void) {
  // TODO: Add any peripheral initialization here
  uart_init();
}
// baudrate=115200,
// parity=serial.PARITY_NONE,
// stopbits=serial.STOPBITS_ONE,
// bytesize=serial.EIGHTBITS,
// TODO: Add any other wrapper functions here
static void uart_init(void){

  MXC_UART_Init(MXC_UART0, 115200, MXC_UART_APB_CLK); //dont know the clock here
  MXC_UART_SetParity(MXC_UART0, MXC_UART_PARITY_DISABLE);
  MXC_UART_SetStopBits(MXC_UART0, MXC_UART_STOP_1);

}

//UART SEND POINTER AND SIZE
unsigned int UART_write_FIFO(const unsigned char *bytes, unsigned int len){

  if (MXC_UART_GetTXFIFOAvailable(MXC_UART0) < len){ //Get the amount of free space available in the transmit FIFO.
    return -1;
  }
  return MXC_UART_WriteTXFIFO(MXC_UART0, *bytes, len); //return num bytes written

} 
//UART RECIEVE LENGTH DATA
unsigned int UART_read_FIFO(unsigned char *bytes, unsigned int len){
  
  if (MXC_UART_GetRXFIFOAvailable(MXC_UART0) == FULL){ //Get the number of bytes currently available in the receive FIFO.
    return -1;
  }
  return MXC_UART_ReadRXFIFO(MXC_UART0, *bytes, len); //return num bytes read

}

 



