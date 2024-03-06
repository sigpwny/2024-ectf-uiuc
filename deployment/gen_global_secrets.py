import argparse
import os
import secrets
import sys

output_file = "../rust-lib/ectf-board/src/ectf_global_secrets.rs"

def error(msg):
    print(msg, file=sys.stderr)
    exit(1)

def to_hex_str(bytes):
    return ''.join('\\x{:02x}'.format(byte) for byte in bytes)

def gen_ascon_key():
    bytes = secrets.token_bytes(16)
    return to_hex_str(bytes)

def gen_secrets_files():
    secrets_file = (
f"""use crate::secure_comms::Ascon128Keys;

pub const ASCON_SECRET_KEYS: Ascon128Keys = Ascon128Keys {{
    ap_to_c: *b"{gen_ascon_key()}",
    c_to_ap: *b"{gen_ascon_key()}",
}};
"""
    )

    try:
        with open(output_file, "w") as f:
            f.write(secrets_file)
            print(f"Generated {output_file}")
    except FileNotFoundError:
        error(f"Could not create file {output_file}")

def clean_secrets_files():
    try:
        os.remove(output_file)
        print(f"Cleaned {output_file}")
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