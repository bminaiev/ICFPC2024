from typing import List, Union, Callable, Dict
import sys
import string

sys.setrecursionlimit(2000)

class ICFPValue:
    pass

class Boolean(ICFPValue):
    def __init__(self, value: bool):
        self.value = value

class Integer(ICFPValue):
    def __init__(self, value: int):
        self.value = value

class String(ICFPValue):
    def __init__(self, value: str):
        self.value = value

class Lambda(ICFPValue):
    def __init__(self, var_num: int, body, env: Dict[int, ICFPValue]):
        self.var_num = var_num
        self.body = body
        self.env = env

class Variable(ICFPValue):
    def __init__(self, var_num: int):
        self.var_num = var_num

class Application(ICFPValue):
    def __init__(self, func, arg):
        self.func = func
        self.arg = arg

class BuiltinFunction(ICFPValue):
    def __init__(self, func):
        self.func = func

def parse_tokens(tokens: List[str]) -> ICFPValue:
    if not tokens:
        raise ValueError("Empty token list")

    token = tokens.pop(0)
    indicator = token[0]
    body = token[1:]

    if indicator == 'T':
        return Boolean(True)
    elif indicator == 'F':
        return Boolean(False)
    elif indicator == 'I':
        return Integer(base94_to_int(body))
    elif indicator == 'S':
        return String(decode_string(body))
    elif indicator == 'U':
        op = body
        arg = parse_tokens(tokens)
        return Application(BuiltinFunction(lambda x: apply_unary_op(op, x)), arg)
    elif indicator == 'B':
        op = body
        arg1 = parse_tokens(tokens)
        arg2 = parse_tokens(tokens)
        return Application(Application(BuiltinFunction(lambda x: BuiltinFunction(lambda y: apply_binary_op(op, x, y))), arg1), arg2)
    elif indicator == '?':
        cond = parse_tokens(tokens)
        true_branch = parse_tokens(tokens)
        false_branch = parse_tokens(tokens)
        return Application(Application(Application(BuiltinFunction(lambda c: BuiltinFunction(lambda t: BuiltinFunction(lambda f: t if c.value else f))), cond), true_branch), false_branch)
    elif indicator == 'L':
        var_num = base94_to_int(body)
        lambda_body = parse_tokens(tokens)
        return Lambda(var_num, lambda_body, {})
    elif indicator == 'v':
        return Variable(base94_to_int(body))
    else:
        raise ValueError(f"Unknown indicator: {indicator}")

def base94_to_int(s: str) -> int:
    base94_chars = string.printable[:-6]
    return sum(base94_chars.index(c) * (94 ** i) for i, c in enumerate(reversed(s)))

def int_to_base94(n: int) -> str:
    base94_chars = string.printable[:-6]
    if n == 0:
        return base94_chars[0]
    result = ""
    while n > 0:
        n, r = divmod(n, 94)
        result = base94_chars[r] + result
    return result

def decode_string(s: str) -> str:
    char_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
    return ''.join(char_map[ord(c) - 33] for c in s)

def encode_string(s: str) -> str:
    char_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
    return ''.join(chr(char_map.index(c) + 33) for c in s)

def apply_unary_op(op: str, arg: ICFPValue) -> ICFPValue:
    arg = evaluate(arg)
    if op == '-':
        return Integer(-arg.value)
    elif op == '!':
        return Boolean(not arg.value)
    elif op == '#':
        return Integer(base94_to_int(arg.value))
    elif op == '$':
        return String(int_to_base94(arg.value))
    else:
        raise ValueError(f"Unknown unary operator: {op}")

def apply_binary_op(op: str, arg1: ICFPValue, arg2: ICFPValue) -> ICFPValue:
    arg1, arg2 = evaluate(arg1), evaluate(arg2)
    if op in '+-*/%<>=':
        return apply_arithmetic_op(op, arg1, arg2)
    elif op in '|&':
        return apply_boolean_op(op, arg1, arg2)
    elif op == '.':
        return String(arg1.value + arg2.value)
    elif op == 'T':
        return String(arg2.value[:arg1.value])
    elif op == 'D':
        return String(arg2.value[arg1.value:])
    elif op == '$':
        return apply_lambda(arg1, arg2)
    else:
        raise ValueError(f"Unknown binary operator: {op}")

def apply_arithmetic_op(op: str, arg1: ICFPValue, arg2: ICFPValue) -> ICFPValue:
    if op == '+':
        return Integer(arg1.value + arg2.value)
    elif op == '-':
        return Integer(arg1.value - arg2.value)
    elif op == '*':
        return Integer(arg1.value * arg2.value)
    elif op == '/':
        return Integer(arg1.value // arg2.value)
    elif op == '%':
        return Integer(arg1.value % arg2.value)
    elif op == '<':
        return Boolean(arg1.value < arg2.value)
    elif op == '>':
        return Boolean(arg1.value > arg2.value)
    elif op == '=':
        return Boolean(arg1.value == arg2.value)

def apply_boolean_op(op: str, arg1: ICFPValue, arg2: ICFPValue) -> ICFPValue:
    if op == '|':
        return Boolean(arg1.value or arg2.value)
    elif op == '&':
        return Boolean(arg1.value and arg2.value)

def apply_lambda(lambda_expr: Lambda, arg: ICFPValue) -> ICFPValue:
    new_env = lambda_expr.env.copy()
    new_env[lambda_expr.var_num] = arg
    return substitute(lambda_expr.body, new_env)

def evaluate(expr: ICFPValue) -> ICFPValue:
    while True:
        if isinstance(expr, (Boolean, Integer, String)):
            return expr
        elif isinstance(expr, Variable):
            raise ValueError(f"Unbound variable: {expr.var_num}")
        elif isinstance(expr, Lambda):
            return expr
        elif isinstance(expr, BuiltinFunction):
            return expr
        elif isinstance(expr, Application):
            func = evaluate(expr.func)
            if isinstance(func, Lambda):
                new_env = func.env.copy()
                new_env[func.var_num] = expr.arg
                expr = substitute(func.body, new_env)
            elif isinstance(func, BuiltinFunction):
                expr = func.func(expr.arg)
            else:
                raise ValueError(f"Cannot apply non-function: {func}")
        else:
            raise ValueError(f"Unknown expression type: {type(expr)}")

def substitute(expr: ICFPValue, env: Dict[int, ICFPValue]) -> ICFPValue:
    if isinstance(expr, Variable):
        if expr.var_num in env:
            return env[expr.var_num]
        else:
            return expr
    elif isinstance(expr, Lambda):
        new_env = {k: v for k, v in env.items() if k != expr.var_num}
        return Lambda(expr.var_num, expr.body, new_env)
    elif isinstance(expr, Application):
        return Application(substitute(expr.func, env), substitute(expr.arg, env))
    else:
        return expr

def interpret_icfp(program: str) -> ICFPValue:
    tokens = program.split()
    ast = parse_tokens(tokens)
    return evaluate(ast)

result = interpret_icfp("""B. SF B$ B$ L" B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L$ L# ? B= v# I" v" B. v" B$ v$ B- v# I" Sl I#,""")
print(result.value)
