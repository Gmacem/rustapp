import pytest
import subprocess
import pathlib
import os

root_dir = pathlib.Path(__file__).parent.resolve()

class Application:
    def __init__(self):
        pass

    def ls(self, path):
        process = subprocess.Popen([
            f'cd {root_dir} &&'
            f'/tmp/debug/rustapp ls {path}',
        ],
            shell=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE)
        process.wait()
        return process.communicate()

    def find(self, path, filename):
        exec_path = os.path.join(root_dir, path)
        process = subprocess.Popen([
            f'cd {exec_path} &&'
            f'/tmp/debug/rustapp find {filename}',
        ],
            shell=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE)
        process.wait()
        return process.communicate()


@pytest.fixture(scope='package', name='main_app')
def _main_app():
    args = ['cargo build --bin rustapp --target-dir /tmp']
    process = subprocess.Popen(args, shell=True, stdout=subprocess.PIPE,  stderr=subprocess.PIPE)
    process.wait()
    return Application()
