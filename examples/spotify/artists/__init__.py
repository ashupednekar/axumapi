import logging

logging.basicConfig(
    format="%(asctime)s %(levelname)-8s %(message)s",
    level=logging.INFO,
    datefmt="%Y-%m-%d %H:%M:%S",
)
logger = logging.getLogger(__name__)


# class Artists:


def list(**kwargs): ...


def retrieve(id: int, **kwargs): ...


def get_albums(id: int, **kwargs): ...


def get_top_tracks(id: int, **kwargs): ...
