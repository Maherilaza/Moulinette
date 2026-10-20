use std::io;
use std::path::Path;
use std::process::Command;

use crate::exercises::Exercise;

pub const MAX_SCORE: u8 = 2;

pub struct GradeResult {
    pub compile_ok: bool,
    pub output_ok: bool,
    pub compiler_stderr: String,
    pub expected: String,
    pub got: String,
    pub run_error: Option<String>,
}

impl GradeResult {
    pub fn points(&self) -> u8 {
        u8::from(self.compile_ok) + u8::from(self.output_ok)
    }
}

pub fn grade(ex: &Exercise, rendu: &Path, build: &Path) -> io::Result<GradeResult> {
    let cc = std::env::var("CC").unwrap_or_else(|_| "cc".to_string());
    let student = rendu.join(format!("{}.c", ex.id));
    let obj = build.join(format!("{}.o", ex.id));

    let compile = Command::new(&cc)
        .args(["-Wall", "-Wextra", "-Werror", "-c"])
        .arg(&student)
        .arg("-o")
        .arg(&obj)
        .output()?;
    let compile_ok = compile.status.success();
    let compiler_stderr = String::from_utf8_lossy(&compile.stderr).into_owned();

    let main_src = build.join(format!("main_{}.c", ex.id));
    std::fs::write(&main_src, ex.test_main)?;
    let bin = build.join(ex.id);
    let link = Command::new(&cc)
        .args(["-Wall", "-Wextra", "-Werror"])
        .arg(&student)
        .arg(&main_src)
        .arg("-o")
        .arg(&bin)
        .output()?;

    let mut output_ok = false;
    let mut got = String::new();
    let mut run_error = None;
    if link.status.success() {
        match Command::new(&bin).output() {
            Ok(out) => {
                got = String::from_utf8_lossy(&out.stdout).into_owned();
                output_ok = outputs_match(ex.expected, &got);
            }
            Err(e) => run_error = Some(e.to_string()),
        }
    } else {
        run_error = Some(String::from_utf8_lossy(&link.stderr).into_owned());
    }

    Ok(GradeResult {
        compile_ok,
        output_ok,
        compiler_stderr,
        expected: ex.expected.to_string(),
        got,
        run_error,
    })
}

pub fn outputs_match(expected: &str, got: &str) -> bool {
    expected == got
}

pub fn best_of(stored: u8, current: u8) -> u8 {
    stored.max(current).min(MAX_SCORE)
}

pub fn parse_scores(text: &str, ids: &[&str]) -> Vec<u8> {
    let mut out = vec![0u8; ids.len()];
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if let Some(i) = ids.iter().position(|id| *id == key.trim())
            && let Ok(n) = value.trim().parse::<u8>()
        {
            out[i] = n.min(MAX_SCORE);
        }
    }
    out
}

pub fn format_scores(ids: &[&str], scores: &[u8]) -> String {
    let mut out = String::new();
    for (id, &score) in ids.iter().zip(scores) {
        out.push_str(&format!("{id}={}\n", score.min(MAX_SCORE)));
    }
    out
}

pub fn load_scores(path: &Path, ids: &[&str]) -> Vec<u8> {
    match std::fs::read_to_string(path) {
        Ok(text) => parse_scores(&text, ids),
        Err(_) => vec![0u8; ids.len()],
    }
}

