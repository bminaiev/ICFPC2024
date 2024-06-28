from typing import List, Dict, Union
import string

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
    def __init__(self, var_num: int, body, env: Dict[int, 'Thunk']):
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

class Thunk:
    def __init__(self, expr, env):
        self.expr = expr
        self.env = env
        self.evaluated = None

    def force(self):
        if self.evaluated is None:
            self.evaluated = evaluate(self.expr, self.env)
        return self.evaluated

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
        return Application(create_unary_op(op), arg)
    elif indicator == 'B':
        op = body
        arg1 = parse_tokens(tokens)
        arg2 = parse_tokens(tokens)
        return Application(Application(create_binary_op(op), arg1), arg2)
    elif indicator == '?':
        cond = parse_tokens(tokens)
        true_branch = parse_tokens(tokens)
        false_branch = parse_tokens(tokens)
        return Application(Application(Application(Lambda(0, Lambda(1, Lambda(2, Application(Application(Variable(0), Variable(1)), Variable(2)), {}), {}), {}), cond), true_branch), false_branch)
    elif indicator == 'L':
        var_num = base94_to_int(body)
        lambda_body = parse_tokens(tokens)
        return Lambda(var_num, lambda_body, {})
    elif indicator == 'v':
        return Variable(base94_to_int(body))
    else:
        raise ValueError(f"Unknown indicator: {indicator}")

def base94_to_int(s: str) -> int:
    return sum((ord(c) - 33) * (94 ** i) for i, c in enumerate(reversed(s)))

def int_to_base94(n: int) -> str:
    if n == 0:
        return '!'
    result = ""
    while n > 0:
        n, r = divmod(n, 94)
        result = chr(r + 33) + result
    return result

def decode_string(s: str) -> str:
    char_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
    return ''.join(char_map[ord(c) - 33] for c in s)

def create_unary_op(op: str) -> Lambda:
    return Lambda(0, create_unary_op_body(op), {})

def create_unary_op_body(op: str) -> ICFPValue:
    if op == '-':
        return Lambda(0, Integer(-evaluate(Variable(0), {}).value), {})
    elif op == '!':
        return Lambda(0, Boolean(not evaluate(Variable(0), {}).value), {})
    elif op == '#':
        return Lambda(0, Integer(base94_to_int(evaluate(Variable(0), {}).value)), {})
    elif op == '$':
        return Lambda(0, String(int_to_base94(evaluate(Variable(0), {}).value)), {})
    else:
        raise ValueError(f"Unknown unary operator: {op}")

def create_binary_op(op: str) -> Lambda:
    return Lambda(0, Lambda(1, create_binary_op_body(op), {}), {})

def create_binary_op_body(op: str) -> ICFPValue:
    op_map = {'+': lambda x, y: x + y, '-': lambda x, y: x - y, '*': lambda x, y: x * y, 
              '/': lambda x, y: x // y, '%': lambda x, y: x % y, '<': lambda x, y: x < y, 
              '>': lambda x, y: x > y, '=': lambda x, y: x == y}
    if op in op_map:
        return Lambda(0, Lambda(1, Integer(op_map[op](evaluate(Variable(0), {}).value, evaluate(Variable(1), {}).value)), {}), {})
    elif op in {'|', '&'}:
        bool_op_map = {'|': lambda x, y: x or y, '&': lambda x, y: x and y}
        return Lambda(0, Lambda(1, Boolean(bool_op_map[op](evaluate(Variable(0), {}).value, evaluate(Variable(1), {}).value)), {}), {})
    elif op == '.':
        return Lambda(0, Lambda(1, String(evaluate(Variable(0), {}).value + evaluate(Variable(1), {}).value), {}), {})
    elif op == 'T':
        return Lambda(0, Lambda(1, String(evaluate(Variable(1), {}).value[:evaluate(Variable(0), {}).value]), {}), {})
    elif op == 'D':
        return Lambda(0, Lambda(1, String(evaluate(Variable(1), {}).value[evaluate(Variable(0), {}).value:]), {}), {})
    elif op == '$':
        return Lambda(0, Lambda(1, Application(Variable(0), Variable(1)), {}), {})
    else:
        raise ValueError(f"Unknown binary operator: {op}")

def evaluate(expr: ICFPValue, env: Dict[int, Thunk]) -> ICFPValue:
    if isinstance(expr, (Boolean, Integer, String)):
        return expr
    elif isinstance(expr, Variable):
        if expr.var_num in env:
            return env[expr.var_num].force()
        else:
            raise ValueError(f"Unbound variable: {expr.var_num}")
    elif isinstance(expr, Lambda):
        return Lambda(expr.var_num, expr.body, env)
    elif isinstance(expr, Application):
        func = evaluate(expr.func, env)
        if isinstance(func, Lambda):
            new_env = {**func.env, func.var_num: Thunk(expr.arg, env)}
            return evaluate(func.body, new_env)
        else:
            raise ValueError(f"Cannot apply non-function: {func}")
    else:
        raise ValueError(f"Unknown expression type: {type(expr)}")

def interpret_icfp(program: str) -> ICFPValue:
    tokens = program.split()
    ast = parse_tokens(tokens)
    result = evaluate(ast, {})
    while isinstance(result, Thunk):
        result = result.force()
    return result

# Example usage
icfp_code = """B. SF B$ B$ L" B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L$ L# ? B= v# I" v" B. v" B$ v$ B- v# I" Sl I#,"""
result = interpret_icfp(icfp_code)
print(result.value)

