import logging

logging.basicConfig(
    format="%(asctime)s %(levelname)-8s %(message)s",
    level=logging.INFO,
    datefmt="%Y-%m-%d %H:%M:%S",
)
logger = logging.getLogger(__name__)


def list(**kwargs):
    logger.info(f"in album list: {kwargs}")


def retrieve(id: int, **kwargs):
    logger.info(f"in album retrieve: {id}, {kwargs}")


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
