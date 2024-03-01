/******************************************************************************
 * Copyright (C) 2023 Maxim Integrated Products, Inc., All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL MAXIM INTEGRATED BE LIABLE FOR ANY CLAIM, DAMAGES
 * OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Except as contained in this notice, the name of Maxim Integrated
 * Products, Inc. shall not be used except as stated in the Maxim Integrated
 * Products, Inc. Branding Policy.
 *
 * The mere transfer of this software does not imply any licenses
 * of trade secrets, proprietary technology, copyrights, patents,
 * trademarks, maskwork rights, or any other form of intellectual
 * property whatsoever. Maxim Integrated Products, Inc. retains all
 * ownership rights.
 *
 ******************************************************************************/

// #if defined(__ICCARM__) || defined(__CC_ARM)
#include "fixes.h"
#include <errno.h>
// #else
// #include <sys/errno.h>
// #endif
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/stat.h>


// #include "mxc_device.h"
// #include "mxc_sys.h"
// #include "board.h"
// #include "uart.h"

#define UART0               ((uint32_t)0x40042000UL)
#define UART0_STATUS        (uint32_t *)(UART0 + 0x0004)
#define UART0_FIFO          (uint32_t *)(UART0 + 0x0020)

#define MXC_F_UART_REVB_STATUS_RX_EM    (1 << 4)
#define MXC_F_UART_REVB_STATUS_TX_FULL  (1 << 7)

int UART_ReadCharacter()
{
    volatile uint32_t *uart_status = UART0_STATUS;
    volatile uint32_t *uart_fifo = UART0_FIFO;

    while (*uart_status & MXC_F_UART_REVB_STATUS_RX_EM) {
        // return E_UNDERFLOW;
    }

    return *uart_fifo;
}

void UART_WriteCharacter(char byte)
{
    volatile uint32_t *uart_status = UART0_STATUS;
    volatile uint32_t *uart_fifo = UART0_FIFO;
    // Require the TX FIFO to be empty, so that we write out the expected character
    // Return error if the FIFO is full
    while (*uart_status & MXC_F_UART_REVB_STATUS_TX_FULL) {
        // return E_OVERFLOW;
    }

    *uart_fifo = byte;
}

// #include "halffi.h"

/* The following libc stub functions are required for a proper link with printf().
 * These can be tailored for a complete stdio implementation.
 * GNUC requires all functions below. IAR & KEIL only use read and write.
 */
int _open(const char *name, int flags, int mode)
{
    return -1;
}
int _close(int file)
{
    return -1;
}
int _isatty(int file)
{
    return -1;
}
int _lseek(int file, off_t offset, int whence)
{
    return -1;
}
int _fstat(int file, struct stat *st)
{
    return -1;
}

/* Handle IAR and ARM/Keil Compilers for _read/_write. Keil uses fputc and
   fgetc for stdio */
// GNUC _read function prototype
int _read(int file, char *ptr, int len)
{
    int n;

    int num = 0; // count of number received.

    switch (file) {
    case STDIN_FILENO:
        for (n = 0; n < len; n++) {
            *ptr = UART_ReadCharacter(); // read a byte.
            UART_WriteCharacter(*ptr); // echo the byte.
            // *ptr = ffi_uart_read_byte(); // read a byte.
            // ffi_uart_write_byte(*ptr); // echo the byte.

            if (*ptr == '\r') { // check for end of line.
                *ptr = '\n';
                num++;
                ptr++;

                break;
            } else {
                ptr++;
                num++;
            }
        }

        break;

    default:
        errno = EBADF;
        return -1;
    }

    return num;
}

/* newlib/libc printf() will eventually call write() to get the data to the stdout */
// GNUC _write function prototype
int _write(int file, char *ptr, int len)
{
    int n;

    switch (file) {
    case STDOUT_FILENO:
    case STDERR_FILENO:

        // This function should be as fast as possible
        // So we'll forgo the UART driver for now
        for (n = 0; n < len; n++) {
            if (*ptr == '\n') {
                // Wait until there's room in the FIFO
                // while (MXC_UART_GetTXFIFOAvailable(MXC_UARTn) == 0) {}

                UART_WriteCharacter('\r');
                // ffi_uart_write_byte('\r');
            }

            // Wait until there's room in the FIFO
            // while (MXC_UART_GetTXFIFOAvailable(MXC_UARTn) == 0) {}

            UART_WriteCharacter(*ptr++);
            // ffi_uart_write_byte(*ptr++);
        }

        break;

    default:
        errno = EBADF;
        return -1;
    }

    return len;
}

