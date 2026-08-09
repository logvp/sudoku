import argparse
import random
import shlex
import subprocess
import sys
from pathlib import Path
from typing import Optional


def run(cmd: list[str]) -> int:
    ret = subprocess.run(
        cmd,
        check=False,
    ).returncode
    return ret


def get_current_head() -> str:
    try:
        output = subprocess.check_output(
            ["git", "symbolic-ref", "--short", "HEAD"], text=True
        )
    except subprocess.SubprocessError:
        output = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True)
    return output.strip()


def tree_is_dirty() -> bool:
    ret = run(["git", "diff-index", "--quiet", "HEAD", "--"])
    return ret != 0


def rev_exists(rev: str) -> bool:
    ret = run(["git", "rev-parse", "--verify", "--quiet", rev])
    return ret == 0


def checkout(rev: str) -> bool:
    ret = run(["git", "checkout", rev, "--"])
    return ret == 0


def build(release=True, build_args: Optional[str] = None) -> bool:
    cmd = ["cargo", "build"]
    if release:
        cmd.append("--release")
    if build_args:
        cmd.extend(shlex.split(build_args))
    ret = run(cmd)
    return ret == 0


def move_artifact(old: Path, new: str) -> Path:
    return old.replace(new)


def benchmark(
    a: Path,
    b: Path,
    args: Optional[str],
    warmup: Optional[int],
    min_runs: Optional[int],
):
    a_cmd = str(a) if args is None else " ".join((str(a), args))
    b_cmd = str(b) if args is None else " ".join((str(b), args))
    cmd = ["hyperfine", a_cmd, b_cmd]
    if warmup:
        cmd += ["--warmup", f"{warmup}"]
    if min_runs:
        cmd += ["--min-runs", f"{min_runs}"]
    print(cmd)
    run(cmd)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Cargo benchmarking helper using hyperfine"
    )
    parser.add_argument("a", help="git revision A to compare against B")
    parser.add_argument("b", help="git revision B to compare against A")
    parser.add_argument(
        "--args", default=None, help="arguments to pass for the benchmark"
    )
    parser.add_argument(
        "--build-args", default=None, help="arguments to pass to cargo build"
    )
    parser.add_argument(
        "--artifact",
        type=Path,
        default=Path("./target/release/sudoku.exe"),
        help="the binary produced by cargo build to benchmark",
    )
    parser.add_argument("-w", "--warmup", type=int)
    parser.add_argument("-m", "--min-runs", type=int)
    parser.add_argument("--repeat", default=1, type=int)
    args = parser.parse_args()

    if tree_is_dirty():
        print("Tree is dirty!")
        sys.exit(1)

    build_artifact = args.artifact
    rev_a = args.a
    rev_b = args.b
    assert rev_exists(rev_a)
    assert rev_exists(rev_b)

    prior_state = get_current_head()
    print(f"Current head: {prior_state}")

    try:
        print(f"Checking out {rev_a}")
        assert checkout(rev_a)
        assert build(build_args=args.build_args)
        artifact_a = move_artifact(build_artifact, "rev_a.exe")
        print("Built rev_a!")

        print(f"Checking out {rev_b}")
        assert checkout(rev_b)
        assert build(build_args=args.build_args)
        artifact_b = move_artifact(build_artifact, "rev_b.exe")
        print("Built rev_b!")

        for i in range(args.repeat):
            if random.random() > 0.5:
                benchmark(
                    artifact_a,
                    artifact_b,
                    args.args,
                    warmup=args.warmup,
                    min_runs=args.min_runs,
                )
            else:
                benchmark(
                    artifact_b,
                    artifact_a,
                    args.args,
                    warmup=args.warmup,
                    min_runs=args.min_runs,
                )

    finally:
        print(f"Returning to previous HEAD ({prior_state})")
        checkout(prior_state)
