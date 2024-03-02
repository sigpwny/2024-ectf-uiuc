import argparse
import os
import pathlib
import secrets
import sys

file_name = "ectf_global_secrets.rs"

output_dirs = [
    "../application_processor/rust/src",
    "../component/rust/src",
]

def error(msg):
    print(msg, file=sys.stderr)
    exit(1)

def gen_ascon_key():
    bytes = secrets.token_bytes(16)
    return ''.join([f"\\x{byte:02x}" for byte in bytes])

def gen_secrets_files():
    secrets_file = f"pub const ASCON_SECRET_KEY_AP_TO_C: &[u8] = b\"{gen_ascon_key()}\";\n"
    secrets_file += f"pub const ASCON_SECRET_KEY_C_TO_AP: &[u8] = b\"{gen_ascon_key()}\";\n"

    for _dir in output_dirs:
        try:
            file = pathlib.Path(_dir) / file_name
            with open(file, "w") as f:
                f.write(secrets_file)
                print(f"Generated {file}")
        except FileNotFoundError:
            error(f"Could not create file {file}")

def clean_secrets_files():
    for _dir in output_dirs:
        try:
            file = pathlib.Path(_dir) / file_name
            os.remove(file)
            print(f"Cleaned {file}")
        except FileNotFoundError:
            pass

def main():
    parser = argparse.ArgumentParser(
        prog="UIUC Build Global Secrets Tool",
        description="Tool to generate shared secrets between the Application Processor and the Components"
    )
    parser.add_argument(
        "-c", "--clean", action="store_true", help="Clean the generated secret files"
    )

    args = parser.parse_args()

    if args.clean:
        clean_secrets_files()
    else:
        gen_secrets_files()

if __name__ == '__main__':
    main()