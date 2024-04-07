import axumapi
import asyncio


async def start():
    await axumapi.start_server()


if __name__ == "__main__":
    asyncio.run(start())
