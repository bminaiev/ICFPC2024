import string
import sys

sys.setrecursionlimit(2000)

# Mapping of characters for encoding and decoding
ENCODING_CHARS = (
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    "!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
)

def decode_base94(encoded_str):
    print(f"decoding number {encoded_str}")
    base = len(ENCODING_CHARS)
    decoded_value = 0
    for char in encoded_str:
        decoded_value = decoded_value * base + ord(char) - 33
    print(f"got {decoded_value}")
    return decoded_value

def encode_base94(value):
    if value == 0:
        return chr(33)
    base = len(ENCODING_CHARS)
    encoded_str = ""
    while value > 0:
        encoded_str = ENCODING_CHARS[base - value % base] + encoded_str
        value //= base
    return encoded_str

def encode_string(encoded_str):
    return ''.join(chr(ENCODING_CHARS.index(char) + 33) for char in encoded_str)

def decode_string(value):
    return ''.join(ENCODING_CHARS[ord(char) - 33] for char in value)

def extract_lambda_body(tokens):
    # A helper function to extract the lambda body until the matching scope ends
    depth = 1
    body_tokens = []
    while depth > 0 and tokens:
        token = tokens.pop(0)
        body_tokens.append(token)
        if token.startswith('L'):
            depth += 0
        elif token.startswith('B'):
            depth += 1
        elif token.startswith('?'):
            depth += 2
        else:
            depth -= 1
    return body_tokens

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
                print(f"calling {x} with {y}")
                return x(y)
            else:
                raise TypeError(f"Expected a callable for application, got {x}")
    elif indicator == '?':
        condition = evaluate(tokens, env)
        tokens_true = extract_lambda_body(tokens)
        tokens_false = extract_lambda_body(tokens)
        #if_true = evaluate(tokens, env)
        #if_false = evaluate(tokens, env)
        return evaluate(tokens_true, env) if condition else evaluate(tokens_false, env)
    elif indicator == 'L':
        variable_number = decode_base94(body)
        lambda_body = extract_lambda_body(tokens)
        print(lambda_body)
        current_env = env.copy()
        res = lambda arg: evaluate(lambda_body.copy(), {**current_env, variable_number: arg})
        print("lambda parsed")
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
    icfp_string = """B. SF B$ B$ L" B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L$ L# ? B= v# I" v" B. v" B$ v$ B- v# I" Sl I#"""
    icfp_string = """B$ L+ B. B. SF B$ B$ v+ Sl IR B$ B$ v+ B. S~ B$ B$ v+ Sl IS IR L" B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L$ L# ? B= v# I" v" B. v" B$ v$ B- v# I\""""
    #icfp_string = "B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK"
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

