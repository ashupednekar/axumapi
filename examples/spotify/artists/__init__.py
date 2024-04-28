import logging

logging.basicConfig(
    format="%(asctime)s %(levelname)-8s %(message)s",
    level=logging.INFO,
    datefmt="%Y-%m-%d %H:%M:%S",
)
logger = logging.getLogger(__name__)


# class Artists:


def list(*args): ...


def retrieve(id: int, *args): ...


def get_albums(id: int, *args): ...


def get_top_tracks(id: int, *args): ...
