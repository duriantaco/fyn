def main():
    import sys

    # This works both as redirect to use the proper fyn package and as smoke test.
    print(
        "fyn_build contains only the PEP 517 build backend for fyn and can't be used on the CLI. "
        "Use `fyn build` or another build frontend instead.",
        file=sys.stderr,
    )
    if "--help" in sys.argv or "-h" in sys.argv:
        sys.exit(0)
    else:
        sys.exit(1)


if __name__ == "__main__":
    main()
