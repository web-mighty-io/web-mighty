#!/usr/bin/env python3

import argparse
from clint.textui import colored
import os
import requests
from typing import List

static_path = ''
remove_files = False


def print_error(msg: str) -> None:
    print(colored.red('error') + ': ' + msg)


def print_info(msg: str) -> None:
    print(colored.blue('info') + ': ' + msg)


def run_fast_scandir(path: str, ext: List[str]) -> (List[str], List[str]):
    """
    From [stack overflow](https://stackoverflow.com/a/59803793/9163028) answer
    Searches all files with extensions below path
    """
    subfolders, files = [], []

    for i in os.scandir(path):
        if i.is_dir():
            subfolders.append(i.path)
        if i.is_file():
            if os.path.splitext(i.name)[1].lower() in ext:
                files.append(i.path)

    for path in list(subfolders):
        sf, i = run_fast_scandir(path, ext)
        subfolders.extend(sf)
        files.extend(i)
    return subfolders, files


def minify_file(path: str, url: str) -> bool:
    data = {'input': open(path, 'rb').read()}
    response = requests.post(url, data=data)
    if response.status_code >= 400:
        print_error('minify {} failed due to bad response to {}'.format(path, url))
        return False

    origin = path
    path = path.split('.')
    extension = path[-1]
    path = path[:-1]
    path.extend(['min', extension])
    path = '.'.join(path)

    f = open(path, 'w')
    f.write(response.text)

    print_info('minified {} to {}'.format(origin, path))

    if remove_files:
        os.remove(origin)
        print_info('removed {}'.format(origin))
    return True


def minify_files() -> None:
    success = True
    _, files = run_fast_scandir(static_path, ['.css'])
    for i in files:
        if 'min' not in i.split('.'):
            success = success and minify_file(i, 'https://cssminifier.com/raw')
    _, files = run_fast_scandir(static_path, ['.js'])
    for i in files:
        if 'min' not in i.split('.'):
            success = success and minify_file(i, 'https://javascript-minifier.com/raw')

    if not success:
        exit(1)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Minify files')
    parser.add_argument('--path', type=str, help='path to static files (defaults to `static`)', default='static')
    parser.add_argument('--remove', type=bool, help='remove original files (defaults to `False`)', default=False, const=True, nargs='?')
    args = parser.parse_args()
    static_path = args.path
    remove_files = args.remove

    minify_files()
