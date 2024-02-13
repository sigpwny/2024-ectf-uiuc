# @file build_ap.py
# @author Frederich Stine
# @brief Tool for building application processor firmware
# @date 2024
#
# This source file is part of an example system for MITRE's 2024 Embedded CTF (eCTF).
# This code is being provided only for educational purposes for the 2024 MITRE eCTF
# competition, and may not meet MITRE standards for quality. Use this code at your
# own risk!
#
# @copyright Copyright (c) 2024 The MITRE Corporation

from loguru import logger
import argparse
import asyncio
from pathlib import Path
import os

from ectf_tools.utils import run_shell, package_binary, i2c_address_is_blacklisted


def build_ap(
    output_name,
    output_dir: Path
):
    """
    Build an application processor.
    """

    try:
        os.makedirs(output_dir, exist_ok=True)
    except Exception as e:
        print(e)
        raise

    output_dir = os.path.abspath(output_dir)
    output_elf = f"{output_dir}/{output_name}.elf"
    output_bin = f"{output_dir}/{output_name}.bin"
    output_img = f"{output_dir}/{output_name}.img"

    logger.info("Checking output paths")
    if os.path.exists(output_elf):
        logger.info("Removing old .elf output")
        os.remove(output_elf)
    if os.path.exists(output_bin):
        logger.info("Removing old .bin output")
        os.remove(output_bin)
    if os.path.exists(output_img):
        logger.info("Removing old .img output")
        os.remove(output_img)


    if not os.path.exists(output_bin):
        logger.error("Error: tool did not build properly")
        exit(1)

    logger.info("Built application processor")

    logger.info("Packaging binary")
    package_binary(output_bin, output_img)
    logger.info("Binary packaged")

    return


def main():
    parser = argparse.ArgumentParser(
        prog="eCTF Build Application Processor Tool",
        description="Build an Application Processor using Nix"
    )

    parser.add_argument(
        "-on", "--output-name", required=True,
        help=("Output prefix of the built application processor binary \n"
              "Example 'ap' -> ap.bin, ap.elf, ap.img" 
        )
    )

    parser.add_argument(
        "-od", "--output-dir", required=False, type=Path,
        default=Path('.'), help=f"Output name of the directory to store the result: default: %(default)s"
    )

    args = parser.parse_args()

    build_ap(
        args.output_name,
        args.output_dir,
    )
 

if __name__ == '__main__':
    # main()
    package_binary('./max78000-test-debug.bin', './max78000-test-debug.img')
