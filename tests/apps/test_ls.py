import pathlib
import pytest
import os


root_dir = pathlib.Path(__file__).parent.resolve()


@pytest.mark.parametrize('path, expected',
                         [
                             ('./fakes/fs', ['file_1.txt', 'file_2.txt', 'file_3.txt']),
                             ('./fakes/fs/folder_1', ['']),
                             (os.path.join(root_dir, '../fakes/fs'), ['file_1.txt', 'file_2.txt', 'file_3.txt'])
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
                             (os.path.join(root_dir, '../fakes/fs1'))
                         ])
def test_ls_error(
    main_app,
    path,
):
    out, err = main_app.ls(path)
    assert err != None, out
