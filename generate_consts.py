import constants
import re
import os


def xor_with_byte(message: str, key: int) -> bytes:
    return bytes([b ^ key for b in message.encode("utf-8")])


def encode_base128(data: bytes) -> list[int]:
    output = []
    buffer = 0
    bits_in_buffer = 0

    for byte in data:
        buffer = (buffer << 8) | byte
        bits_in_buffer += 8
        while bits_in_buffer >= 7:
            bits_in_buffer -= 7
            char_code = (buffer >> bits_in_buffer) & 0x7F
            output.append(char_code)

    if bits_in_buffer > 0:
        char_code = (buffer << (7 - bits_in_buffer)) & 0x7F
        output.append(char_code)

    return output


def generate_rust_constants(output_path):
    consts = []

    for key in dir(constants):
        if not key.startswith("__"):
            print(f"Fixing {key}")
            value = getattr(constants, key)

            if key.startswith("decoy_"):
                consts.append(f"// DECOY\n#[used]\n#[unsafe(no_mangle)]")
                consts.append(
                    f'pub static {key.removeprefix("decoy_")}: &str = "{value}";'
                )
                continue

            xored = xor_with_byte(value, 190)
            encoded_bytes = encode_base128(xored)

            byte_str = format_byte_block(encoded_bytes)
            consts.append(f"// {value}")
            consts.append(
                f"pub const {key}: [u8; {len(encoded_bytes)}] = [{byte_str}];\n"
            )

    with open(output_path, "w") as f:
        f.write("/*\n *   Generated file - do not edit\n */\n\n")
        f.write("\n".join(consts))
        f.write("\n")


def format_byte_block(bytes: list[int]) -> str:
    count = 0
    buff = []

    if len(bytes) < 11:
        return ", ".join(f"0x{b:02x}" for b in bytes)

    buff.append("\n   ")
    for b in bytes:
        if count == 16:
            count = 0
            buff.append("\n   ")
        count += 1

        buff.append(f" 0x{b:02x},")
    buff.append("\n")

    return "".join(buff)


if __name__ == "__main__":
    generate_rust_constants("./src/constants.gen.rs")
