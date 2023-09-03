import argparse
import logging
import sys

from assembler import Assembler

logger = logging.getLogger("assemble")


def setup_logging():
    logger.setLevel(logging.DEBUG)
    ch = logging.StreamHandler()
    ch.setLevel(logging.DEBUG)
    formatter = logging.Formatter(
        "%(asctime)s - %(name)s - %(levelname)s - %(message)s"
    )
    ch.setFormatter(formatter)
    logger.addHandler(ch)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--infile", type=argparse.FileType("r"), required=True)
    parser.add_argument("-o", "--outfile", type=argparse.FileType("w"), required=True)
    return parser.parse_args()


def main():
    setup_logging()
    args = parse_args()
    assembler = Assembler(args.infile, args.outfile)
    logger.info(
        f"Assembling `{assembler.infile.name}` and writing output to `{assembler.outfile.name}`..."
    )
    try:
        assembler.assemble()
    except Exception as e:
        logger.exception(f"Assembly failed: {e}")
        return 1

    logger.info(f"Assembly successful. Output written to `{assembler.outfile.name}`.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
