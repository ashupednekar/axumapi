import logging

logging.basicConfig(
    format="%(asctime)s %(levelname)-8s %(message)s",
    level=logging.INFO,
    datefmt="%Y-%m-%d %H:%M:%S",
)
logger = logging.getLogger(__name__)


def list(*args):
    logger.info(f"in album list: {args}")


def retrieve(id: int, *args):
    logger.info(f"in album retrieve: {id}, {args}")


def get_tracks(id: int, *args):
    logger.info(f"in album get tracks: {id}, {args}")


# TODO: maybe allow named classes in same file to behave same as seperate modules
# class User:
#
#
#    def list(*args): ...
#
#
#    def update_add(*args): ...
#
#
#    def update_remove(*args): ...
#
#
#    def get_contains(*args): ...
