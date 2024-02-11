import subprocess
import sys

subprocess.call(['./jack_compiler_rs', sys.argv[1]])