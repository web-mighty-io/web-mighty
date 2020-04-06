#!/usr/bin/env python3

import os
import subprocess
from typing import List

import requests
from PyInquirer import prompt
from clint.textui import colored
from pyfiglet import Figlet


def check_command(command: str) -> bool:
    child = subprocess.Popen(['command', '-v', command], stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True)
    child.communicate()
    return child.returncode == 0


def print_error(msg: str):
    print(colored.red('error') + ': ' + msg)


def print_info(msg: str):
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


def minify_file(path: str, url: str):
    data = {'input': open(path, 'rb').read()}
    response = requests.post(url, data=data)
    path = path.split('.')
    extension = path[-1]
    path = path[:-1]
    path.extend(['min', extension])
    path = '.'.join(path)
    f = open(path, 'w')
    f.write(response.text)

    print_info('minified to {}'.format(path))


def main():
    _, width = os.popen('stty size', 'r').read().split()
    f = Figlet(font='slant', width=int(width))
    print(colored.yellow(f.renderText('     Mighty')))
    print(colored.yellow(f.renderText('Card Game')))
    print('Mighty Card Game Web Server: version 1.0.0-dev\n\n')

    platform = prompt([{
        'name': 'platform',
        'type': 'list',
        'message': 'Select platform to run',
        'choices': [
            'docker',
            'native',
        ],
    }])['platform']

    if platform == 'docker':
        # TODO: add docker configuration
        pass
    else:
        cargo_path = ''
        print_info('checking if cargo is installed')
        if not check_command('cargo'):
            print_error('rust not installed')
            install = prompt([{
                'name': 'install',
                'type': 'confirm',
                'message': 'Install rust?',
                'default': True,
            }])['install']

            if install:
                print_info('installing rust')
                os.system("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
                cargo_path = '~/.cargo/bin/cargo'
            else:
                print_error('cargo should be installed')
                exit(1)
        else:
            print_info('cargo is installed')
            cargo_path = 'cargo'

        wasm_path = ''
        print_info('checking if wasm-pack is installed')
        if not check_command('wasm-pack'):
            print_error('wasm-pack not installed')
            install = prompt([{
                'name': 'install',
                'type': 'confirm',
                'message': 'Install wasm-pack?',
                'default': True,
            }])['install']

            if install:
                print_info('installing wasm-pack')
                os.system('curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh')
                wasm_path = '~/.cargo/bin/wasm-pack'
            else:
                print_error('wasm-pack should be installed')
                exit(1)
        else:
            print_info('wasm-pack is installed')
            wasm_path = 'wasm-pack'

        if not os.path.isfile('public/Cargo.toml') or not os.path.isfile('server/Cargo.toml'):
            print_error('Wrong directory; please run this in root of project')
            exit(1)

        print_info('building wasm')
        os.system('cd public && {} build --target web'.format(wasm_path))
        print_info('building server')
        os.system('cd server && {} install --root build --path .'.format(cargo_path))

        _, files = run_fast_scandir('public/static', ['.html'])
        for i in files:
            if 'min' not in i.split('.'):
                minify_file(i, 'https://html-minifier.com/raw')
        _, files = run_fast_scandir('public/static', ['.css'])
        for i in files:
            if 'min' not in i.split('.'):
                minify_file(i, 'https://cssminifier.com/raw')
        _, files = run_fast_scandir('public/static', ['.js'])
        for i in files:
            if 'min' not in i.split('.'):
                minify_file(i, 'https://javascript-minifier.com/raw')


if __name__ == '__main__':
    main()
