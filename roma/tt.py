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
        decoded_value = decoded_value * base + ENCODING_CHARS.index(char)
    return decoded_value

def encode_base94(value):
    if value == 0:
        return ENCODING_CHARS[0]
    base = len(ENCODING_CHARS)
    encoded_str = ""
    while value > 0:
        encoded_str = ENCODING_CHARS[value % base] + encoded_str
        value //= base
    return encoded_str

def decode_string(encoded_str):
    return ''.join(chr(ENCODING_CHARS.index(char) + 33) for char in encoded_str)

def encode_string(value):
    return ''.join(ENCODING_CHARS[ord(char) - 33] for char in value)

def evaluate(tokens, env=None):
    if env is None:
        env = {}
    if not tokens:
        return None

    token = tokens.pop(0)
    indicator = token[0]
    body = token[1:]

    print(f"Evaluating token: {token}, Indicator: {indicator}, Body: {body}, Env: {env}")

    if indicator == 'T':
        return True
    elif indicator == 'F':
        return False
    elif indicator == 'I':
        return decode_base94(body)
    elif indicator == 'S':
        return decode_string(body)
    elif indicator == 'U':
        operator = body
        operand = evaluate(tokens, env)
        if operator == '-':
            return -operand
        elif operator == '!':
            return not operand
        elif operator == '#':
            return decode_base94(operand)
        elif operator == '$':
            return encode_string(operand)
    elif indicator == 'B':
        operator = body
        x = evaluate(tokens, env)
        y = evaluate(tokens, env)
        if operator == '+':
            return x + y
        elif operator == '-':
            return x - y
        elif operator == '*':
            return x * y
        elif operator == '/':
            return x // y
        elif operator == '%':
            return x % y
        elif operator == '<':
            return x < y
        elif operator == '>':
            return x > y
        elif operator == '=':
            return x == y
        elif operator == '|':
            return x or y
        elif operator == '&':
            return x and y
        elif operator == '.':
            return x + y
        elif operator == 'T':
            return y[:x]
        elif operator == 'D':
            return y[x:]
        elif operator == '$':
            if callable(x):
                return x(y)
            else:
                raise TypeError(f"Expected a callable for application, got {x}")
    elif indicator == '?':
        condition = evaluate(tokens, env)
        if_true = evaluate(tokens, env)
        if_false = evaluate(tokens, env)
        return if_true if condition else if_false
    elif indicator == 'L':
        variable_number = decode_base94(body)
        lambda_body = tokens.copy()
        current_env = env.copy()
        res = lambda arg: evaluate(lambda_body.copy(), {**current_env, variable_number: arg})
        print("lambda parse ok")
        return res
    elif indicator == 'v':
        variable_number = decode_base94(body)
        if variable_number in env:
            return env[variable_number]
        else:
            raise ValueError(f"Variable {variable_number} not found in environment")
    return None

def encode_boolean(value):
    return 'T' if value else 'F'

def encode_integer(value):
    return 'I' + encode_base94(value)

def encode_icfp(value):
    if isinstance(value, bool):
        return encode_boolean(value)
    elif isinstance(value, int):
        return encode_integer(value)
    elif isinstance(value, str):
        return 'S' + encode_string(value)
    return None

def main():
    # Evaluate an ICFP string
    icfp_string = "B. SF B$ B$ L\" B$ L\" B$ L# B$ v\" B$ v# v# L# B$ v\" B$ v# v# L$ L# ? B= v# I\" v\" B. v\" B$ v$ B- v# I\" Sl I#"
    icfp_string = "B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK"
    tokens = icfp_string.split()
    result = evaluate(tokens)
    print("Evaluation result:", result)

    # Encode values to ICFP
    boolean_value = True
    integer_value = 1337
    string_value = "Hello World!"
    
    icfp_boolean = encode_icfp(boolean_value)
    icfp_integer = encode_icfp(integer_value)
    icfp_string_value = encode_icfp(string_value)
    
    print("Encoded boolean:", icfp_boolean)
    print("Encoded integer:", icfp_integer)
    print("Encoded string:", icfp_string_value)

if __name__ == "__main__":
    main()

