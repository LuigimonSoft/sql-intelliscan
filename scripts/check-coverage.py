#!/usr/bin/env python3
import argparse
import json
import pathlib
import sys


def resolved_paths(paths):
    return {pathlib.Path(path).resolve() for path in paths}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--report", required=True)
    parser.add_argument("--root", required=True)
    parser.add_argument("--label", required=True)
    parser.add_argument("--min", type=float, default=80.0)
    parser.add_argument("--exclude", action="append", default=[])
    args = parser.parse_args()

    report = json.loads(pathlib.Path(args.report).read_text())
    source_root = pathlib.Path(args.root).resolve()
    excluded = resolved_paths(args.exclude)

    coverage_files = []
    for coverage_file in report["data"][0]["files"]:
        filename = pathlib.Path(coverage_file["filename"]).resolve()
        if source_root in filename.parents and filename not in excluded:
            coverage_files.append(coverage_file)

    if not coverage_files:
        print(f"{args.label} coverage files were not found.", file=sys.stderr)
        return 1

    total_count = sum(file["summary"]["lines"]["count"] for file in coverage_files)
    total_covered = sum(file["summary"]["lines"]["covered"] for file in coverage_files)
    coverage = (total_covered / total_count) * 100 if total_count else 0.0

    print(f"{args.label} coverage files:")
    for coverage_file in sorted(coverage_files, key=lambda item: item["filename"]):
        summary = coverage_file["summary"]["lines"]
        print(
            f"- {coverage_file['filename']}: {summary['percent']:.2f}% "
            f"({summary['covered']}/{summary['count']} lines)"
        )

    print(f"{args.label} line coverage: {coverage:.2f}%")

    if coverage < args.min:
        print(
            f"{args.label} coverage is below the required {args.min:.0f}%.",
            file=sys.stderr,
        )
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
