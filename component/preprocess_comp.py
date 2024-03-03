import argparse
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
    _component_id = re.search(r'COMPONENT_ID (.*?)\n', header_file).group(1)
    _component_id = int(_component_id, 0)
    _component_boot_msg = re.search(r'COMPONENT_BOOT_MSG "(.*?)"', header_file).group(1)
    _attestation_loc = re.search(r'ATTESTATION_LOC "(.*?)"', header_file).group(1)
    _attestation_date = re.search(r'ATTESTATION_DATE "(.*?)"', header_file).group(1)
    _attestation_customer = re.search(r'ATTESTATION_CUSTOMER "(.*?)"', header_file).group(1)

    # Safety checks
    if _component_id < 0 or _component_id > 0xFFFFFFFF:
        error(f"Component ID {_component_id} is out of range")
    if len(_component_boot_msg) > 64:
        error("Boot message is too long")
    if len(_attestation_loc) > 64:
        error("Attestation location is too long")
    if len(_attestation_date) > 64:
        error("Attestation date is too long")
    if len(_attestation_customer) > 64:
        error("Attestation customer is too long")

    params = {}
    _component_id = ", ".join([f"0x{byte:02x}" for byte in _component_id.to_bytes(4, "big")])
    params["COMPONENT_ID"] = _component_id
    params["COMPONENT_BOOT_MSG"] = _component_boot_msg
    params["ATTESTATION_LOCATION"] = _attestation_loc
    params["ATTESTATION_DATE"] = _attestation_date
    params["ATTESTATION_CUSTOMER"] = _attestation_customer
    return params

def gen_params_file():
    params = extract_params(header_path)
    try:
        pf = open(params_rs_path, "w")
    except:
        error("Could not open ectf_params.rs")
    try:
        pf.write(f"pub const COMPONENT_ID: [u8; 4] = [{params['COMPONENT_ID']}];\n")
        pf.write(f'pub const COMPONENT_BOOT_MSG: &[u8] = b"{params["COMPONENT_BOOT_MSG"]}";\n')
        pf.write(f'pub const ATTESTATION_LOCATION: &[u8] = b"{params["ATTESTATION_LOCATION"]}";\n')
        pf.write(f'pub const ATTESTATION_DATE: &[u8] = b"{params["ATTESTATION_DATE"]}";\n')
        pf.write(f'pub const ATTESTATION_CUSTOMER: &[u8] = b"{params["ATTESTATION_CUSTOMER"]}";\n')
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
        prog="UIUC Preprocess Component Tool",
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