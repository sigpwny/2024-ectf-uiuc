import argparse
import os
import sys

from ectf_tools.utils import package_binary


def main():
    parser = argparse.ArgumentParser(
        prog="UIUC Debug Image Packaging Tool",
        description="Used to build a flash image file from a binary file to skip the full build process"
    )
    parser.add_argument(
        "-b", "--bin", required=True,
        help=("Input .bin file")
    )
    parser.add_argument(
        "-i", "--img",
        help=f"Output .img file"
    )

    args = parser.parse_args()

    if os.path.exists(args.bin):
        package_binary(
            args.bin,
            args.img,
        )
    else:
        print(f"Error: {args.bin} does not exist", file=sys.stderr)
        exit(1)
 

if __name__ == '__main__':
    main()