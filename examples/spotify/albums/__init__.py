import logging
import json

logging.basicConfig(
    format="%(asctime)s %(levelname)-8s %(message)s",
    level=logging.INFO,
    datefmt="%Y-%m-%d %H:%M:%S",
)
logger = logging.getLogger(__name__)


albums = [
    "Thriller - Michael Jackson",
    "Dark Side of the Moon - Pink Floyd",
    "Abbey Road - The Beatles",
    "Back in Black - AC/DC",
    "The Wall - Pink Floyd",
    "Led Zeppelin IV - Led Zeppelin",
    "Rumours - Fleetwood Mac",
    "Hotel California - Eagles",
    "Nevermind - Nirvana",
    "The Joshua Tree - U2",
]


def create(**kwargs):
    logger.info(f"in album create: {kwargs}")


def list(**kwargs):
    logger.info(f"in album list: {kwargs}")
    return json.dumps(albums)


def retrieve(*args, **kwargs):
    id = args[0]
    logger.info(f"in album retrieve: {id}, {kwargs}")
    try:
        albums[id + 1]
    except IndexError:
        return {"detail": "album not found"}


def get_tracks(id: int, **kwargs):
    logger.info(f"in album get tracks: {id}, {kwargs}")


# TODO: maybe allow named classes in same file to behave same as seperate modules
# class User:
#
#
#    def list(**kwargs): ...
#
#
#    def update_add(**kwargs): ...
#
#
#    def update_remove(**kwargs): ...
#
#
#    def get_contains(**kwargs): ...
