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
    icfp_string = """B$ L+ B. B. SF B$ B$ v+ Sl IR B$ B$ v+ B. S~ B$ B$ v+ Sl IS IR L" B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L$ L# ? B= v# I" v" B. v" B$ v$ B- v# I\","""
    icfp_string = """SX}W\\\\~VWY}Y]~V^Y}V[Y~XW^}YUZ~V[Y}W\\~YWU}VV^~V]U}ZXY~XZW}WZY~X^W}VZ^~YZ\\}^Z~W]\\}ZX]~VY]}\\^~V\\]}VUY~W\\[}XUY~XWV}Y^]~YVX}VXW~V[]}VX\\~WXX}Z[]~W]X}YXW~WXX}VY^~X\\\\}WZY~V][}V][~WZ[}ZY[~WUZ}ZXV~X[[}W\\Y~W\\W}X\\]~W^Y}XXU~Y\\]}]Z~YV[}VYU~V]U}]\\~X[X}X]Z~V]V}ZZU~W\\Y}ZYU~Y]X}X\\~XZ^}XZW~V][}Z]X~WU\\}VUY~X^]}WZ[~W\\\\}YZY~WWZ}V\\Z~V\\\\}^X~Y[W}VU\\~WUV}VW[~XUX}YZ[~XZ^}XYV~W\\^}Z[Y~W]X}YX[~XVX}YW]~X\\Z}V^]~Y[X}\\]~YZZ}VYV~X\\X}XZ]~WY^}ZXV~WYX}W[V~YYU}VU\\~WX[}ZX]~X\\Z}W[V~XYY}X[X~XVZ}ZU[~WXY}WVU~XUV}ZXW~VX]}\\X~WUW}VX\\~Y[W}YX~X\\V}XVU~XVV}X][~X\\U}W\\W~XUX}YYZ~Y[V}[U~W^X}Y][~XY\\}XW\\~VZ]}VWU~YWY}VYU~W\\V}X[]~YXV}VV^~YZY}ZV~W[Z}ZYV~V[Z}VU\\~VXW}ZZ~W^V}Y[\\~WVX}WWZ~YU^}WYZ~V^V}ZV~W^^}XUX~Y[[}VWU~XVX}XVW~XW^}XXZ~V[U}\\X~X[V}XXU~WWY}V]U~W^[}Y\\\\~X\\Z}WV\\~WYX}Z\\^~XY]}W^Y~WXW}ZZZ~WZ[}Z\\V~XUU}ZWY~Y\\^}[U~VX^}[Z~W[[}WZU~YV\\}V^V~XWU}XZW~X]\\}WWY~X\\U}W^\\~XU[}YV]~XWY}X]Y~VZ]}^Y~WY\\}ZZ^~XUV}X\\Y~V\\]}ZY~YX\\}YZ~WVX}V[\\~VX[}ZZ~X^U}VY]~XV^}YWV~VWU}XW~YUW}VYV~WUY}]Z~W^X}XU^~XUU}Y^U~WWY}WUZ~YU[}VXW~VY[}^Z~XU[}XZV~WUY}VZZ~WX^}WYY~W^[}ZX]~XVV}Y^V~VV]}W^~X\\U}X\\V~W^V}XZ^~YXV}V[X~WZV}V]V~X\\V}W^Y~WY^}XVZ~XX^}XZY~W^X}YXW~XY]}WZW~Y\\V}YU~W[]}ZYZ~X]X}XW\\~YUU}W^U~XUU}X\\\\~XUW}ZWZ~WZX}WX]~XWU}YZX~XZ\\}W[[~WUZ}WVX~WZV}XW\\~YUV}WX[~YWX}VV^~W[W}ZZZ~Y\\X}ZW~XUX}Y[[~XWY}YU]~WVZ}WY\\~X[[}X[[~V]\\}VUU~YXX}VUW~WXZ}V\\X~V\\\\}X[~W\\Z}ZY]~X[V}X[Z~X^[}VXV~VYU}[]~W\\^}WZ[~XZV}YU]~XYV}XY[~X^Z}XUV~WX\\}WZX~X\\U}W^U~YUU}V]\\~YVZ}V]V~XW[}XZV~YVY}VUU~YU]}VWU~W[\\}WVV~WV\\}V^\\~YU]}VWY~WW^}VYX~VZY}^W~X[V}X[]~YVX}V][~WWZ}Z]U~W[Y}WYZ~XZX}X^Y~W]X}XYW~XZZ}W\\Z~YV[}WZZ~WZZ}XXW~XY^}W]^~V^V}ZXY~W^[}Y[]~V]^}VW]~V]U}V[W~V]^}V[\\~Y]U}X]~W]W}ZW^~Y[]}Z^~XV\\}YXV~X]]}WZ]~XW^}YWZ~WUV}Z[^~W^U}ZZ\\~Y\\\\}[Y~XY[}W]]~XUV}ZWY~W[X}ZWX~WWW}WVZ~YX]}[Y~W^[}YWW~WXZ}V[X~V^U}ZYZ~V]Z}Z\\Z~W[\\}WYW~W\\\\}W\\Y~X[]}XX^~XYX}YVV~V]V}VYZ~XZ\\}XV^~W\\W}W^Y~X]^}W]V~WXY}WV]~WZ\\}V^W~XYW}W^X~X]W}WW[~X]V}WX^~XZX}WYX~W\\\\}XV[~X^^}W[V~XV\\}Y]Z~XYU}YX]~W[Z}WYX~W]Y}YX[~X^V}WYY~V[Z}]X~W[U}Z[Y~YZU}V[Y~WZ[}ZW\\~YUV}WYY~V]\\}Z\\Z~WX[}ZYY~V][}V][~Y[[}VV]~Y[W}[V~W[]}W\\V~WXY}VYU~WV\\}WU]~W\\Y}W]W~X^Z}V[U~WW[}VZ^~WVV}VYV~X\\]}V\\W~WZX}XUU~WWZ}Z]W~WVX}VU[~X^^}VYY~W\\X}Y^]~WY^}WU]~W[^}W\\^~W]Z}X[W~WY]}Z\\X~XU]}XY^~W[U}ZW^~XZ\\}W[Y~XWZ}Y[W~V]W}VW\\~Y[\\}\\^~WUY}WU]~WWV}V^Y~VZY}^Z~V]W}X]~XYW}XXY~YXV}WVZ~XY[}XZ\\~X[W}X]W~XXX}Y[V~V^U}VXZ~WZ\\}Z[X~W^Y}X\\Y~XUY}Y^[~W\\W}XV\\~YX\\}\\X~VYV}W[~XYY}YX[~VY]}\\Z~WZ\\}ZVX~V]V}^V~YV\\}WWV~YVY}WY]~WYZ}V]U~W[V}WW^~V]W}VVV~WU]}VYW~"""
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