pub fn save_scores(path: &Path, ids: &[&str], scores: &[u8]) -> io::Result<()> {
    std::fs::write(path, format_scores(ids, scores))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercises::Exercise;

    #[test]
    fn exact_match_including_newline() {
        assert!(outputs_match("42\n", "42\n"));
        assert!(!outputs_match("42\n", "42"));
        assert!(!outputs_match("42\n", "24\n"));
    }

    #[test]
    fn best_score_is_kept() {
        assert_eq!(best_of(2, 0), 2);
        assert_eq!(best_of(1, 2), 2);
        assert_eq!(best_of(0, 1), 1);
        assert_eq!(best_of(9, 9), 2);
    }

    #[test]
    fn score_roundtrip_and_leniency() {
        let ids = ["a", "b", "c"];
        let text = "a=1\nb=2\nunknown=5\nc=nope\n";
        let parsed = parse_scores(text, &ids);
        assert_eq!(parsed, vec![1, 2, 0]);
        assert_eq!(format_scores(&ids, &parsed), "a=1\nb=2\nc=0\n");
        assert_eq!(parse_scores(&format_scores(&ids, &parsed), &ids), parsed);
    }

    fn putchar_exercise() -> Exercise {
        Exercise {
            id: "ft_putchar_test",
            description: "",
            prototype: "void ft_putchar(char c);",
            test_main: "void\tft_putchar(char c);\n\nint\tmain(void)\n{\n\tft_putchar('4');\n\tft_putchar('2');\n\tft_putchar('\\n');\n\treturn (0);\n}\n",
            expected: "42\n",
        }
    }

    #[test]
    fn grades_compile_and_output() {
        let dir = std::env::temp_dir().join(format!("moulinette_test_{}", std::process::id()));
        let rendu = dir.join("rendu");
        let build = rendu.join(".build");
        std::fs::create_dir_all(&build).unwrap();
        let ex = putchar_exercise();
        let student = rendu.join(format!("{}.c", ex.id));

        std::fs::write(
            &student,
            "#include <unistd.h>\nvoid\tft_putchar(char c)\n{\n\twrite(1, &c, 1);\n}\n",
        )
        .unwrap();
        let good = grade(&ex, &rendu, &build).unwrap();
        assert!(good.compile_ok, "stderr: {}", good.compiler_stderr);
        assert!(good.output_ok, "got: {:?}", good.got);
        assert_eq!(good.points(), 2);

        std::fs::write(&student, "void\tft_putchar(char c)\n{\n\t(void)c;\n}\n").unwrap();
        let logic = grade(&ex, &rendu, &build).unwrap();
        assert!(logic.compile_ok);
        assert!(!logic.output_ok);
        assert_eq!(logic.points(), 1);

        std::fs::write(&student, "this is not C\n").unwrap();
        let syntax = grade(&ex, &rendu, &build).unwrap();
        assert!(!syntax.compile_ok);
        assert_eq!(syntax.points(), 0);

        std::fs::remove_dir_all(&dir).ok();
    }

    fn reference_solution(id: &str) -> &'static str {
        match id {
            "ft_putchar" => {
                "#include <unistd.h>\nvoid\tft_putchar(char c)\n{\n\twrite(1, &c, 1);\n}\n"
            }
            "ft_strlen" => {
                "int\tft_strlen(char *str)\n{\n\tint\ti = 0;\n\n\twhile (str[i])\n\t\ti++;\n\treturn (i);\n}\n"
            }
            "ft_swap" => {
                "void\tft_swap(int *a, int *b)\n{\n\tint\ttmp = *a;\n\n\t*a = *b;\n\t*b = tmp;\n}\n"
            }
            "ft_is_negative" => {
                "#include <unistd.h>\nvoid\tft_is_negative(int n)\n{\n\tif (n < 0)\n\t\twrite(1, \"N\", 1);\n\telse\n\t\twrite(1, \"P\", 1);\n}\n"
            }
            "ft_strdup" => {
                "#include <stdlib.h>\nchar\t*ft_strdup(char *src)\n{\n\tchar\t*dup;\n\tint\t\ti;\n\n\ti = 0;\n\twhile (src[i])\n\t\ti++;\n\tdup = malloc(sizeof(char) * (i + 1));\n\tif (!dup)\n\t\treturn (0);\n\ti = 0;\n\twhile (src[i])\n\t{\n\t\tdup[i] = src[i];\n\t\ti++;\n\t}\n\tdup[i] = '\\0';\n\treturn (dup);\n}\n"
            }
            "ft_isdigit" => "int\tft_isdigit(int c)\n{\n\treturn (c >= '0' && c <= '9');\n}\n",
            "ft_strcmp" => {
                "int\tft_strcmp(char *s1, char *s2)\n{\n\tint\ti = 0;\n\n\twhile (s1[i] && s1[i] == s2[i])\n\t\ti++;\n\treturn ((unsigned char)s1[i] - (unsigned char)s2[i]);\n}\n"
            }
            "ft_putstr" => {
                "#include <unistd.h>\nvoid\tft_putstr(char *str)\n{\n\tint\ti;\n\n\ti = 0;\n\twhile (str[i])\n\t{\n\t\twrite(1, &str[i], 1);\n\t\ti++;\n\t}\n}\n"
            }
            "ft_strcpy" => {
                "char\t*ft_strcpy(char *dest, char *src)\n{\n\tint\ti;\n\n\ti = 0;\n\twhile (src[i])\n\t{\n\t\tdest[i] = src[i];\n\t\ti++;\n\t}\n\tdest[i] = '\\0';\n\treturn (dest);\n}\n"
            }
            "ft_isalpha" => {
                "int\tft_isalpha(int c)\n{\n\treturn ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z'));\n}\n"
            }
            "ft_putnbr" => {
                "#include <unistd.h>\nstatic void\tput(long n)\n{\n\tchar\tc;\n\n\tif (n >= 10)\n\t\tput(n / 10);\n\tc = '0' + (char)(n % 10);\n\twrite(1, &c, 1);\n}\n\nvoid\tft_putnbr(int nb)\n{\n\tlong\tn;\n\n\tn = nb;\n\tif (n < 0)\n\t{\n\t\twrite(1, \"-\", 1);\n\t\tn = -n;\n\t}\n\tput(n);\n}\n"
            }
            "ft_bzero" => {
                "void\tft_bzero(void *s, unsigned int n)\n{\n\tunsigned char\t*p;\n\tunsigned int\ti;\n\n\tp = s;\n\ti = 0;\n\twhile (i < n)\n\t{\n\t\tp[i] = 0;\n\t\ti++;\n\t}\n}\n"
            }
            "ft_toupper" => {
                "int\tft_toupper(int c)\n{\n\tif (c >= 'a' && c <= 'z')\n\t\treturn (c - 32);\n\treturn (c);\n}\n"
            }
            "ft_strcat" => {
                "char\t*ft_strcat(char *dest, char *src)\n{\n\tint\ti;\n\tint\tj;\n\n\ti = 0;\n\twhile (dest[i])\n\t\ti++;\n\tj = 0;\n\twhile (src[j])\n\t{\n\t\tdest[i] = src[j];\n\t\ti++;\n\t\tj++;\n\t}\n\tdest[i] = '\\0';\n\treturn (dest);\n}\n"
            }
            "ft_strchr" => {
                "char\t*ft_strchr(char *str, int c)\n{\n\tint\ti;\n\n\ti = 0;\n\twhile (str[i] && str[i] != (char)c)\n\t\ti++;\n\tif (str[i] == (char)c)\n\t\treturn (str + i);\n\treturn (0);\n}\n"
            }
            "ft_atoi" => {
                "int\tft_atoi(char *str)\n{\n\tint\ti;\n\tint\tsign;\n\tint\tn;\n\n\ti = 0;\n\tsign = 1;\n\tn = 0;\n\twhile (str[i] == ' ' || (str[i] >= '\\t' && str[i] <= '\\r'))\n\t\ti++;\n\tif (str[i] == '-' || str[i] == '+')\n\t{\n\t\tif (str[i] == '-')\n\t\t\tsign = -1;\n\t\ti++;\n\t}\n\twhile (str[i] >= '0' && str[i] <= '9')\n\t{\n\t\tn = n * 10 + (str[i] - '0');\n\t\ti++;\n\t}\n\treturn (n * sign);\n}\n"
            }
            "ft_isascii" => "int\tft_isascii(int c)\n{\n\treturn (c >= 0 && c <= 127);\n}\n",
            "ft_strncpy" => {
                "char\t*ft_strncpy(char *dest, char *src, unsigned int n)\n{\n\tunsigned int\ti;\n\n\ti = 0;\n\twhile (i < n && src[i])\n\t{\n\t\tdest[i] = src[i];\n\t\ti++;\n\t}\n\twhile (i < n)\n\t{\n\t\tdest[i] = '\\0';\n\t\ti++;\n\t}\n\treturn (dest);\n}\n"
            }
            "ft_tolower" => {
                "int\tft_tolower(int c)\n{\n\tif (c >= 'A' && c <= 'Z')\n\t\treturn (c + 32);\n\treturn (c);\n}\n"
            }
            "ft_memset" => {
                "void\t*ft_memset(void *s, int c, unsigned int n)\n{\n\tunsigned char\t*p;\n\tunsigned int\ti;\n\n\tp = s;\n\ti = 0;\n\twhile (i < n)\n\t{\n\t\tp[i] = (unsigned char)c;\n\t\ti++;\n\t}\n\treturn (s);\n}\n"
            }
            "ft_strncmp" => {
                "int\tft_strncmp(char *s1, char *s2, unsigned int n)\n{\n\tunsigned int\ti;\n\n\ti = 0;\n\twhile (i < n && s1[i] && s1[i] == s2[i])\n\t\ti++;\n\tif (i == n)\n\t\treturn (0);\n\treturn ((unsigned char)s1[i] - (unsigned char)s2[i]);\n}\n"
            }
            "ft_strrchr" => {
                "char\t*ft_strrchr(char *str, int c)\n{\n\tint\ti;\n\tint\tlast;\n\n\ti = 0;\n\tlast = -1;\n\twhile (str[i])\n\t{\n\t\tif (str[i] == (char)c)\n\t\t\tlast = i;\n\t\ti++;\n\t}\n\tif (c == 0)\n\t\treturn (str + i);\n\tif (last >= 0)\n\t\treturn (str + last);\n\treturn (0);\n}\n"
            }
            "ft_isalnum" => {
                "int\tft_isalnum(int c)\n{\n\treturn ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')\n\t\t|| (c >= '0' && c <= '9'));\n}\n"
            }
            "ft_strncat" => {
                "char\t*ft_strncat(char *dest, char *src, unsigned int n)\n{\n\tunsigned int\ti;\n\tunsigned int\tj;\n\n\ti = 0;\n\twhile (dest[i])\n\t\ti++;\n\tj = 0;\n\twhile (j < n && src[j])\n\t{\n\t\tdest[i + j] = src[j];\n\t\tj++;\n\t}\n\tdest[i + j] = '\\0';\n\treturn (dest);\n}\n"
            }
            "ft_isprint" => "int\tft_isprint(int c)\n{\n\treturn (c >= 32 && c <= 126);\n}\n",
            other => panic!("no reference solution for {other}"),
        }
    }

    #[test]
    fn all_real_exercises_grade_full() {
        use crate::exercises::EXERCISES;
        let dir = std::env::temp_dir().join(format!("moulinette_full_{}", std::process::id()));
        let rendu = dir.join("rendu");
        let build = rendu.join(".build");
        std::fs::create_dir_all(&build).unwrap();

        for ex in &EXERCISES {
            let student = rendu.join(format!("{}.c", ex.id));
            std::fs::write(&student, reference_solution(ex.id)).unwrap();
            let r = grade(ex, &rendu, &build).unwrap();
            assert!(
                r.compile_ok,
                "{} did not compile: {}",
                ex.id, r.compiler_stderr
            );
            assert!(
                r.output_ok,
                "{} expected {:?} got {:?}",
                ex.id, r.expected, r.got
            );
            assert_eq!(r.points(), 2, "{}", ex.id);
        }

        std::fs::remove_dir_all(&dir).ok();
    }
}
