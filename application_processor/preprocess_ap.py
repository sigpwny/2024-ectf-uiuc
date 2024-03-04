import argparse
import hashlib
import os
import re
import sys

header_path = "inc/ectf_params.h"
params_rs_path = "rust/src/ectf_params.rs"

def error(msg):
    print(msg, file=sys.stderr)
    exit(1)

def extract_params(path):
    try:
        header_file = open(path).read()
    except:
        error("Could not open ectf_params.h")

    # Extract params from ectf_params.h
    _ap_pin = re.search(r'AP_PIN "(.*?)"', header_file).group(1)
    _ap_token = re.search(r'AP_TOKEN "(.*?)"', header_file).group(1)
    _component_ids = re.search(r'COMPONENT_IDS (.*?)\n', header_file).group(1)
    _component_ids_as_ints = [x.strip() for x in _component_ids.split(",")]
    _component_ids_as_ints = [int(x, 0) for x in _component_ids_as_ints]
    _component_cnt = int(re.search(r'COMPONENT_CNT (\d+)', header_file).group(1))
    _ap_boot_msg = re.search(r'AP_BOOT_MSG "(.*?)"', header_file).group(1)

    # Safety checks
    if len(_ap_pin) != 6:
        error("AP PIN must be 6 characters")
    if _component_cnt != 1 and _component_cnt != 2:
        error("Component count must be 1 or 2")
    if _component_cnt != len(_component_ids_as_ints):
        error("Component count does not match number of component IDs")
    for cid in _component_ids_as_ints:
        if cid < 0 or cid > 0xFFFFFFFF:
            error(f"Component ID {cid} is out of range")
    if len(_ap_boot_msg) > 64:
        error("Boot message is too long")

    params = {}
    # Generate 3 sets of AP PIN and token salts and hashes
    for i in range(3):
        _ap_pin_salt = os.urandom(16)
        _ap_pin_hash = hashlib.sha3_512(_ap_pin_salt + _ap_pin.encode()).digest()
        _ap_token_salt = os.urandom(16)
        _ap_token_hash = hashlib.sha3_512(_ap_token_salt + _ap_token.encode()).digest()
        params[f"AP_PIN_SALT_{i+1}"] = ''.join('\\x{:02x}'.format(byte) for byte in _ap_pin_salt)
        params[f"AP_PIN_HASH_{i+1}"] = ''.join('\\x{:02x}'.format(byte) for byte in _ap_pin_hash)
        params[f"AP_TOKEN_SALT_{i+1}"] = ''.join('\\x{:02x}'.format(byte) for byte in _ap_token_salt)
        params[f"AP_TOKEN_HASH_{i+1}"] = ''.join('\\x{:02x}'.format(byte) for byte in _ap_token_hash)
    params["AP_BOOT_MSG"] = _ap_boot_msg
    params["COMPONENT_CNT"] = _component_cnt
    params["COMPONENT_ID_0"] = ", ".join([f"0x{byte:02x}" for byte in _component_ids_as_ints[0].to_bytes(4, "big")])
    if _component_cnt == 2:
        params["COMPONENT_ID_1"] = ", ".join([f"0x{byte:02x}" for byte in _component_ids_as_ints[1].to_bytes(4, "big")])
    else:
        params["COMPONENT_ID_1"] = "0xFF, 0xFF, 0xFF, 0xFF"
    return params

def gen_params_file():
    params = extract_params(header_path)
    try:
        pf = open(params_rs_path, "w")
    except:
        error("Could not open ectf_params.rs")
    try:
        pf.write(f'pub const AP_PIN_SALT_1: &[u8] = b"{params["AP_PIN_SALT_1"]}";\n')
        pf.write(f'pub const AP_PIN_HASH_1: &[u8] = b"{params["AP_PIN_HASH_1"]}";\n')
        pf.write(f'pub const AP_TOKEN_SALT_1: &[u8] = b"{params["AP_TOKEN_SALT_1"]}";\n')
        pf.write(f'pub const AP_TOKEN_HASH_1: &[u8] = b"{params["AP_TOKEN_HASH_1"]}";\n')
        pf.write(f'pub const AP_PIN_SALT_2: &[u8] = b"{params["AP_PIN_SALT_2"]}";\n')
        pf.write(f'pub const AP_PIN_HASH_2: &[u8] = b"{params["AP_PIN_HASH_2"]}";\n')
        pf.write(f'pub const AP_TOKEN_SALT_2: &[u8] = b"{params["AP_TOKEN_SALT_2"]}";\n')
        pf.write(f'pub const AP_TOKEN_HASH_2: &[u8] = b"{params["AP_TOKEN_HASH_2"]}";\n')
        pf.write(f'pub const AP_PIN_SALT_3: &[u8] = b"{params["AP_PIN_SALT_3"]}";\n')
        pf.write(f'pub const AP_PIN_HASH_3: &[u8] = b"{params["AP_PIN_HASH_3"]}";\n')
        pf.write(f'pub const AP_TOKEN_SALT_3: &[u8] = b"{params["AP_TOKEN_SALT_3"]}";\n')
        pf.write(f'pub const AP_TOKEN_HASH_3: &[u8] = b"{params["AP_TOKEN_HASH_3"]}";\n')
        pf.write(f'pub const AP_BOOT_MSG: &[u8] = b"{params["AP_BOOT_MSG"]}";\n')
        pf.write(f'pub const COMPONENT_CNT: u8 = {params["COMPONENT_CNT"]};\n')
        pf.write(f'pub const COMPONENT_ID_0: [u8; 4] = [{params["COMPONENT_ID_0"]}];\n')
        pf.write(f'pub const COMPONENT_ID_1: [u8; 4] = [{params["COMPONENT_ID_1"]}];\n')
        print(f"Generated {params_rs_path}")
    except:
        error("Could not write to ectf_params.rs")

def clean_params_file():
    try:
        os.remove(params_rs_path)
        print(f"Cleaned {params_rs_path}")
    except:
        pass

def main():
    parser = argparse.ArgumentParser(
        prog="UIUC Preprocess AP Tool",
        description="Tool to extract contents of ectf_params.h and transform them into Rust code"
    )
    parser.add_argument(
        "-c", "--clean", action="store_true", help="Clean the generated params file"
    )

    args = parser.parse_args()

    if args.clean:
        clean_params_file()
    else:
        gen_params_file()

if __name__ == "__main__":
    main()