#!/usr/bin/env python3

from clint.textui import colored
import json
import os
from pyfiglet import Figlet
from PyInquirer import prompt
import requests
import shutil
import subprocess
from typing import List

os.chdir(os.path.dirname(os.path.abspath(__file__)))
os.chdir('../')


def shell_output(command: str) -> str:
    output = subprocess.run(command, shell=True, capture_output=True)
    return output.stdout.decode('utf-8')


def shell_output_code(command: str) -> int:
    output = subprocess.run(command, shell=True, capture_output=True)
    return output.returncode


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
            brew_path = check_command('brew')
            if len(brew_path) == 0:
                npm_path = check_command('npm')
                if len(npm_path) == 0:
                    print_error('Please install brew or npm')
                    exit(1)
                os.system('{} i -g sass'.format(npm_path))
                return shell_output('npm bin -g').strip() + '/sass'
            else:
                os.system('{} install sass/sass/sass'.format(brew_path))
                return shell_output('brew --prefix') + '/bin/sass'
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


def compile_sass_files(sass_path: str) -> None:
    _, files = run_fast_scandir('static/res/scss', ['.scss'])
    success = True
    for i in files:
        output = subprocess.run('{} {} {}'.format(sass_path, i, i.replace('scss', 'css')), shell=True)
        success = success and output.returncode == 0

    if not success:
        exit(1)


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


def minify_files() -> None:
    success = True
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


def init_postgres() -> None:
    print_info('checking if postgresql is installed')
    if len(check_command('pg_isready')) == 0:
        print_error('postgresql is not installed')
        print_info('install postgresql: https://www.postgresql.org/download/')
        exit(1)

    if shell_output_code('pg_isready -q') != 0:
        print_error('postgresql is not running')
        print_info('run postgresql')
        exit(1)

    init = prompt([{
        'name': 'init',
        'type': 'confirm',
        'message': 'Init database?',
        'default': True,
    }])

    if init:
        # TODO: make init command
        pass


def main() -> None:
    _, width = os.popen('stty size', 'r').read().split()
    f = Figlet(font='slant', width=int(width))
    print(colored.yellow(f.renderText('Web Mighty')))
    print('Mighty Card Game Web Server: version 1.0.0-dev\n\n')

    cargo_path = install_cargo()
    wasm_path = install_wasm()
    sass_path = install_sass()
    install_bulma()
    compile_sass_files(sass_path)

    if not os.path.isfile('public/Cargo.toml') or not os.path.isfile('server/Cargo.toml'):
        print_error('Wrong directory; please run this in root of project')
        exit(1)

    print_info('building wasm')
    os.system('cd public && {} build --target web -d ../static/res/pkg'.format(wasm_path))

    https = prompt([{
        'name': 'https',
        'type': 'confirm',
        'message': 'Enable https?',
        'default': True,
    }])
    https_str = ' --features https' if https else ''

    file_watch = prompt([{
        'name': 'file_watch',
        'type': 'confirm',
        'message': 'Enable file watcher?',
        'default': False,
    }])
    file_watch_str = ' --features watch-file' if file_watch else ''

    print_info('building server')
    os.system('cd server && {} install --root build --path .{}{}'.format(cargo_path, https_str, file_watch_str))
    minify_files()
    init_postgres()

    os.system('cd server && nohup ./build/bin/server > /dev/null&')


if __name__ == '__main__':
    main()
