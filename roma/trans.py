import string

# Mapping of characters for encoding and decoding
ENCODING_CHARS = (
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    "!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
)

def decode_base94(encoded_str):
    base = len(ENCODING_CHARS)
    decoded_value = 0
    for char in encoded_str:
        decoded_value = decoded_value * base + ord(char) - 33
    return decoded_value

def decode_string(encoded_str):
    return ''.join(chr(ENCODING_CHARS.index(char) + 33) for char in encoded_str)

def transpile_icfp_to_python(tokens):
    def transpile(tokens, env=None):
        if env is None:
            env = {}
        if not tokens:
            return None, tokens

        token = tokens.pop(0)
        indicator = token[0]
        body = token[1:]

        if indicator == 'T':
            return 'True', tokens
        elif indicator == 'F':
            return 'False', tokens
        elif indicator == 'I':
            return str(decode_base94(body)), tokens
        elif indicator == 'S':
            return f'"{decode_string(body)}"', tokens
        elif indicator == 'U':
            operator = body
            operand, tokens = transpile(tokens, env)
            if operator == '-':
                return f'(-{operand})', tokens
            elif operator == '!':
                return f'(not {operand})', tokens
            elif operator == '#':
                return f'decode_base94({operand})', tokens
            elif operator == '$':
                return f'encode_string({operand})', tokens
        elif indicator == 'B':
            operator = body
            x, tokens = transpile(tokens, env)
            y, tokens = transpile(tokens, env)
            if operator == '+':
                return f'({x} + {y})', tokens
            elif operator == '-':
                return f'({x} - {y})', tokens
            elif operator == '*':
                return f'({x} * {y})', tokens
            elif operator == '/':
                return f'({x} // {y})', tokens
            elif operator == '%':
                return f'({x} % {y})', tokens
            elif operator == '<':
                return f'({x} < {y})', tokens
            elif operator == '>':
                return f'({x} > {y})', tokens
            elif operator == '=':
                return f'({x} == {y})', tokens
            elif operator == '|':
                return f'({x} or {y})', tokens
            elif operator == '&':
                return f'({x} and {y})', tokens
            elif operator == '.':
                return f'({x} + {y})', tokens
            elif operator == 'T':
                return f'({y}[:{x}])', tokens
            elif operator == 'D':
                return f'({y}[{x}:])', tokens
            elif operator == '$':
                return f'({x}({y}))', tokens
        elif indicator == '?':
            condition, tokens = transpile(tokens, env)
            if_true, tokens = transpile(tokens, env)
            if_false, tokens = transpile(tokens, env)
            return f'({if_true} if {condition} else {if_false})', tokens
        elif indicator == 'L':
            variable_number = decode_base94(body)
            lambda_body, tokens = transpile(tokens, env)
            return f'(lambda v{variable_number}: {lambda_body})', tokens
        elif indicator == 'v':
            variable_number = decode_base94(body)
            return f'v{variable_number}', tokens
        return None, tokens

    transpiled_code, _ = transpile(tokens.split())
    return transpiled_code

def main():
    # Example ICFP string
    icfp_string = "B. SF B$ B$ L\" B$ L\" B$ L# B$ v\" B$ v# v# L# B$ v\" B$ v# v# L$ L# ? B= v# I\" v\" B. v\" B$ v$ B- v# I\" Sl I#"
    icfp_string = """B. SF B$ B$ L" B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L$ L# ? B= v# I" v" B. v" B$ v$ B- v# I" Sl I#,"""

    # Transpile ICFP to Python
    python_code = transpile_icfp_to_python(icfp_string)
    print("Transpiled Python Code:")
    print(python_code)

    # Optionally, execute the generated Python code
    # exec(python_code)

if __name__ == "__main__":
    main()

