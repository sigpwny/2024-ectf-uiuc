import argparse
import os
import pathlib
import secrets

file_name = "ectf_global_secrets.rs"

output_dirs = [
    "../application_processor/rust/src",
    "../component/rust/src",
]

def gen_ascon_key():
    bytes = secrets.token_bytes(16)
    return ''.join([f"\\x{byte:02x}" for byte in bytes])

def gen_lib_files():
    lib_file = f"pub const ASCON_SECRET_KEY_AP_TO_C: &[u8] = b\"{gen_ascon_key()}\";\n"
    lib_file += f"pub const ASCON_SECRET_KEY_C_TO_AP: &[u8] = b\"{gen_ascon_key()}\";\n"

    for _dir in output_dirs:
        try:
            file = pathlib.Path(_dir) / file_name
            with open(file, "w") as f:
                f.write(lib_file)
                print(f"Generated {file}")
        except FileNotFoundError:
            print(f"Error: Could not find file {file}")
            exit(1)

def clean_lib_files():
    for _dir in output_dirs:
        try:
            file = pathlib.Path(_dir) / file_name
            os.remove(file)
            print(f"Cleaned {file}")
        except FileNotFoundError:
            pass

def main():
    parser = argparse.ArgumentParser(
        prog="Build Global Secrets Tool",
        description="UIUC tool to generate shared secrets between the Application Processor and the Components"
    )
    parser.add_argument(
        "-c", "--clean", action="store_true", help="Clean the generated secret files"
    )

    args = parser.parse_args()

    if args.clean:
        clean_lib_files()
    else:
        gen_lib_files()

if __name__ == '__main__':
    main()