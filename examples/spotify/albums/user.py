import logging

logging.basicConfig(
    format="%(asctime)s %(levelname)-8s %(message)s",
    level=logging.INFO,
    datefmt="%Y-%m-%d %H:%M:%S",
)
logger = logging.getLogger(__name__)


# class User:


def list(**kwargs):
    logger.info(f"in user list... {kwargs}")


def update_add(**kwargs): ...


def update_remove(**kwargs): ...


def get_contains(**kwargs): ...
