#!/usr/bin/env python3

import os
import subprocess
from typing import List

import requests
from PyInquirer import prompt
from clint.textui import colored
from pyfiglet import Figlet
import json
import shutil

os.chdir(os.path.dirname(os.path.abspath(__file__)))
os.chdir('../')


def check_command(command: str) -> str:
    output = subprocess.run('command -v {}'.format(command), shell=True, capture_output=True)
    if output.returncode == 0:
        return output.stdout.decode('utf-8').strip()
    else:
        return ''


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

    path = path.split('.')
    extension = path[-1]
    path = path[:-1]
    path.extend(['min', extension])
    path = '.'.join(path)

    f = open(path, 'w')
    f.write(response.text)

    print_info('minified to {}'.format(path))
    return True


def install_cargo() -> str:
    print_info('checking if cargo is installed')
    if len(check_command('cargo')) == 0:
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
            return '~/.cargo/bin/cargo'
        else:
            print_error('cargo should be installed')
            exit(1)
    else:
        print_info('cargo is installed')
        return check_command('cargo')


def install_wasm() -> str:
    print_info('checking if wasm-pack is installed')
    if len(check_command('wasm-pack')) == 0:
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
            return '~/.cargo/bin/wasm-pack'
        else:
            print_error('wasm-pack should be installed')
            exit(1)
    else:
        print_info('wasm-pack is installed')
        return check_command('wasm-pack')


def install_sass() -> str:
    print_info('checking if sass is installed')
    if len(check_command('sass')) == 0:
        print_error('sass is not installed')
        install = prompt([{
            'name': 'install',
            'type': 'confirm',
            'message': 'Install sass?',
            'default': True,
        }])

        if install:
            print_info('installing sass')
            # TODO
        else:
            print_error('sass should be installed')
            exit(1)
    else:
        print_info('sass is installed')
        return check_command('sass')


def install_bulma() -> None:
    res = requests.get('https://api.github.com/repos/jgthms/bulma/releases/latest')
    version = json.loads(res.text)['tag_name']
    shutil.rmtree('public/static/bulma', ignore_errors=True)
    url = 'https://github.com/jgthms/bulma/releases/download/{0}/bulma-{0}.zip'.format(version)
    res = requests.get(url)
    with open('bulma.zip', 'wb') as f:
        f.write(res.content)
    os.system('unzip -d public/static/res bulma.zip')
    os.remove('bulma.zip')


def minify_files() -> None:
    # _, files = run_fast_scandir('public/static', ['.html'])
    success = True
    # for i in files:
    #     if 'min' not in i.split('.'):
    #         success = success and minify_file(i, 'https://html-minifier.com/raw')
    _, files = run_fast_scandir('public/static', ['.css'])
    for i in files:
        if 'min' not in i.split('.'):
            success = success and minify_file(i, 'https://cssminifier.com/raw')
    _, files = run_fast_scandir('public/static', ['.js'])
    for i in files:
        if 'min' not in i.split('.'):
            success = success and minify_file(i, 'https://javascript-minifier.com/raw')

    if not success:
        print_error('minify failed')
        go = prompt([{
            'name': 'go',
            'type': 'confirm',
            'message': 'Start server?',
            'default': False,
        }])['go']

        if not go:
            exit(1)


def compile_sass_files(sass_path: str) -> None:
    _, files = run_fast_scandir('public/static/res/scss', ['.scss'])
    success = True
    for i in files:
        output = subprocess.run('{} {} {}'.format(sass_path, i, i.replace('scss', 'css')), shell=True)
        success = success and output.returncode == 0

    if not success:
        exit(1)


def main() -> None:
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
        cargo_path = install_cargo()
        wasm_path = install_wasm()
        sass_path = install_sass()
        install_bulma()
        compile_sass_files(sass_path)

        if not os.path.isfile('public/Cargo.toml') or not os.path.isfile('server/Cargo.toml'):
            print_error('Wrong directory; please run this in root of project')
            exit(1)

        print_info('building wasm')
        os.system('cd public && {} build --target web'.format(wasm_path))
        print_info('building server')
        os.system('cd server && {} install --root build --path .'.format(cargo_path))

        minify_files()

        # TODO: start server


if __name__ == '__main__':
    main()
