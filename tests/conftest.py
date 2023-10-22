import pytest
import subprocess
import pathlib

root_dir = pathlib.Path(__file__).parent.resolve()

class Application:
    def __init__(self):
        pass

    def ls(self, path):
        process = subprocess.Popen([
            f'cd {root_dir} &&'
            f'/tmp/debug/rustapp ls {path}',
            path,
        ],
            shell=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE)
        return process.communicate()


@pytest.fixture(scope='session', name='main_app')
def _main_app():
    args = ['cd ..', '&&', 'cargo build --bin rustapp --target-dir /tmp']
    subprocess.Popen(args, shell=True)
    return Application()
