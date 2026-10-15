pub struct Exercise {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub prototype: &'static str,
    pub skeleton: &'static str,
    pub test_main: &'static str,
    pub expected: &'static str,
}

pub const EXERCISES: [Exercise; 5] = [
    Exercise {
        id: "ft_putchar",
        title: "Exercise 1 - ft_putchar",
        description: "Write a function that displays the character passed as a \
                      parameter on the standard output. Use the write syscall \
                      (unistd.h).",
        prototype: "void ft_putchar(char c);",
        skeleton: "#include <unistd.h>\n\nvoid\tft_putchar(char c)\n{\n}\n",
        test_main: "void\tft_putchar(char c);\n\nint\tmain(void)\n{\n\tft_putchar('4');\n\tft_putchar('2');\n\tft_putchar('\\n');\n\treturn (0);\n}\n",
        expected: "42\n",
    },
    Exercise {
        id: "ft_strlen",
        title: "Exercise 2 - ft_strlen",
        description: "Write a function that returns the length of the string \
                      passed as a parameter (without the terminating '\\0').",
        prototype: "int ft_strlen(char *str);",
        skeleton: "int\tft_strlen(char *str)\n{\n}\n",
        test_main: "#include <stdio.h>\n\nint\tft_strlen(char *str);\n\nint\tmain(void)\n{\n\tprintf(\"%d\\n\", ft_strlen(\"moulinette\"));\n\treturn (0);\n}\n",
        expected: "10\n",
    },
    Exercise {
        id: "ft_swap",
        title: "Exercise 3 - ft_swap",
        description: "Write a function that swaps the values pointed to by the \
                      two integer pointers.",
        prototype: "void ft_swap(int *a, int *b);",
        skeleton: "void\tft_swap(int *a, int *b)\n{\n}\n",
        test_main: "#include <stdio.h>\n\nvoid\tft_swap(int *a, int *b);\n\nint\tmain(void)\n{\n\tint\ta = 1;\n\tint\tb = 2;\n\n\tft_swap(&a, &b);\n\tprintf(\"%d %d\\n\", a, b);\n\treturn (0);\n}\n",
        expected: "2 1\n",
    },
    Exercise {
        id: "fix_it",
        title: "Exercise 4 - fix_it",
        description: "The function below is provided but contains a bug: it \
                      skips the first character of the string. Find the error \
                      and fix it so the whole string is displayed.",
        prototype: "void ft_putstr(char *str);",
        skeleton: "#include <unistd.h>\n\nvoid\tft_putstr(char *str)\n{\n\tint\ti;\n\n\ti = 1;\n\twhile (str[i])\n\t{\n\t\twrite(1, &str[i], 1);\n\t\ti++;\n\t}\n}\n",
        test_main: "void\tft_putstr(char *str);\n\nint\tmain(void)\n{\n\tft_putstr(\"moulinette\\n\");\n\treturn (0);\n}\n",
        expected: "moulinette\n",
    },
    Exercise {
        id: "ft_strcmp",
        title: "Exercise 5 - ft_strcmp",
        description: "Write a function that compares the two strings \
                      lexicographically and returns an integer whose sign \
                      reflects the result: 0 if equal, negative if s1 < s2, \
                      positive if s1 > s2.",
        prototype: "int ft_strcmp(char *s1, char *s2);",
        skeleton: "int\tft_strcmp(char *s1, char *s2)\n{\n}\n",
        test_main: "#include <stdio.h>\n\nint\tft_strcmp(char *s1, char *s2);\n\nint\tmain(void)\n{\n\tint\ta = ft_strcmp(\"abc\", \"abc\");\n\tint\tb = ft_strcmp(\"abc\", \"abd\");\n\tint\tc = ft_strcmp(\"abd\", \"abc\");\n\n\tprintf(\"%d %d %d\\n\", a == 0, b < 0, c > 0);\n\treturn (0);\n}\n",
        expected: "1 1 1\n",
    },
];
