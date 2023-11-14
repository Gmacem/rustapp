import pytest
import os
from . import ROOT_DIR

@pytest.mark.parametrize('path, expected',
                         [
                             ('./fakes/fs', ['file_1.txt', 'file_2.txt', 'file_3.txt']),
                             ('./fakes/fs/folder_1', ['folder_1']),
                             ('./fakes/fs/folder_2', ['file_2.txt']),
                             (os.path.join(ROOT_DIR, './fakes/fs'), ['file_1.txt', 'file_2.txt', 'file_3.txt']),
                         ])
def test_ls_ok(
    main_app,
    path,
    expected,
):
    out, err = main_app.ls(path)
    assert len(err) == 0, err

    files = out.decode().rstrip('\n').split('\n')
    assert files == expected, out


@pytest.mark.parametrize('path',
                         [
                             ('./fakes/fs1'),
                             ('./fakes'),
                             (os.path.join(ROOT_DIR, './fakes/fs1'))
                         ])
def test_ls_error(
    main_app,
    path,
):
    out, err = main_app.ls(path)
    assert err != None, out
