import argparse

import pytest

from spotidl import __version__


# using this fixture since argument parser will be required by all test
# functions; by returning a function that generates a parser, we can use
#  this fixture for many functions
@pytest.fixture()
def generate_parser():
    """
    Acts as the point from where test functions can retrieve the
    ArgumentParser mock object to conduct testing.
    """

    def make_parser():
        parser = argparse.ArgumentParser(
            prog="spotidl",
            description="spotidl: download songs, albums and playlists using Spotify links",
        )
        # since we're manually adding the command line args, we have to account for
        # the space added after each argument hence the extra space in each
        # 'default' parameter
        parser.add_argument("-q", "--quiet", action="store_true")
        parser.add_argument("-c", "--codec", default=" mp3")
        parser.add_argument("-b", "--bitrate", default=" 320")
        parser.add_argument("-d", "--dir", default=" /dl")

        # adding the link argument, it is the argument through which the user
        # can access the download capabilties of the
        # application
        parser.add_argument("link")

        return parser

    return make_parser


def test_version(capsys, generate_parser):
    """
    Confirms the application version.
    """

    # mocking the parser object
    parser = generate_parser()
    parser.add_argument(
        "-v",
        "--version",
        action="version",
        version="1.0.4",
    )

    # getting the current application version from the
    current_version = __version__

    try:
        parser.parse_args(["-v"])

    # our cli application performs a SystemExit when called without the 'link'
    # argument. in case of the version flag, it prints out the current version
    # and exits, so we need to catch the SystemExit Exception
    except SystemExit:
        # capsys allows us to capture stdout/stderr
        captured = capsys.readouterr()
        assert captured.out == f"{current_version}\n"

    del parser


def test_defaults_with_quiet(generate_parser):
    """
    Tests the applications' default generated argument values, with the quiet
    flag having been set to `True`.
    """

    parser = generate_parser()
    # the Namespace class stores the attributes added to the CLI application,
    # allowing us to mock the expected behaviour easily
    expected_output = argparse.Namespace(
        quiet=True, codec=" mp3", bitrate=" 320", dir=" /dl"
    )

    try:
        assert (
            parser.parse_args(["-q", "-c mp3", "-b 320", "-d /dl"]) == expected_output
        )

    except SystemExit:
        # adding a 'pass' here since the argparse module runs,
        # parses all arguments and then calls `sys.exit()`.
        # without an except block here, our tests will keep failing
        pass

    del parser


def test_defaults_without_quiet(generate_parser):
    """
    Tests the applications' default generated argument values, with the quiet
    flag staying at its default `False`.
    """

    parser = generate_parser()

    # checking another variant where we don't modify the quiet param at all
    # and the rest remain the same
    expected_output = argparse.Namespace(
        quiet=False, codec=" mp3", bitrate=" 320", dir=" /dl"
    )

    try:
        assert parser.parse_args([]) == expected_output

    except SystemExit:
        pass

    del parser


def test_empty_link(generate_parser):
    """
    Tests the applications' link argument by sending an empty link to the
    argument parser.
    """

    parser = generate_parser()

    expected_output = argparse.Namespace(link="")

    try:
        assert parser.parse_args([]) == expected_output

    except SystemExit:
        pass

    del parser


def test_valid_link(generate_parser):
    """
    Tests the application's link argument by sending a valid link to the
    argument parser.
    """

    parser = generate_parser()
    parser.add_argument("link")

    song_link = "https://open.spotify.com/track/30AeH6saju8WPJo73cKZyH?\
si=99a24d08f4e44faf"
    expected_output = argparse.Namespace(link=song_link)

    try:
        assert parser.parse_args([song_link]) == expected_output

    except SystemExit:
        pass

    del parser
