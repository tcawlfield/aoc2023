#!/usr/bin/python3

import argparse
import requests
import browsercookie  # https://github.com/richardpenman/browsercookie
import os.path
import sys

DEFAULT_YEAR = 2023

parser = argparse.ArgumentParser(description='Get input file from AoC.')
parser.add_argument('-o', '--output', default='inputs/input_{day_n}.txt', help='Output filename')
parser.add_argument('-y', '--year', default=DEFAULT_YEAR, help='Year')
parser.add_argument('day_n', type=str, help='day number, 1 -- 25')
args = parser.parse_args()

# None of these are working for me:
# cj = browsercookie.chrome(cookie_file=[os.path.expanduser('~/.config/google-chrome/Profile 3/Cookies')]) # not chromium or google-chrome-beta
# cj = browsercookie.chrome(cookie_files=[os.path.expanduser('~/.config/google-chrome/Default/Cookies')]) # not chromium or google-chrome-beta
# cj = browsercookie.chrome()
cj = browsercookie.firefox([os.path.expanduser('~/snap/firefox/common/.mozilla/firefox/profiles.ini')])

output = args.output
if '{' in output:
    output = output.format(day_n=args.day_n)
print(f"Getting {args.day_n} -> {output}")
req = requests.get(f'https://adventofcode.com/{args.year}/day/{args.day_n}/input', cookies=cj)
if req.status_code != 200:
    print(req.text)
    sys.exit(1)
with open(output, "w") as fout:
    fout.write(req.text)
