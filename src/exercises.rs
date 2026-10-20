pub struct Exercise {
    pub id: &'static str,
    pub description: &'static str,
    pub prototype: &'static str,
    pub test_main: &'static str,
    pub expected: &'static str,
}

// Deliberately mixed order: neighbours never share a kind, so the list does not
// climb from easiest to hardest.
pub const EXERCISES: [Exercise; 25] = [
    Exercise {
        id: "ft_strlen",
        description: "Write a function that returns the length of the string \
                      passed as a parameter (without the terminating '\\0').",
        prototype: "int ft_strlen(char *str);",
        test_main: "#include <stdio.h>\n\nint\tft_strlen(char *str);\n\nint\tmain(void)\n{\n\tprintf(\"%d\\n\", ft_strlen(\"moulinette\"));\n\treturn (0);\n}\n",
        expected: "10\n",
    },
    Exercise {
        id: "ft_putstr",
        description: "Write a function that displays the string passed as a \
                      parameter on the standard output, and nothing else (no \
                      trailing newline). An empty string displays nothing. Use \
                      the write syscall (unistd.h).",
        prototype: "void ft_putstr(char *str);",
        test_main: "#include <stdio.h>\n\nvoid\tft_putstr(char *str);\n\nint\tmain(void)\n{\n\tft_putstr(\"moulinette\");\n\tft_putstr(\"\");\n\tprintf(\"\\n\");\n\treturn (0);\n}\n",
        expected: "moulinette\n",
    },
    Exercise {
        id: "ft_strcpy",
        description: "Write a function that copies the string src into dest, \
                      including the terminating '\\0', and returns dest.",
        prototype: "char *ft_strcpy(char *dest, char *src);",
        test_main: "#include <stdio.h>\n\nchar\t*ft_strcpy(char *dest, char *src);\n\nint\tmain(void)\n{\n\tchar\tbuf[32];\n\tchar\t*r;\n\n\tr = ft_strcpy(buf, \"moulinette\");\n\tprintf(\"%s %d\\n\", buf, r == buf);\n\treturn (0);\n}\n",
        expected: "moulinette 1\n",
    },
    Exercise {
        id: "ft_isalpha",
        description: "Write a function that returns a non-zero value if the \
                      character is an alphabetic letter ('a'-'z' or 'A'-'Z'), \
                      and 0 otherwise.",
        prototype: "int ft_isalpha(int c);",
        test_main: "#include <stdio.h>\n\nint\tft_isalpha(int c);\n\nint\tmain(void)\n{\n\tprintf(\"%d %d %d %d\\n\", ft_isalpha('a') != 0, ft_isalpha('Z') != 0, ft_isalpha('0') != 0, ft_isalpha(' ') != 0);\n\treturn (0);\n}\n",
        expected: "1 1 0 0\n",
    },
    Exercise {
        id: "ft_putnbr",
        description: "Write a function that displays the integer passed as a \
                      parameter in decimal on the standard output. It must \
                      handle negative values, including INT_MIN \
                      (-2147483648). Use the write syscall (unistd.h).",
        prototype: "void ft_putnbr(int nb);",
        test_main: "#include <stdio.h>\n\nvoid\tft_putnbr(int nb);\n\nint\tmain(void)\n{\n\tft_putnbr(-2147483647 - 1);\n\tprintf(\"\\n\");\n\treturn (0);\n}\n",
        expected: "-2147483648\n",
    },
    Exercise {
        id: "ft_strcmp",
        description: "Write a function that compares the two strings \
                      lexicographically and returns an integer whose sign \
                      reflects the result: 0 if equal, negative if s1 < s2, \
                      positive if s1 > s2.",
        prototype: "int ft_strcmp(char *s1, char *s2);",
        test_main: "#include <stdio.h>\n\nint\tft_strcmp(char *s1, char *s2);\n\nint\tmain(void)\n{\n\tint\ta = ft_strcmp(\"abc\", \"abc\");\n\tint\tb = ft_strcmp(\"abc\", \"abd\");\n\tint\tc = ft_strcmp(\"abd\", \"abc\");\n\n\tprintf(\"%d %d %d\\n\", a == 0, b < 0, c > 0);\n\treturn (0);\n}\n",
        expected: "1 1 1\n",
    },
    Exercise {
        id: "ft_bzero",
        description: "Write a function that sets the first n bytes of the \
                      memory area pointed to by s to zero. Nothing is returned.",
        prototype: "void ft_bzero(void *s, unsigned int n);",
        test_main: "#include <stdio.h>\n\nvoid\tft_bzero(void *s, unsigned int n);\n\nint\tmain(void)\n{\n\tunsigned char\tb[4] = {1, 2, 3, 4};\n\n\tft_bzero(b, 3);\n\tprintf(\"%d %d %d %d\\n\", b[0], b[1], b[2], b[3]);\n\treturn (0);\n}\n",
        expected: "0 0 0 4\n",
    },
    Exercise {
        id: "ft_toupper",
        description: "Write a function that returns the uppercase version of \
                      a lowercase letter, and the character unchanged for any \
                      other value.",
        prototype: "int ft_toupper(int c);",
        test_main: "#include <stdio.h>\n\nint\tft_toupper(int c);\n\nint\tmain(void)\n{\n\tprintf(\"%c%c%c%c\\n\", ft_toupper('a'), ft_toupper('z'), ft_toupper('A'), ft_toupper('1'));\n\treturn (0);\n}\n",
        expected: "AZA1\n",
    },
    Exercise {
        id: "ft_strcat",
        description: "Write a function that appends the string src to the end \
                      of dest (overwriting the terminating '\\0'), \
                      '\\0'-terminates the result and returns dest.",
        prototype: "char *ft_strcat(char *dest, char *src);",
        test_main: "#include <stdio.h>\n\nchar\t*ft_strcat(char *dest, char *src);\n\nint\tmain(void)\n{\n\tchar\tbuf[32] = \"moulinette\";\n\tchar\t*r;\n\n\tr = ft_strcat(buf, \"42\");\n\tprintf(\"%s %d\\n\", buf, r == buf);\n\treturn (0);\n}\n",
        expected: "moulinette42 1\n",
    },
    Exercise {
        id: "ft_isdigit",
        description: "Write a function that returns a non-zero value if the \
                      character passed as a parameter is a digit between '0' \
                      and '9', and 0 otherwise.",
        prototype: "int ft_isdigit(int c);",
        test_main: "#include <stdio.h>\n\nint\tft_isdigit(int c);\n\nint\tmain(void)\n{\n\tprintf(\"%d %d %d %d\\n\", ft_isdigit('0') != 0, ft_isdigit('9') != 0, ft_isdigit('/') != 0, ft_isdigit('a') != 0);\n\treturn (0);\n}\n",
        expected: "1 1 0 0\n",
    },
    Exercise {
        id: "ft_strchr",
        description: "Write a function that returns a pointer to the first \
                      occurrence of the character c in the string str, or NULL \
                      if it does not appear. Searching for '\\0' must return a \
                      pointer to the terminator.",
        prototype: "char *ft_strchr(char *str, int c);",
        test_main: "#include <stdio.h>\n#include <stddef.h>\n\nchar\t*ft_strchr(char *str, int c);\n\nint\tmain(void)\n{\n\tchar\ts[] = \"moulinette\";\n\n\tprintf(\"%s %d %d\\n\", ft_strchr(s, 'l'), ft_strchr(s, 'z') == NULL, ft_strchr(s, 0) == s + 10);\n\treturn (0);\n}\n",
        expected: "linette 1 1\n",
    },
    Exercise {
        id: "ft_putchar",
        description: "Write a function that displays the character passed as a \
                      parameter on the standard output. Use the write syscall \
                      (unistd.h).",
        prototype: "void ft_putchar(char c);",
        test_main: "void\tft_putchar(char c);\n\nint\tmain(void)\n{\n\tft_putchar('4');\n\tft_putchar('2');\n\tft_putchar('\\n');\n\treturn (0);\n}\n",
        expected: "42\n",
    },
    Exercise {
        id: "ft_atoi",
        description: "Write a function that converts the beginning of the \
                      string to an int: skip whitespace, accept one optional \
                      '+' or '-', then read digits until the first non-digit. \
                      Return 0 when nothing can be converted.",
        prototype: "int ft_atoi(char *str);",
        test_main: "#include <stdio.h>\n\nint\tft_atoi(char *str);\n\nint\tmain(void)\n{\n\tprintf(\"%d %d %d %d %d %d %d\\n\", ft_atoi(\"42\"), ft_atoi(\"   -42abc\"), ft_atoi(\"+007\"), ft_atoi(\"abc\"), ft_atoi(\"\\t\\n 42\"), ft_atoi(\"--42\"), ft_atoi(\"\"));\n\treturn (0);\n}\n",
        expected: "42 -42 7 0 42 0 0\n",
    },
    Exercise {
        id: "ft_isascii",
        description: "Write a function that returns a non-zero value if the \
                      character is part of the ASCII table (0 to 127), and 0 \
                      otherwise.",
        prototype: "int ft_isascii(int c);",
        test_main: "#include <stdio.h>\n\nint\tft_isascii(int c);\n\nint\tmain(void)\n{\n\tprintf(\"%d %d %d %d\\n\", ft_isascii('A') != 0, ft_isascii(0) != 0, ft_isascii(127) != 0, ft_isascii(128) != 0);\n\treturn (0);\n}\n",
        expected: "1 1 1 0\n",
    },
    Exercise {
        id: "ft_strncpy",
        description: "Write a function that copies at most n characters from \
                      src into dest. If src is shorter than n, the remaining \
                      bytes must be filled with '\\0'. It never writes more than \
                      n bytes and returns dest.",
        prototype: "char *ft_strncpy(char *dest, char *src, unsigned int n);",
        test_main: "#include <stdio.h>\n\nchar\t*ft_strncpy(char *dest, char *src, unsigned int n);\n\nint\tmain(void)\n{\n\tchar\tbuf[8];\n\tchar\tbuf2[4];\n\tint\t\ti;\n\n\ti = 0;\n\twhile (i < 8)\n\t{\n\t\tbuf[i] = 'X';\n\t\ti++;\n\t}\n\ti = 0;\n\twhile (i < 4)\n\t{\n\t\tbuf2[i] = 'X';\n\t\ti++;\n\t}\n\tft_strncpy(buf, \"ab\", 4);\n\tft_strncpy(buf2, \"ab\", 0);\n\tprintf(\"%d %d %d %d %d %d\\n\", buf[0] == 'a', buf[1] == 'b', buf[2] == 0, buf[3] == 0, buf[4] == 'X', buf2[0] == 'X');\n\treturn (0);\n}\n",
        expected: "1 1 1 1 1 1\n",
    },
    Exercise {
        id: "ft_tolower",
        description: "Write a function that returns the lowercase version of \
                      an uppercase letter, and the character unchanged for any \
                      other value.",
        prototype: "int ft_tolower(int c);",
        test_main: "#include <stdio.h>\n\nint\tft_tolower(int c);\n\nint\tmain(void)\n{\n\tprintf(\"%c%c%c%c\\n\", ft_tolower('A'), ft_tolower('Z'), ft_tolower('a'), ft_tolower('1'));\n\treturn (0);\n}\n",
        expected: "aza1\n",
    },
    Exercise {
        id: "ft_memset",
        description: "Write a function that fills the first n bytes of the \
                      memory area pointed to by s with the byte c, and returns \
                      s.",
        prototype: "void *ft_memset(void *s, int c, unsigned int n);",
        test_main: "#include <stdio.h>\n\nvoid\t*ft_memset(void *s, int c, unsigned int n);\n\nint\tmain(void)\n{\n\tunsigned char\tb[4] = {0};\n\tvoid\t\t\t*r;\n\n\tr = ft_memset(b, 'A', 3);\n\tprintf(\"%c%c%c %d %d\\n\", b[0], b[1], b[2], b[3] == 0, r == (void *)b);\n\treturn (0);\n}\n",
        expected: "AAA 1 1\n",
    },
    Exercise {
        id: "ft_strncmp",
        description: "Write a function that compares at most n characters of \
                      the two strings and returns an integer whose sign \
                      reflects the result of the comparison.",
        prototype: "int ft_strncmp(char *s1, char *s2, unsigned int n);",
        test_main: "#include <stdio.h>\n\nint\tft_strncmp(char *s1, char *s2, unsigned int n);\n\nint\tmain(void)\n{\n\tprintf(\"%d %d %d %d\\n\", ft_strncmp(\"abc\", \"abd\", 2) == 0, ft_strncmp(\"abc\", \"abd\", 3) < 0, ft_strncmp(\"abd\", \"abc\", 3) > 0, ft_strncmp(\"abc\", \"abd\", 0) == 0);\n\treturn (0);\n}\n",
        expected: "1 1 1 1\n",
    },
    Exercise {
        id: "ft_is_negative",
        description: "Write a function that displays 'N' if the integer passed \
                      as a parameter is negative, and 'P' if it is positive or \
                      zero. Use the write syscall (unistd.h).",
        prototype: "void ft_is_negative(int n);",
        test_main: "#include <stdio.h>\n\nvoid\tft_is_negative(int n);\n\nint\tmain(void)\n{\n\tft_is_negative(-42);\n\tft_is_negative(-1);\n\tft_is_negative(0);\n\tft_is_negative(42);\n\tprintf(\"\\n\");\n\treturn (0);\n}\n",
        expected: "NNPP\n",
    },
    Exercise {
        id: "ft_strrchr",
        description: "Write a function that returns a pointer to the last \
                      occurrence of the character c in the string str, or NULL \
                      if it does not appear. Searching for '\\0' must return a \
                      pointer to the terminator.",
        prototype: "char *ft_strrchr(char *str, int c);",
        test_main: "#include <stdio.h>\n#include <stddef.h>\n\nchar\t*ft_strrchr(char *str, int c);\n\nint\tmain(void)\n{\n\tchar\ts[] = \"moulinette\";\n\n\tprintf(\"%s %d %d\\n\", ft_strrchr(s, 't'), ft_strrchr(s, 'z') == NULL, ft_strrchr(s, 0) == s + 10);\n\treturn (0);\n}\n",
        expected: "te 1 1\n",
    },
    Exercise {
        id: "ft_isalnum",
        description: "Write a function that returns a non-zero value if the \
                      character is a letter or a digit, and 0 otherwise.",
        prototype: "int ft_isalnum(int c);",
        test_main: "#include <stdio.h>\n\nint\tft_isalnum(int c);\n\nint\tmain(void)\n{\n\tprintf(\"%d %d %d %d\\n\", ft_isalnum('a') != 0, ft_isalnum('9') != 0, ft_isalnum('_') != 0, ft_isalnum(' ') != 0);\n\treturn (0);\n}\n",
        expected: "1 1 0 0\n",
    },
    Exercise {
        id: "ft_strdup",
        description: "Write a function that allocates memory (malloc) and \
                      returns a duplicate of the string passed as a parameter. \
                      The copy must be independent of the original and properly \
                      '\\0'-terminated. Return NULL on allocation failure.",
        prototype: "char *ft_strdup(char *src);",
        test_main: "#include <stdio.h>\n#include <string.h>\n#include <stdlib.h>\n\nchar\t*ft_strdup(char *src);\n\nint\tmain(void)\n{\n\tchar\tsrc[] = \"moulinette\";\n\tchar\t*dup = ft_strdup(src);\n\n\tif (!dup)\n\t\treturn (1);\n\tsrc[0] = 'X';\n\tprintf(\"%s %d\\n\", dup, strcmp(dup, \"moulinette\"));\n\tfree(dup);\n\treturn (0);\n}\n",
        expected: "moulinette 0\n",
    },
    Exercise {
        id: "ft_strncat",
        description: "Write a function that appends at most n characters of \
                      src to the end of dest, '\\0'-terminates the result and \
                      returns dest.",
        prototype: "char *ft_strncat(char *dest, char *src, unsigned int n);",
        test_main: "#include <stdio.h>\n\nchar\t*ft_strncat(char *dest, char *src, unsigned int n);\n\nint\tmain(void)\n{\n\tchar\tbuf[32] = \"moulinette\";\n\n\tft_strncat(buf, \"42abc\", 2);\n\tft_strncat(buf, \"!\", 10);\n\tprintf(\"%s\\n\", buf);\n\treturn (0);\n}\n",
        expected: "moulinette42!\n",
    },
    Exercise {
        id: "ft_isprint",
        description: "Write a function that returns a non-zero value if the \
                      character is printable, space included, and 0 otherwise.",
        prototype: "int ft_isprint(int c);",
        test_main: "#include <stdio.h>\n\nint\tft_isprint(int c);\n\nint\tmain(void)\n{\n\tprintf(\"%d %d %d %d\\n\", ft_isprint(' ') != 0, ft_isprint('~') != 0, ft_isprint(31) != 0, ft_isprint(127) != 0);\n\treturn (0);\n}\n",
        expected: "1 1 0 0\n",
    },
    Exercise {
        id: "ft_swap",
        description: "Write a function that swaps the values pointed to by the \
                      two integer pointers.",
        prototype: "void ft_swap(int *a, int *b);",
        test_main: "#include <stdio.h>\n\nvoid\tft_swap(int *a, int *b);\n\nint\tmain(void)\n{\n\tint\ta = 1;\n\tint\tb = 2;\n\n\tft_swap(&a, &b);\n\tprintf(\"%d %d\\n\", a, b);\n\treturn (0);\n}\n",
        expected: "2 1\n",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    fn category(id: &str) -> &'static str {
        match id {
            "ft_strlen" => "len",
            "ft_putchar" | "ft_putstr" | "ft_putnbr" => "out",
            "ft_isalpha" | "ft_isalnum" | "ft_isascii" | "ft_isprint" | "ft_isdigit"
            | "ft_is_negative" => "pred",
            "ft_toupper" | "ft_tolower" => "case",
            "ft_strcpy" | "ft_strncpy" | "ft_strcat" | "ft_strncat" => "copy",
            "ft_strcmp" | "ft_strncmp" => "cmp",
            "ft_strchr" | "ft_strrchr" => "search",
            "ft_strdup" | "ft_memset" | "ft_bzero" => "mem",
            "ft_atoi" => "conv",
            "ft_swap" => "swap",
            _ => "",
        }
    }

    #[test]
    fn ids_are_unique_and_every_id_has_a_category() {
        for (i, ex) in EXERCISES.iter().enumerate() {
            assert!(!ex.id.is_empty());
            assert!(!category(ex.id).is_empty(), "{} has no category", ex.id);
            assert!(
                !EXERCISES[..i].iter().any(|other| other.id == ex.id),
                "duplicate id {}",
                ex.id
            );
        }
    }

    #[test]
    fn neighbours_mix_categories() {
        for pair in EXERCISES.windows(2) {
            assert_ne!(
                category(pair[0].id),
                category(pair[1].id),
                "{} and {} are both {}",
                pair[0].id,
                pair[1].id,
                category(pair[0].id)
            );
        }
    }

    #[test]
    fn every_expected_output_is_newline_terminated() {
        for ex in &EXERCISES {
            assert!(
                ex.expected.ends_with('\n'),
                "{} has no trailing newline",
                ex.id
            );
        }
    }
}
