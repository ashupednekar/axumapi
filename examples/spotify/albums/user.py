import logging

logging.basicConfig(
    format="%(asctime)s %(levelname)-8s %(message)s",
    level=logging.INFO,
    datefmt="%Y-%m-%d %H:%M:%S",
)
logger = logging.getLogger(__name__)


# class User:


def list(*args):
    logger.info(f"in user list... {args}")


def update_add(*args): ...


def update_remove(*args): ...


def get_contains(*args): ...
