from . import ROOT_DIR
import pytest
import os

path_to_fakes = os.path.join(ROOT_DIR, 'fakes/fs')


@pytest.mark.parametrize('path, filename, expected',
[
    ('./fakes/fs', 'file_1.txt', 
        [
            os.path.join(path_to_fakes, 'file_1.txt'),
            os.path.join(path_to_fakes, 'folder_1/folder_1_1/file_1.txt'),
            os.path.join(path_to_fakes, 'folder_1/folder_1_2/file_1.txt'),
        ]),
    ('./fakes/fs', 'folder_1', 
        [
            os.path.join(path_to_fakes, 'folder_1/folder_1'),
            os.path.join(path_to_fakes, 'folder_1'),
        ]),
    ('./fakes/fs', 'missing_folder', 
        ['']),
])
def test_find_ok(
    main_app,
    path,
    filename,
    expected,
):
    out, err = main_app.find(path, filename)
    assert len(err) == 0, err

    files = out.decode().rstrip('\n').split('\n')
    assert files == expected, out


def test_find_with_sort(
    main_app
):
    out, err = main_app.find('./fakes/fs', 'file_1.txt', sort=True)
    assert len(err) == 0, err

    files = out.decode().rstrip('\n').split('\n')
    for i in range(0, len(files) - 1):
        assert files[i] < files[i+1]


def test_find_in_file(
    main_app,
):
    out, err = main_app.find('./fakes/fs', 'file_1.txt', sort=True, in_file='some_text_2')
    assert len(err) == 0, err
    files = out.decode().rstrip('\n').split('\n')
    assert len(files) == 2
    assert files[0] == os.path.join(path_to_fakes, 'file_1.txt')
    assert files[1] == os.path.join(path_to_fakes, 'folder_1/folder_1_1/file_1.txt')
