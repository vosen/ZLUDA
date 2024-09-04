import os, sys, subprocess


SPACE = [".reg", ".sreg", ".param", ".param::entry",  ".param::func", ".local", ".global", ".const", ".shared", ".shared::cta", ".shared::cluster"]
TYPE_AND_INIT = ["", " = 1", "[1]", "[1] = {1}"]
MULTIVAR =  ["", "<1>" ]
VECTOR =  ["", ".v2" ]

HEADER = """
    .version 8.5
    .target sm_90
    .address_size 64
"""


def directive(space, variable, multivar, vector):
    return """{3}
    {0} {4} .b32 variable{2} {1};
    """.format(space, variable, multivar, HEADER, vector)

def entry_arg(space, variable, multivar, vector):
    return """{3}
    .entry  foobar ( {0} {4} .b32 variable{2} {1})
    {{
    ret;
    }}
    """.format(space, variable, multivar, HEADER, vector)


def fn_arg(space, variable, multivar, vector):
    return """{3}
    .func foobar ( {0} {4} .b32 variable{2} {1})
    {{
    ret;
    }}
    """.format(space, variable, multivar, HEADER, vector)


def fn_body(space, variable, multivar, vector):
    return """{3}
    .func foobar ()
    {{
    {0} {4} .b32 variable{2} {1};
    ret;
    }}
    """.format(space, variable, multivar, HEADER, vector)


def generate(generator):
    legal = []
    for space in SPACE:
        for init in TYPE_AND_INIT:
            for multi in MULTIVAR:
                for vector in VECTOR:
                    ptx = generator(space, init, multi, vector)
                    if 0 == subprocess.call(["C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.6\\bin\\ptxas.exe", "-arch", "sm_90", "-ias", ptx], stdout = subprocess.DEVNULL): #
                        legal.append((space, vector, init, multi))
    print(generator.__name__)
    print(legal)


def main():
    generate(directive)
    generate(entry_arg)
    generate(fn_arg)
    generate(fn_body)

if __name__ == "__main__":
    main()
